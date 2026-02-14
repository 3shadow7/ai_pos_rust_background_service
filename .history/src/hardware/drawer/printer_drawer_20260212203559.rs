use async_trait::async_trait;
use crate::hardware::traits::Drawer;
use crate::hardware::traits::Printer;
use crate::errors::ServiceError;
use std::sync::Arc;
use tracing::info;

pub struct PrinterDrivenDrawer {
    id: String,
    printer: Arc<dyn Printer>,
}

impl PrinterDrivenDrawer {
    pub fn new(id: String, printer: Arc<dyn Printer>) -> Self {
        Self { id, printer }
    }
}

#[async_trait]
impl Drawer for PrinterDrivenDrawer {
    async fn open(&self) -> Result<(), ServiceError> {
        info!("[PrinterDrawer {}] Sending OPEN pulse via printer", self.id);
        // Standard ESC/POS drawer kick command
        // ESC p m t1 t2
        // m=0 (pin 2), t1=50ms, t2=50ms
        // Decimal: 27 112 0 25 250
        let kick_command = b"\x1B\x70\x00\x19\xFA";
        self.printer.print_raw(kick_command).await
    }
}
