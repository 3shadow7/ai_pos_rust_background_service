use async_trait::async_trait;
use crate::hardware::traits::Display;
use crate::errors::ServiceError;
use tokio_serial::SerialPortBuilderExt;
use tokio::io::AsyncWriteExt;
use tracing::info;

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

pub struct SerialDisplay {
    id: String,
    port_name: String,
    baud_rate: u32,
}

impl SerialDisplay {
    pub fn new(id: String, port_name: String, baud_rate: u32) -> Self {
        Self { id, port_name, baud_rate }
    }

    async fn send_command(&self, data: &[u8]) -> Result<(), ServiceError> {
         let mut port = tokio_serial::new(&self.port_name, self.baud_rate)
            .open_native_async()
            .map_err(|e| ServiceError::IoError(format!("Failed to open display port {}: {}", self.port_name, e)))?;

        port.write_all(data).await
            .map_err(|e| ServiceError::IoError(format!("Failed to write to display: {}", e)))?;
        Ok(())
    }
}

#[async_trait]
impl Display for SerialDisplay {
    async fn show_text(&self, line1: &str, line2: &str) -> Result<(), ServiceError> {
        info!("[SerialDisplay {}] Showing text on {}", self.id, self.port_name);
        
        // Basic ESC/POS or CD5220 commands
        // 1. Clear screen usually (CLR = 0x0C)
        let mut data = Vec::new();
        data.push(0x0C); 
        
        // 2. Line 1
        data.extend_from_slice(line1.as_bytes());
        
        // 3. Move to Line 2 (CR = 0x0D, LF = 0x0A) or specific Pos command usually 0x0D 0x0A
        // Many logic displays just wrap, but let's send newline
        data.push(0x0D);
        data.push(0x0A);
        
        // 4. Line 2
        data.extend_from_slice(line2.as_bytes());

        self.send_command(&data).await
    }

    async fn clear(&self) -> Result<(), ServiceError> {
        info!("[SerialDisplay {}] Clearing", self.id);
        self.send_command(&[0x0C]).await
    }
}
