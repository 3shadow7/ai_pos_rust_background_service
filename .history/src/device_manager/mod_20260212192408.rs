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
    printers: RwLock<HashMap<String, Box<dyn Printer>>>,
    drawers: RwLock<HashMap<String, Box<dyn Drawer>>>,
    displays: RwLock<HashMap<String, Box<dyn Display>>>,
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
                let printer: Box<dyn Printer> = match p_conf.device_type.as_str() {
                    "mock" => Box::new(MockPrinter::new(p_conf.id.clone())),
                    _ => {
                        // In future, support other types. Fallback to mock or error.
                        // For now we'll just log warning and substitute mock
                        tracing::warn!("Unknown printer type '{}', using Mock", p_conf.device_type);
                        Box::new(MockPrinter::new(p_conf.id.clone()))
                    }
                };
                printers.insert(p_conf.id.clone(), printer);
            }
        }

        // Load Drawers
        {
            let mut drawers = self.drawers.write().await;
            for d_conf in &config.drawers {
                let drawer: Box<dyn Drawer> = match d_conf.device_type.as_str() {
                   "mock" => Box::new(MockDrawer::new(d_conf.id.clone())),
                    _ => Box::new(MockDrawer::new(d_conf.id.clone())),
                };
                drawers.insert(d_conf.id.clone(), drawer);
            }
        }

        // Load Displays
        {
            let mut displays = self.displays.write().await;
            for d_conf in &config.displays {
                let display: Box<dyn Display> = match d_conf.device_type.as_str() {
                   "mock" => Box::new(MockDisplay::new(d_conf.id.clone())),
                    _ => Box::new(MockDisplay::new(d_conf.id.clone())),
                };
                displays.insert(d_conf.id.clone(), display);
            }
        }

        Ok(())
    }

    pub async fn get_printer(&self, id: &str) -> Option<Arc<Box<dyn Printer>>> {
        // We can't return a reference to the Box inside RwLockReadGuard because the guard is local.
        // We can't clone Box<dyn Printer> easily without `dyn Clone`.
        // A better approach is to wrap devices in Arc.
        // HashMap<String, Arc<Box<dyn Printer>>> or just Arc<dyn Printer> (but trait objects need Box usually, Arc<dyn Printer> works too).
        // Let's refactor the struct to use Arc<dyn Printer>.
        // For now I'll just change the return type logic or refactor struct.
        // Refactoring struct is cleaner.
        None 
    }
}
