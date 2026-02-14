# POS Hardware Service (Production Grade)

> **"The Bridge between your Web POS and the Real World"**

This software runs silently on your store's computer (Windows, Linux, or Mac). It connects your **POS Web Application** to physical hardware like **Thermal Printers**, **Cash Drawers**, and **Customer Displays**.

It handles:
1.  **USB/Serial Connections**: Talking to older hardware that web browsers can't reach.
2.  **Network Printers**: Managing reliable connections to kitchen printers.
3.  **Security**: Ensuring only YOUR POS can print.
4.  **Auto-Maintenance**: Cleaning up its own logs automatically.

---

## 🚀 The Full Scenario: How it works

1.  **You install this service** on the computer that has the printer plugged in (using a USB or Serial cable).
2.  **The service starts** automatically when the computer turns on. It listens on port `7777`.
3.  **Your POS Website** (e.g., `app.myrestaurant.com`) tries to "talk" to `localhost:7777`.
4.  The service asks for a **Password** (Auth Token).
5.  Once logged in, the POS sends JSON commands like `{"type": "print", ...}`.
6.  This service translates those commands into raw bytes and sends them to the physical printer immediately.

---

## 🛠️ Step 1: One-Time Configuration

You only need to fail this file **ONCE** per computer, unless you change the hardware.

**File:** `config.toml`

| Setting | Value | Why? |
| :--- | :--- | :--- |
| `port` | `7777` | The port the web app will connect to. |
| `auth_token` | **CHANGE ME** | **CRITICAL:** Set this to a secret password. Your POS App needs this same password to connect. |
| `log_retention_days` | `90` | Automatic cleanup. Deletes logs older than 90 days. |

### Defining Hardware (in `config.toml`)

**1. If you have a USB/Serial Printer:**
Check which "COM" port it is using (look at Device Manager on Windows).
```toml
[[devices.printers]]
id = "printer_receipt"
device_type = "serial"
connection = "COM3:9600"  # Replace COM3 with your port
```

**2. If you have a Network Printer (Ethernet/WiFi):**
```toml
[[devices.printers]]
id = "printer_kitchen"
device_type = "network"
connection = "192.168.1.50:9100" # Replace with printer IP
```

**3. If you have a Cash Drawer:**
Most drawers plug into the back of the printer.
```toml
[[devices.drawers]]
id = "drawer_1"
device_type = "printer_driven"
connection = "printer_receipt" # The ID of the printer it is connected to
```

---

## 💿 Step 2: Installation (Make it Automatic)

How to make this run forever without you touching it.

### ✅ Windows (Automatic)
1.  Navigate to the `scripts/` folder.
2.  Right-click `install_windows.ps1` and select **"Run with PowerShell"**.
3.  **Done!** It will now start automatically every time you turn on the PC.

### ✅ Linux / Raspberry Pi (Automatic)
1.  Open terminal in the project folder.
2.  Run:
    ```bash
    chmod +x scripts/install_linux.sh
    sudo scripts/install_linux.sh
    ```
3.  **Done!** It is now a system service.

### ✅ Mac
1.  Open terminal.
2.  Run: `cargo run --release`
3.  (For auto-start, use standard macOS "Automator" to run the command on login).

---

## ❓ Maintenance & FAQ

### "I changed the hardware!"
1.  Stop the service.
2.  Edit `config.toml` with the new COM port or IP.
3.  Restart the computer (or restart the service).
    *   Windows: Task Scheduler -> Run
    *   Linux: `sudo systemctl restart pos_hardware`

### "The logs are filling up my disk!"
**Fixed automatically.** The config `log_retention_days = 90` ensures files are deleted after 3 months. You don't need to do anything.

### "How do I see what ports I have?"
Just run the program manually once:
```powershell
./target/release/pos_hardware_service.exe
```
It will print: `INFO Detected Serial Ports: COM3, COM4`. Use one of those in your config.

---

## 👨‍💻 Developer Guide (Building)

If you are the developer making changes to the code:
1.  Install Rust.
2.  Build the project:
    ```bash
    cargo build --release
    ```
3.  The output file is in `target/release/`.

