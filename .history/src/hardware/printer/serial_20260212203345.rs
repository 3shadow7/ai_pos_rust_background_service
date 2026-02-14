use async_trait::async_trait;
use crate::hardware::traits::Printer;
use crate::errors::ServiceError;
use tokio_serial::SerialPortBuilderExt;
use tokio::io::AsyncWriteExt;
use tracing::info;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

pub struct SerialPrinter {
    id: String,
    port_name: String,
    baud_rate: u32,
}

impl SerialPrinter {
    pub fn new(id: String, port_name: String, baud_rate: u32) -> Self {
        Self { id, port_name, baud_rate }
    }

    async fn send_data(&self, data: &[u8]) -> Result<(), ServiceError> {
        let mut port = tokio_serial::new(&self.port_name, self.baud_rate)
            .open_native_async()
            .map_err(|e| ServiceError::IoError(format!("Failed to open serial port {}: {}", self.port_name, e)))?;

        // On Windows specifically, and some serial devices, setting DTR/RTS is sometimes needed, 
        // effectively resetting the line or asserting ready.
        // For basic ESC/POS, defaults are often fine, but we'll stick to defaults unless issues arise.
        
        port.write_all(data).await
            .map_err(|e| ServiceError::IoError(format!("Failed to write to serial printer: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl Printer for SerialPrinter {
    async fn print_text(&self, text: &str) -> Result<(), ServiceError> {
        info!("[SerialPrinter {}] Printing text to {}", self.id, self.port_name);
        
        let mut data = Vec::new();
        data.extend_from_slice(b"\x1B\x40"); // ESC @ (Init)
        data.extend_from_slice(text.as_bytes());
        data.extend_from_slice(b"\n");
        
        self.send_data(&data).await
    }

    async fn cut_paper(&self) -> Result<(), ServiceError> {
        info!("[SerialPrinter {}] Cutting paper", self.id);
        let data = b"\x1D\x56\x42\x00"; 
        self.send_data(data).await
    }

    async fn print_raw(&self, data: &[u8]) -> Result<(), ServiceError> {
        info!("[SerialPrinter {}] Sending raw data", self.id);
        self.send_data(data).await
    }
}
