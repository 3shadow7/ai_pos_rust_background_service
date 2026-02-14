use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use jni::objects::{JClass, JString};
#[cfg(target_os = "android")]
use jni::sys::jstring;

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_pos_hardware_HardwareService_startServer(
    env: JNIEnv,
    _class: JClass,
    port: jni::sys::jint,
) {
    // Android logging setup
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Info),
    );

    // We need to run the tokio runtime in a separate thread because JNI calls are blocking
    // and we don't want to freeze the UI (or Service main thread)
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Load default "production" settings for Android or minimal config
            // For now, we will just construct a hardcoded DeviceManager or try to read a local config file
            // Note: On Android, reading "config.toml" from assets is harder in Rust without passing context.
            // Simplified: We assume defaults or pass config via Arguments later.
            
            log::info!("Starting Android Rust Service on port {}", port);
            
            let device_manager = Arc::new(crate::device_manager::DeviceManager::new());
             // Initialize with defaults intentionally for now as we can't easily read file yet
            
            let security = Arc::new(crate::security::SecurityManager::new("android_secret".to_string()));

            if let Err(e) = crate::socket::run_server(port as u16, device_manager, security).await {
                log::error!("Android Server Failed: {}", e);
            }
        });
    });
}
