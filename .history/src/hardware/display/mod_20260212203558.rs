pub mod serial;

use async_trait::async_trait;
use crate::hardware::traits::Display;
use crate::errors::ServiceError;
use tracing::info;

pub struct MockDisplay {
    id: String,
}

impl MockDisplay {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[async_trait]
impl Display for MockDisplay {
    async fn show_text(&self, line1: &str, line2: &str) -> Result<(), ServiceError> {
        info!("[Display {}] Line1: '{}', Line2: '{}'", self.id, line1, line2);
        Ok(())
    }

    async fn clear(&self) -> Result<(), ServiceError> {
        info!("[Display {}] Clearing", self.id);
        Ok(())
    }
}
