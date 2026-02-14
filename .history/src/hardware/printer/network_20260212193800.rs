use async_trait::async_trait;
use crate::hardware::traits::Printer;
use crate::errors::ServiceError;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tracing::{info, error};

pub struct NetworkPrinter {
    id: String,
    address: String,
}

impl NetworkPrinter {
    pub fn new(id: String, address: String) -> Self {
        Self { id, address }
    }

    async fn send_data(&self, data: &[u8]) -> Result<(), ServiceError> {
        let mut stream = TcpStream::connect(&self.address).await
            .map_err(|e| ServiceError::IoError(format!("Failed to connect to printer at {}: {}", self.address, e)))?;
        
        stream.write_all(data).await
            .map_err(|e| ServiceError::IoError(format!("Failed to write to printer: {}", e)))?;
            
        Ok(())
    }
}

#[async_trait]
impl Printer for NetworkPrinter {
    async fn print_text(&self, text: &str) -> Result<(), ServiceError> {
        info!("[NetworkPrinter {}] Printing text to {}", self.id, self.address);
        // Simple text print command usually involves initializing printer then sending text
        // ESC @ = Initialize
        // Text
        // LF = Line Feed
        let mut data = Vec::new();
        data.extend_from_slice(b"\x1B\x40"); // ESC @ (Init)
        data.extend_from_slice(text.as_bytes());
        data.extend_from_slice(b"\n");
        
        self.send_data(&data).await
    }

    async fn cut_paper(&self) -> Result<(), ServiceError> {
        info!("[NetworkPrinter {}] Cutting paper", self.id);
        // GS V 66 0 = Cut paper
        let data = b"\x1D\x56\x42\x00"; 
        self.send_data(data).await
    }

    async fn print_raw(&self, data: &[u8]) -> Result<(), ServiceError> {
        info!("[NetworkPrinter {}] Sending raw data", self.id);
        self.send_data(data).await
    }
}
