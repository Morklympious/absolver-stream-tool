mod server;
use serde::{Deserialize, Serialize};
use std::{sync::Arc};
use tokio::sync::{Mutex, broadcast};
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn grunkle(name: &str) -> String {
    format!("HEY YOU PIECE OF SHIT. {}!", name)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DisplayData {
    message : String,
}

impl Default for DisplayData {
    fn default() -> Self {
        Self {
            message: "Websocket says hello!".to_string()
        }
    }
}

struct ApplicationState {
    channel: broadcast::Sender<DisplayData>,
    data: Arc<Mutex<DisplayData>>,
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn update(message: String, state: tauri::State<'_, ApplicationState>) -> Result<(), String> {
  println!("Updating display data! {}", message);

  let data = DisplayData { message };

  { 
    let mut payload = state.data.lock().await;
    *payload = data.clone();
  }

  // Well this is... fun.
  state
    .channel
    .send(data)
    .map_err(|e| format!("Failed to broadcast: {}", e))?;

  Ok(())
}

async fn websockify(    
    channel: broadcast::Sender<DisplayData>,
    data: Arc<Mutex<DisplayData>>
) {
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;

    let listener =  TcpListener::bind("127.0.0.1:9001")
    .await
    .expect("Failed to bind websocket server");

    println!("🚀 WebSocket server running on ws://127.0.0.1:9001");

    /* 
     * This is like an if-init statement in C++. basically
     * the body is executed if the await statement doesn't fucking explode.
     */
    while let Ok((stream, address)) = listener.accept().await {
        println!("✅ New WebSocket connection from: {}", address);

        // We're cloning the payload in this closure we passed in.
        let payload = data.clone();
        let mut rx = channel.subscribe();  

        // Okay so now I guess we spawn a websocket server with this shit.
        tokio::spawn(async move {
            let stream = match accept_async(stream).await {
                Ok(websocket) => websocket,
                Err(error) => {
                    eprintln!("[ERROR] Websocket handshake failed: {}", error);
                    return;
                }
            };

            let (mut write, mut read) = stream.split();

            {
                let _data = payload.lock().await;

                // Excuse me what the fuck is this &* shit.
                let json = serde_json::to_string(&*_data).unwrap();


                if let Err(e) = write.send(Message::Text(json)).await {
                    eprintln!("❌ Failed to send initial state: {}", e);
                    return;
                }

                println!("📤 Sent initial state to client");
            }

            let mut readify = tokio::spawn(async move {
                while let Some(message) = read.next().await {
                    if let Ok(Message::Close(_)) = message {
                        break;
                    }
                }
            });

            loop {
                tokio::select! {
                    result = rx.recv() => {
                        match result {
                            Ok(data) => {
                                let json = serde_json::to_string(&data).unwrap();
                                if let Err(e) = write.send(Message::Text(json)).await {
                                    eprintln!("❌ Failed to send update: {}", e);
                                    break;
                                }
                                println!("📤 Broadcast sent to client");
                            }
                            Err(broadcast::error::RecvError::Closed) => {
                                println!("🔴 Broadcast channel closed");
                                break;
                            }
                            Err(broadcast::error::RecvError::Lagged(_)) => {
                                eprintln!("⚠️ Client lagged behind");
                            }
                        }
                    }
                    _ = &mut readify => {
                        println!("Disconnected");
                        break;
                    }
                }
            }

            println!("👋 Client connection closed");
        });
    };
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_channel, _rx) = broadcast::channel::<DisplayData>(100);
    let _data = Arc::new(Mutex::new(DisplayData::default()));

    tauri::Builder::default()
        .manage(ApplicationState {
            channel: _channel.clone(),
            data: _data.clone()
        })
        .setup(move |_application| {
            println!("🎯 Starting WebSocket server...");
            
            tauri::async_runtime::spawn(async move {
                websockify(_channel, _data).await;
            });

            // Start HTTP server (only in production)
            #[cfg(not(debug_assertions))]
            {
                tauri::async_runtime::spawn(async move {
                    server::serve().await;  // ← Add this!
                });
                 println!("🎯 Starting HTTP server...");
            }

            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, grunkle, update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
