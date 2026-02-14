use std::sync::Arc;
use tracing::{info, error};
use pos_hardware_service::{config, logging, device_manager, security, socket, utils};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ------------------------------------------------------------------------
    // STEP 1: Load Configuration
    // ------------------------------------------------------------------------
    // This reads the 'config.toml' file to find out what port to listen on
    // and what printers/devices are configured.
    let settings = match config::Settings::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // ------------------------------------------------------------------------
    // STEP 2: Initialize Logging
    // ------------------------------------------------------------------------
    // This sets up the system to print messages to the terminal/console.
    // Info/Error messages help you see what the service is doing.
    // We keep _guard alive for the duration of main to ensure logs are flushed.
    let (subscriber, _guard) = logging::get_subscriber(&settings.log_level);
    
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Failed to set global logger: {}", e);
    }

    info!("Starting POS Hardware Service v{}", utils::get_version());
    info!("Configuration loaded. Port: {}, Log Level: {}", settings.port, settings.log_level);

    // ------------------------------------------------------------------------
    // STEP 3: Initialize Hardware Devices
    // ------------------------------------------------------------------------
    // This 'DeviceManager' is the brain that holds connections to all your printers.
    // It creates the "driver" for each printer defined in config.toml.
    let device_manager = Arc::new(device_manager::DeviceManager::new());
    match device_manager.load_from_config(&settings.devices).await {
        Ok(_) => info!("Device configuration loaded successfully"),
        Err(e) => {
            error!("Failed to load devices: {}", e);
            return Err(e.into());
        }
    }

    // ------------------------------------------------------------------------
    // STEP 4: Initialize Security
    // ------------------------------------------------------------------------
    // This stores the password/token that the POS app must provide to be allowed in.
    let security = Arc::new(security::SecurityManager::new(settings.auth_token.clone()));

    // ------------------------------------------------------------------------
    // STEP 5: Start WebSocket Server
    // ------------------------------------------------------------------------
    // This opens the network port (e.g. 8080) and waits for the POS app (client)
    // to connect. It creates a loop that runs forever until you stop the program.
    info!("Initializing WebSocket server...");
    if let Err(e) = socket::run_server(settings.port, device_manager, security).await {
        error!("Server crashed: {}", e);
        return Err(e.into());
    }

    Ok(())
}
