# Production Deployment Guide

## 1. Building for Production

### Windows & Linux & macOS
Run the release build:
```bash
cargo build --release
```
The binary will be located at `target/release/pos_hardware_service`.

---

## 2. Windows Deployment (Windows Service)

To run as a generic Windows Service, we recommend using a service wrapper like [NSSM](https://nssm.cc/) or `winsw`, as the current implementation is a console application.

### Using NSSM (Non-Sucking Service Manager)

1.  **Download NSSM** from https://nssm.cc/download
2.  **Install the Service**:
    Open Administrator Command Prompt:
    ```cmd
    nssm install PosHardwareService "C:\path\to\pos_hardware_service.exe"
    ```
3.  **Configure Directory**:
    ```cmd
    nssm set PosHardwareService AppDirectory "C:\path\to\working_directory"
    ```
    *Note: Ensure `config.toml` and `logs/` directory are accessible in the AppDirectory.*
4.  **Set Auto Restart**:
    NSSM handles auto-restart by default.
5.  **Start the Service**:
    ```cmd
    nssm start PosHardwareService
    ```

---

## 3. Linux Deployment (Systemd)

1.  **Copy Binary**:
    ```bash
    sudo cp target/release/pos_hardware_service /usr/local/bin/
    sudo mkdir -p /etc/pos_hardware_service
    sudo cp config.toml /etc/pos_hardware_service/
    ```
    *Note: You might need to adjust the code to look for config in `/etc/` or pass it as arg, or simply run from a specific working directory.*

2.  **Create Service File**: `/etc/systemd/system/pos_hardware.service`

    ```ini
    [Unit]
    Description=POS Hardware Service
    After=network.target

    [Service]
    Type=simple
    User=nobody
    WorkingDirectory=/opt/pos_hardware_service
    ExecStart=/usr/local/bin/pos_hardware_service
    Restart=always
    RestartSec=5
    Environment=POS_LOG_LEVEL=info

    [Install]
    WantedBy=multi-user.target
    ```

3.  **Setup Directory**:
    ```bash
    sudo mkdir -p /opt/pos_hardware_service
    sudo cp config.toml /opt/pos_hardware_service/
    sudo chown -R nobody:nobody /opt/pos_hardware_service
    ```

4.  **Enable and Start**:
    ```bash
    sudo systemctl daemon-reload
    sudo systemctl enable pos_hardware.service
    sudo systemctl start pos_hardware.service
    ```

---

## 4. macOS Deployment (Launchd)

1.  **Create Plist**: `~/Library/LaunchAgents/com.pos.hardware.plist`

    ```xml
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
        <key>Label</key>
        <string>com.pos.hardware</string>
        <key>ProgramArguments</key>
        <array>
            <string>/usr/local/bin/pos_hardware_service</string>
        </array>
        <key>WorkingDirectory</key>
        <string>/opt/pos_hardware_service</string>
        <key>RunAtLoad</key>
        <true/>
        <key>KeepAlive</key>
        <true/>
    </dict>
    </plist>
    ```

2.  **Load Service**:
    ```bash
    launchctl load ~/Library/LaunchAgents/com.pos.hardware.plist
    ```

---

## 5. Android Support

Since Rust cannot run as a standalone system service on Android in the traditional sense without root, we use a **Kotlin Foreground Service** that loads the Rust code via JNI.

### Architecture
- **Kotlin App**: A minimal Android app with a Foreground Service.
- **JNI Bridge**: Rust code exposed via `extern "C"` functions.
- **Rust Lib**: The core POS service compiled as `libpos_hardware_service.so`.

### Steps:

1.  **Modify Cargo.toml**:
    Add `crate-type = ["cdylib"]` to `[lib]`.

    ```toml
    [lib]
    name = "pos_hardware_service"
    crate-type = ["cdylib"]
    ```

2.  **JNI Functions (src/lib.rs)**:
    You need to expose an entry point.
    ```rust
    #[no_mangle]
    pub extern "C" fn Java_com_example_pos_PosService_startRustServer(env: JNIEnv, _: JClass) {
        // Run the tokio runtime here on a new thread
        std::thread::spawn(|| {
            // ... invoke main logic
        });
    }
    ```

3.  **Compile for Android**:
    Install targets:
    ```bash
    rustup target add aarch64-linux-android armv7-linux-androideabi
    ```
    Use [cargo-ndk](https://github.com/bbqsrc/cargo-ndk) to build:
    ```bash
    cargo ndk -t arm64-v8a -t armeabi-v7a -o ./android/app/src/main/jniLibs build --release
    ```

4.  **Android App**:
    - Load library in `MainActivity` or `Service`: `System.loadLibrary("pos_hardware_service")`
    - Call the native method `startRustServer()` in `onStartCommand` of a Foreground Service.
    - Ensure `WakeLock` is acquired to keep CPU running.

