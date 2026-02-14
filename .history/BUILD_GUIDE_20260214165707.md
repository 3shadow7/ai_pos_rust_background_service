# BUILD & DEPLOYMENT GUIDE

This guide explains how to build the POS Hardware Service for every platform.

---

## 🟢 1. Windows (Your standard workflow)

**How to Build:**
1.  Open the `scripts` folder.
2.  Right-click `package_for_windows.ps1` -> **Run with PowerShell**.

**Result:**
*   A folder named `dist` appears directly in your project root.
*   Zip this folder and send it to your Windows clients.

---

## 🐧 2. Linux (Ubuntu, Debian, Raspberry Pi)

**Requirement:** You must run this command **ON A LINUX MACHINE**. You cannot typically build Linux binaries from Windows easily.

**How to Build:**
1.  Copy this entire project to the Linux machine.
2.  Open Terminal.
3.  Run:
    ```bash
    chmod +x scripts/package_for_linux.sh
    ./scripts/package_for_linux.sh
    ```

**Result:**
*   A file `pos_hardware_linux.tar.gz` is created.

---

## 🍎 3. macOS (Apple)

**Requirement:** You must run this command **ON A MAC**.

**How to Build:**
1.  Copy this entire project to the Mac.
2.  Open Terminal.
3.  Run:
    ```bash
    chmod +x scripts/package_for_macos.sh
    ./scripts/package_for_macos.sh
    ```

**Result:**
*   A file `pos_hardware_macos.zip` is created.

---

## 🤖 4. Android (The Mobile App)

You asked: *"Which is better, Flutter or Kotlin?"*

**Answer:** For this specific project, **Native Android (Java/Kotlin)** is the correct choice.
*   **Why?** Your app has no UI. It is a "Background Service" (a server). Flutter is a UI framework (like making video games or websites). Using Flutter just to run a silent background server is heavy and battery-draining. Native Android is lightweight and handles USB/Background tasks 10x better.
*   **Is it hard?** No. I have automated it below.

**Prerequisites (One Time Setup):**
1.  **Install Android Studio**: [Download Here](https://developer.android.com/studio)
2.  **Install Rust Targets**: Open your PC terminal and run:
    ```powershell
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
    cargo install cargo-ndk
    ```

**How to Build (Easy Code-Free Way):**
1.  Open **Android Studio**.
2.  Select **Open Project** -> Navigate to the `android` folder inside this project.
3.  On the right side, there is a green "Play" (Run) button. **Click it.**
    *   *Note:* I added a script inside the project that automatically compiles the Rust code for you when you hit Play.
4.  To create the APK file to send to phones:
    *   Go to menu: **Build -> Build Bundle(s) / APK(s) -> Build APK(s)**.

**Result:**
*   The `.apk` file will be generated. You can install this on any Android POS terminal.
