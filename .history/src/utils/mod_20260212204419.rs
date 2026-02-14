pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn log_available_ports() {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                tracing::info!("No serial ports detected on this system.");
            } else {
                let port_names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
                tracing::info!("Detected Serial Ports: {}", port_names.join(", "));
                tracing::info!("Hint: Update config.toml with one of these ports if using Serial/USB devices.");
            }
        },
        Err(e) => {
            tracing::warn!("Failed to list serial ports: {}", e);
        }
    }
}
