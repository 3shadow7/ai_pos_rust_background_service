use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::device_manager::DeviceManager;
use crate::security::SecurityManager;
use crate::errors::ServiceError;
use tracing::{info, error, warn, debug};

// =========================================================================
// PROTOCOL DEFINITIONS
// =========================================================================
// These structures define the JSON messages that the POS Client sends to us.
// e.g., { "type": "auth", "token": "..." }
// e.g., { "type": "print", "device_id": "printer_kitchen", "data": { "text": "Hello" } }

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    // Step 1: Client must send this first to log in.
    Auth { token: String },
    
    // Command to print something.
    Print { device_id: String, data: PrintData },
    
    // Command to pop the cash drawer open.
    OpenDrawer { device_id: String },
    
    // Command to show text on the customer pole display.
    DisplayUpdate { device_id: String, data: DisplayData },
}

#[derive(Deserialize, Debug)]
pub struct PrintData {
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DisplayData {
    pub line1: String,
    pub line2: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub status: String, // "ok" or "error"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// -------------------------------------------------------------------------
// SERVER LOGIC
// -------------------------------------------------------------------------

pub async fn run_server(port: u16, devices: Arc<DeviceManager>, security: Arc<SecurityManager>) -> Result<(), ServiceError> {
    let addr = format!("127.0.0.1:{}", port);
    // Bind to the local TCP port
    let listener = TcpListener::bind(&addr).await
        .map_err(|e| ServiceError::IoError(e.to_string()))?;
    
    info!("WebSocket server listening on {}", addr);

    // Accept incoming connections in a loop
    while let Ok((stream, _)) = listener.accept().await {
        let devices = devices.clone();
        let security = security.clone();
        // Spawn a new background task for each client connection
        tokio::spawn(accept_connection(stream, devices, security));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, devices: Arc<DeviceManager>, security: Arc<SecurityManager>) {
    let addr = stream.peer_addr().unwrap_or_else(|_| "unknown".parse().unwrap());
    info!("Incoming connection from {}", addr);

    // Perform the WebSocket Handshake (upgrade TCP to WebSocket)
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    let mut authenticated = false; // connection starts unauthenticated

    // Loop through every message the client sends
    while let Some(msg_result) = read.next().await {
        match msg_result {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    debug!("Received: {}", text);

                    // Process the command and get a result
                    let result = process_message(text, &mut authenticated, &devices, &security).await;
                    
                    // Send the result back to the client as JSON
                    let response_json = serde_json::to_string(&result).unwrap();
                    if let Err(e) = write.send(tokio_tungstenite::tungstenite::Message::Text(response_json)).await {
                        error!("Failed to send response: {}", e);
                        break;
                    }
                } else if msg.is_close() {
                    info!("Client disconnected");
                    break;
                }
            }
            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}

async fn process_message(text: &str, authenticated: &mut bool, devices: &Arc<DeviceManager>, security: &Arc<SecurityManager>) -> Response {
    let command: Result<Command, _> = serde_json::from_str(text);

    match command {
        Ok(Command::Auth { token }) => {
            if security.validate_token(&token) {
                *authenticated = true;
                Response { status: "ok".into(), device_id: None, message: Some("Authenticated".into()) }
            } else {
                warn!("Authentication failed with token: {}", token);
                Response { status: "error".into(), device_id: None, message: Some("Invalid token".into()) }
            }
        }
        Ok(_) if !*authenticated => {
            warn!("Unauthorized command attempt");
            Response { status: "error".into(), device_id: None, message: Some("Authentication required".into()) }
        }
        Ok(Command::Print { device_id, data }) => {
            if let Some(printer) = devices.get_printer(&device_id).await {
                if let Some(text) = data.text {
                     match printer.print_text(&text).await {
                         Ok(_) => Response { status: "ok".into(), device_id: Some(device_id), message: None },
                         Err(e) => Response { status: "error".into(), device_id: Some(device_id), message: Some(e.to_string()) },
                     }
                } else {
                    Response { status: "error".into(), device_id: Some(device_id), message: Some("No text provided".into()) }
                }
            } else {
                Response { status: "error".into(), device_id: Some(device_id), message: Some("Device not found".into()) }
            }
        }
        Ok(Command::OpenDrawer { device_id }) => {
            if let Some(drawer) = devices.get_drawer(&device_id).await {
                match drawer.open().await {
                    Ok(_) => Response { status: "ok".into(), device_id: Some(device_id), message: None },
                    Err(e) => Response { status: "error".into(), device_id: Some(device_id), message: Some(e.to_string()) },
                }
            } else {
                 Response { status: "error".into(), device_id: Some(device_id), message: Some("Device not found".into()) }
            }
        }
        Ok(Command::DisplayUpdate { device_id, data }) => {
            if let Some(display) = devices.get_display(&device_id).await {
                 match display.show_text(&data.line1, &data.line2).await {
                    Ok(_) => Response { status: "ok".into(), device_id: Some(device_id), message: None },
                    Err(e) => Response { status: "error".into(), device_id: Some(device_id), message: Some(e.to_string()) },
                 }
            } else {
                 Response { status: "error".into(), device_id: Some(device_id), message: Some("Device not found".into()) }
            }
        }
        Err(e) => {
            warn!("Invalid JSON: {}", e);
            Response { status: "error".into(), device_id: None, message: Some(format!("Invalid JSON format: {}", e)) }
        }
    }
}
