
description: You are a senior systems architect and Rust expert.

I want you to generate a COMPLETE production-grade cross-platform POS Hardware Service system from zero to production.

This system must be scalable, secure, modular, and enterprise-ready.

========================================
GOAL
========================================

Build a cross-platform POS hardware bridge using:

- Rust as the core engine
- WebSocket communication (NOT HTTP)
- Secure authentication
- Multi-device scalable architecture
- Android support using minimal Kotlin wrapper when necessary

This service connects an Angular PWA POS system with:

- Multiple receipt printers (ESC/POS)
- Multiple cash drawers
- Multiple customer displays
- Future hardware extensions

========================================
SUPPORTED PLATFORMS
========================================

Desktop:
- Windows (as Windows Service)
- Linux (systemd service)
- macOS (launchd service)

Mobile:
- Android (Rust compiled as native .so + Kotlin foreground service wrapper)

Explain clearly:
- Why Android requires Kotlin wrapper
- How Rust integrates using JNI
- How to keep Kotlin minimal (thin bridge only)

========================================
ARCHITECTURE REQUIREMENTS
========================================

Design enterprise architecture with clear modular separation:

src/
 ├── main.rs
 ├── config/
 ├── security/
 ├── socket/
 ├── device_manager/
 ├── hardware/
 │     ├── traits.rs
 │     ├── printer/
 │     ├── drawer/
 │     ├── display/
 ├── logging/
 ├── errors/
 ├── utils/

========================================
SCALABLE DEVICE ARCHITECTURE
========================================

Implement a DeviceManager:

- HashMap<String, Box<dyn Printer>>
- HashMap<String, Box<dyn Drawer>>
- HashMap<String, Box<dyn Display>>

Each device:
- Has unique ID
- Loaded dynamically from config.toml
- Supports multiple instances
- Can be extended later

========================================
WEBSOCKET COMMUNICATION
========================================

Use:
- Tokio
- tokio-tungstenite
- serde
- tracing

Server must:
- Bind to localhost only by default
- Require authentication handshake
- Support JSON commands

Command format:

{
  "type": "auth",
  "token": "DEVICE_SECRET"
}

{
  "type": "print",
  "device_id": "printer_1",
  "data": { "text": "Invoice text" }
}

{
  "type": "open_drawer",
  "device_id": "drawer_1"
}

{
  "type": "display_update",
  "device_id": "display_main",
  "data": { "line1": "...", "line2": "..." }
}

Responses must include:
- status
- device_id
- error message if failed

========================================
SECURITY REQUIREMENTS
========================================

- Token-based authentication
- Reject unauthorized connections
- Rate limiting
- Input validation
- Structured error responses
- Logging all commands
- Graceful shutdown
- Crash recovery support

========================================
HARDWARE LAYER
========================================

Define traits:

trait Printer
trait Drawer
trait Display

Provide:

- Mock implementations
- ESC/POS example implementation
- Serial/USB abstraction layer
- Clear explanation of how to extend drivers

========================================
CONFIGURATION SYSTEM
========================================

Use config.toml:

- port
- auth_token
- log_level
- devices:
    printers:
      - id
      - type
      - connection details
    drawers:
      - id
      - type
    displays:
      - id
      - type

Load config at startup.

========================================
LOGGING
========================================

Use tracing crate:

- File logging
- Rotation support
- Structured logs
- Production-safe log levels

========================================
PRODUCTION DEPLOYMENT
========================================

Explain step-by-step:

WINDOWS:
- Build release
- Register as Windows Service
- Auto restart on crash

LINUX:
- Cross-compile
- systemd service file
- Restart policy

MACOS:
- launchd service configuration

ANDROID:
- Compile Rust to .so
- Create minimal Kotlin Android app
- Foreground service
- JNI bridge
- Permission handling
- Battery optimization handling

========================================
BEGINNER FRIENDLY
========================================

This is my first Rust project.

Explain clearly:
- Project structure
- Async in Rust
- Ownership basics when relevant
- Trait objects
- Error handling patterns
- Why each architectural decision is made

========================================
IMPORTANT
========================================

Do NOT simplify architecture.
Make it enterprise-grade.
Make it scalable.
Make it clean.
Add detailed comments.
Generate code step-by-step.
Explain every file.

Start from project initialization.
Then build modules incrementally.
End with production deployment guide.
