use async_trait::async_trait;
use crate::errors::ServiceError;
use crate::hardware::traits::Printer;
use std::ffi::c_void;
use tracing::{info, error};

#[cfg(windows)]
use windows::Win32::Graphics::Printing::{
    OpenPrinterW, StartDocPrinterW, StartPagePrinter, WritePrinter, EndPagePrinter, EndDocPrinter, ClosePrinter,
    DOC_INFO_1W,
};
#[cfg(windows)]
use windows::Win32::Foundation::HANDLE;
#[cfg(windows)]
use windows::core::{PCWSTR, PWSTR};

pub struct WindowsPrinter {
    printer_name: String,
}

impl WindowsPrinter {
    pub fn new(printer_name: String) -> Self {
        Self { printer_name }
    }

    #[cfg(windows)]
    fn send_raw_to_printer(&self, data: &[u8]) -> Result<(), ServiceError> {
        info!("Sending {} bytes to Windows printer: {}", data.len(), self.printer_name);
        unsafe {
            let mut h_printer = HANDLE::default();
            
            // Encode printer name to wide string (UTF-16) for Windows API
            let printer_name_wide: Vec<u16> = self.printer_name.encode_utf16().chain(std::iter::once(0)).collect();
            let pc_printer_name = PCWSTR(printer_name_wide.as_ptr());

            // 1. Open Printer
            if let Err(e) = OpenPrinterW(pc_printer_name, &mut h_printer, None) {
                return Err(ServiceError::DeviceError(format!("Failed to open Windows printer '{}': {}", self.printer_name, e)));
            }
            
            // Guard to ensure we ClosePrinter even if we crash/return early below
            let _guard = PrinterHandleGuard(h_printer);

            // 2. Start Document
            let mut doc_name: Vec<u16> = "POS Service Receipt".encode_utf16().chain(std::iter::once(0)).collect();
            let mut data_type: Vec<u16> = "RAW".encode_utf16().chain(std::iter::once(0)).collect();
            
            let doc_info = DOC_INFO_1W {
                pDocName: PWSTR(doc_name.as_mut_ptr()),
                pOutputFile: PWSTR::null(),
                pDatatype: PWSTR(data_type.as_mut_ptr()),
            };

            // StartDocPrinterW returns u32 (job id)
            let job_id = StartDocPrinterW(h_printer, 1, &doc_info);
            if job_id == 0 {
                let err = std::io::Error::last_os_error();
                return Err(ServiceError::DeviceError(format!("Failed to start print document: {}", err)));
            }

            // 3. Start Page - Returns BOOL
            if !StartPagePrinter(h_printer).as_bool() {
                let err = std::io::Error::last_os_error();
                let _ = EndDocPrinter(h_printer);
                return Err(ServiceError::DeviceError(format!("Failed to start print page: {}", err)));
            }

            // 4. Write Data - Returns BOOL
            let mut bytes_written = 0;
            let success = WritePrinter(
                h_printer, 
                data.as_ptr() as *const c_void, 
                data.len() as u32, 
                &mut bytes_written
            ).as_bool();

            if !success {
                 let err = std::io::Error::last_os_error();
                 error!("WritePrinter failed: {}", err);
            }

            // 5. Cleanup
            let _ = EndPagePrinter(h_printer);
            let _ = EndDocPrinter(h_printer);

            if !success {
                 return Err(ServiceError::DeviceError("Failed to write bytes to printer".to_string()));
            }

        }
        
        Ok(())
    }

    #[cfg(not(windows))]
    fn send_raw_to_printer(&self, _data: &[u8]) -> Result<(), ServiceError> {
        Err(ServiceError::ConfigError("Windows printing not supported on this OS".to_string()))
    }
}

// Helper to automatically close printer handle when it goes out of scope
#[cfg(windows)]
struct PrinterHandleGuard(HANDLE);

#[cfg(windows)]
impl Drop for PrinterHandleGuard {
    fn drop(&mut self) {
        unsafe { let _ = ClosePrinter(self.0); }
    }
}

#[async_trait]
impl Printer for WindowsPrinter {
    async fn print_text(&self, text: &str) -> Result<(), ServiceError> {
        self.print_raw(text.as_bytes()).await
    }

    async fn cut_paper(&self) -> Result<(), ServiceError> {
         // Some printers need a paper feed before cutting so the text clears the cutter blade.
         // We send:
         // 1. ESC d 3 (Feed 3 lines) -> [0x1B, 0x64, 0x03]
         // 2. GS V 66 0 (Feed & Cut) -> [0x1D, 0x56, 0x42, 0x00]
         // 3. Fallback GS V 0 (Simple Cut) -> [0x1D, 0x56, 0x00]
         
         let cut_sequence = [
             0x1B, 0x64, 0x03,       // Feed 3 lines
             0x1D, 0x56, 0x42, 0x00, // Primary Cut Command
         ];
         
         self.print_raw(&cut_sequence).await
    }

    async fn print_raw(&self, data: &[u8]) -> Result<(), ServiceError> {
        // Windows API calls are blocking, so we run them in a separate thread
        // to prevent stopping the whole server while printing.
        let data = data.to_vec();
        let name = self.printer_name.clone();

        let res = tokio::task::spawn_blocking(move || {
            let printer = WindowsPrinter::new(name);
            printer.send_raw_to_printer(&data)
        }).await;
        
        match res {
            Ok(inner) => inner,
            Err(e) => Err(ServiceError::DeviceError(format!("Thread error: {}", e))),
        }
    }
}
