use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::hardware::traits::{Printer, Drawer, Display};
use crate::hardware::printer::MockPrinter;
use crate::hardware::drawer::MockDrawer;
use crate::hardware::display::MockDisplay;
use crate::config::DevicesConfig;
use crate::errors::ServiceError;

pub struct DeviceManager {
    printers: RwLock<HashMap<String, Arc<dyn Printer>>>,
    drawers: RwLock<HashMap<String, Arc<dyn Drawer>>>,
    displays: RwLock<HashMap<String, Arc<dyn Display>>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            printers: RwLock::new(HashMap::new()),
            drawers: RwLock::new(HashMap::new()),
            displays: RwLock::new(HashMap::new()),
        }
    }

    pub async fn load_from_config(&self, config: &DevicesConfig) -> Result<(), ServiceError> {
        // Load Printers
        {
            let mut printers = self.printers.write().await;
            for p_conf in &config.printers {
                let printer: Arc<dyn Printer> = match p_conf.device_type.as_str() {
                    "mock" => Arc::new(MockPrinter::new(p_conf.id.clone())),
                    _ => {
                        tracing::warn!("Unknown printer type '{}', using Mock", p_conf.device_type);
                        Arc::new(MockPrinter::new(p_conf.id.clone()))
                    }
                };
                printers.insert(p_conf.id.clone(), printer);
            }
        }

        // Load Drawers
        {
            let mut drawers = self.drawers.write().await;
            for d_conf in &config.drawers {
                let drawer: Arc<dyn Drawer> = match d_conf.device_type.as_str() {
                   "mock" => Arc::new(MockDrawer::new(d_conf.id.clone())),
                    _ => Arc::new(MockDrawer::new(d_conf.id.clone())),
                };
                drawers.insert(d_conf.id.clone(), drawer);
            }
        }

        // Load Displays
        {
            let mut displays = self.displays.write().await;
            for d_conf in &config.displays {
                let display: Arc<dyn Display> = match d_conf.device_type.as_str() {
                   "mock" => Arc::new(MockDisplay::new(d_conf.id.clone())),
                    _ => Arc::new(MockDisplay::new(d_conf.id.clone())),
                };
                displays.insert(d_conf.id.clone(), display);
            }
        }

        Ok(())
    }

    pub async fn get_printer(&self, id: &str) -> Option<Arc<dyn Printer>> {
        let printers = self.printers.read().await;
        printers.get(id).cloned()
    }

    pub async fn get_drawer(&self, id: &str) -> Option<Arc<dyn Drawer>> {
        let drawers = self.drawers.read().await;
        drawers.get(id).cloned()
    }

    pub async fn get_display(&self, id: &str) -> Option<Arc<dyn Display>> {
        let displays = self.displays.read().await;
        displays.get(id).cloned()
    }
}
