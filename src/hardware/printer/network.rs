use async_trait::async_trait;
use crate::hardware::traits::Printer;
use crate::errors::ServiceError;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tracing::info;

pub struct NetworkPrinter {
    id: String,
    address: String,
}

impl NetworkPrinter {
    pub fn new(id: String, address: String) -> Self {
        Self { id, address }
    }

    // This helper function opens a real network connection to the printer
    // and sends the raw bytes (commands) to it.
    async fn send_data(&self, data: &[u8]) -> Result<(), ServiceError> {
        // 1. Connect to the printer IP address (e.g., 192.168.1.200:9100)
        let mut stream = TcpStream::connect(&self.address).await
            .map_err(|e| ServiceError::IoError(format!("Failed to connect to printer at {}: {}", self.address, e)))?;
        
        // 2. Send the actual data bytes
        stream.write_all(data).await
            .map_err(|e| ServiceError::IoError(format!("Failed to write to printer: {}", e)))?;
            
        Ok(())
    }
}

#[async_trait]
impl Printer for NetworkPrinter {
    // This function formats simple text and sends it to the printer.
    async fn print_text(&self, text: &str) -> Result<(), ServiceError> {
        info!("[NetworkPrinter {}] Printing text to {}", self.id, self.address);
        // Simple text print command usually involves initializing printer then sending text
        // ESC @ = Initialize Printer Command
        // Text = The raw text content
        // LF = Line Feed (Input key essentially) to make it print
        let mut data = Vec::new();
        data.extend_from_slice(b"\x1B\x40"); // ESC @ (Init)
        data.extend_from_slice(text.as_bytes());
        data.extend_from_slice(b"\n");       // Newline
        
        self.send_data(&data).await
    }

    // This sends the specific command code to cut the paper.
    async fn cut_paper(&self) -> Result<(), ServiceError> {
        info!("[NetworkPrinter {}] Cutting paper", self.id);
        // GS V 66 0 is a standard ESC/POS command for "Cut Paper"
        let data = b"\x1D\x56\x42\x00"; 
        self.send_data(data).await
    }

    // This allows the POS to send raw hexadecimal commands directly
    // (useful for formatted receipts with bold, center, logos, etc.)
    async fn print_raw(&self, data: &[u8]) -> Result<(), ServiceError> {
        info!("[NetworkPrinter {}] Sending raw data", self.id);
        self.send_data(data).await
    }
}
