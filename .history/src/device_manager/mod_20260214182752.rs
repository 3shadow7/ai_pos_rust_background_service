use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::hardware::traits::{Printer, Drawer, Display};
use crate::hardware::printer::{MockPrinter, network::NetworkPrinter, serial::SerialPrinter, windows::WindowsPrinter};
use crate::hardware::drawer::{MockDrawer, printer_drawer::PrinterDrivenDrawer};
use crate::hardware::display::{MockDisplay, serial::SerialDisplay};
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
                    "network" | "esc_pos_network" => Arc::new(NetworkPrinter::new(p_conf.id.clone(), p_conf.connection.clone())),
                    "serial" => {
                        // Support "COM1:9600" format or default to 9600
                        let parts: Vec<&str> = p_conf.connection.split(':').collect();
                        let (port, baud) = if parts.len() == 2 {
                            let baud = parts[1].parse::<u32>().unwrap_or(9600);
                            (parts[0], baud)
                        } else {
                            (p_conf.connection.as_str(), 9600)
                        };
                        
                        Arc::new(SerialPrinter::new(p_conf.id.clone(), port.to_string(), baud))
                    },
                    "windows" => {
                        // NEW: Support for direct Windows Spooler printing
                        tracing::info!("Loading Windows Printer: {}", p_conf.connection);
                        Arc::new(WindowsPrinter::new(p_conf.connection.clone()))
                    },
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
            // Need read access to printers to link them
            let printers = self.printers.read().await;

            for d_conf in &config.drawers {
                let drawer: Arc<dyn Drawer> = match d_conf.device_type.as_str() {
                   "mock" => Arc::new(MockDrawer::new(d_conf.id.clone())),
                   "printer_driven" => {
                        // Find the printer
                        let target_printer_id = d_conf.connection.as_deref().unwrap_or("unknown");
                        if let Some(printer) = printers.get(target_printer_id) {
                             Arc::new(PrinterDrivenDrawer::new(d_conf.id.clone(), printer.clone()))
                        } else {
                            tracing::error!("Drawer {} references unknown printer {}", d_conf.id, target_printer_id);
                            Arc::new(MockDrawer::new(d_conf.id.clone()))
                        }
                   },
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
                   "serial" => {
                        let conn_str = d_conf.connection.clone().unwrap_or_else(|| "COM2:9600".to_string());
                        let parts: Vec<&str> = conn_str.split(':').collect();
                        let (port, baud) = if parts.len() == 2 {
                            let baud = parts[1].parse::<u32>().unwrap_or(9600);
                            (parts[0], baud)
                        } else {
                            (conn_str.as_str(), 9600)
                        };
                        Arc::new(SerialDisplay::new(d_conf.id.clone(), port.to_string(), baud))
                   },
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
