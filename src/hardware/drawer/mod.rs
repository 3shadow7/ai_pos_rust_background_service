pub mod printer_drawer;

use async_trait::async_trait;
use crate::hardware::traits::Drawer;
use crate::errors::ServiceError;
use tracing::info;

pub struct MockDrawer {
    id: String,
}

impl MockDrawer {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[async_trait]
impl Drawer for MockDrawer {
    async fn open(&self) -> Result<(), ServiceError> {
        info!("[Drawer {}] OPENING", self.id);
        Ok(())
    }
}
