use std::sync::Arc;
use tracing::{info, error};
use pos_hardware_service::{config, logging, device_manager, security, socket, utils};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load Config
    let settings = match config::Settings::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // 2. Init Logging
    // We keep _guard alive for the duration of main
    let (subscriber, _guard) = logging::get_subscriber(&settings.log_level);
    
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Failed to set global logger: {}", e);
    }

    info!("Starting POS Hardware Service v{}", utils::get_version());
    info!("Configuration loaded. Port: {}, Log Level: {}", settings.port, settings.log_level);

    // 3. Init Device Manager
    let device_manager = Arc::new(device_manager::DeviceManager::new());
    match device_manager.load_from_config(&settings.devices).await {
        Ok(_) => info!("Device configuration loaded successfully"),
        Err(e) => {
            error!("Failed to load devices: {}", e);
            return Err(e.into());
        }
    }

    // 4. Init Security
    let security = Arc::new(security::SecurityManager::new(settings.auth_token.clone()));

    // 5. Start Server
    info!("Initializing WebSocket server...");
    if let Err(e) = socket::run_server(settings.port, device_manager, security).await {
        error!("Server crashed: {}", e);
        return Err(e.into());
    }

    Ok(())
}
