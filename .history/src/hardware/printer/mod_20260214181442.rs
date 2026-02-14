pub mod network;
pub mod serial;
pub mod windows;

use async_trait::async_trait;
use crate::hardware::traits::Printer;
use crate::errors::ServiceError;
use tracing::info;

pub struct MockPrinter {
    id: String,
}

impl MockPrinter {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[async_trait]
impl Printer for MockPrinter {
    async fn print_text(&self, text: &str) -> Result<(), ServiceError> {
        info!("[Printer {}] Printing: {}", self.id, text);
        Ok(())
    }

    async fn cut_paper(&self) -> Result<(), ServiceError> {
        info!("[Printer {}] Cutting paper", self.id);
        Ok(())
    }

    async fn print_raw(&self, data: &[u8]) -> Result<(), ServiceError> {
        info!("[Printer {}] Raw data: {:?}", self.id, data);
        Ok(())
    }
}
