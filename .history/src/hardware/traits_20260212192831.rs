use async_trait::async_trait;
use crate::errors::ServiceError;

#[async_trait]
pub trait Printer: Send + Sync {
    async fn print_text(&self, text: &str) -> Result<(), ServiceError>;
    async fn cut_paper(&self) -> Result<(), ServiceError>;
    // Add raw for bytes ESC/POS
    async fn print_raw(&self, data: &[u8]) -> Result<(), ServiceError>;
}

#[async_trait]
pub trait Drawer: Send + Sync {
    async fn open(&self) -> Result<(), ServiceError>;
}

#[async_trait]
pub trait Display: Send + Sync {
    async fn show_text(&self, line1: &str, line2: &str) -> Result<(), ServiceError>;
    async fn clear(&self) -> Result<(), ServiceError>;
}
