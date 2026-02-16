use dioxus::prelude::*;
use dioxus::html::HasFileData;
use crate::components::landing::Landing;
use crate::components::player::Player;

pub fn App() -> Element {
    let mut video_url = use_signal(|| "".to_string());
    let mut status = use_signal(|| "拖曳影片至此開始播放".to_string());
    
    // Audio Player State (Global)
    use_context_provider(|| Signal::new(std::sync::Arc::new(std::sync::Mutex::new(t_player::AudioPlayer::new()))));
    let audio_player_signal = use_context::<Signal<std::sync::Arc<std::sync::Mutex<t_player::AudioPlayer>>>>();

    let mut handle_drop = move |evt: Event<DragData>| {
        if let Some(file_engine) = evt.files() {
            let files = file_engine.files();
            if !files.is_empty() {
                let path = files[0].clone();
                let p_str = path.clone();
                let ext = std::path::Path::new(&p_str).extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                
                if ["mp3", "flac", "wav", "aac", "ogg"].contains(&ext.as_str()) {
                    // Audio Mode: Use Rust Backend
                    status.set(format!("🎵 正在播放音訊: {}", path));
                    video_url.set("".to_string());
                    
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                    if let Ok(mut player) = audio_player_signal.read().lock() {
                         let _ = player.load_file(path, tx);
                    }
                    // Keep rx alive by draining events in background
                    spawn(async move {
                        while let Some(_event) = rx.recv().await {
                            // Future: process audio events for UI updates
                        }
                    });
                } else {
                    // Video Mode: Use Local Server
                    status.set(format!("載入中: {}", path));
                    let encoded = urlencoding::encode(&path);
                    let port = *crate::SERVER_PORT.get().unwrap();
                    let video_src = format!("http://127.0.0.1:{}/stream?file={}", port, encoded);
                    video_url.set(video_src);
                    
                    if let Ok(player) = audio_player_signal.read().lock() {
                         player.send_command(t_player::PlayerCommand::Stop);
                    }
                }
            }
        }
    };

    rsx! {
        // Global CSS reset - kill body margins and scrollbars
        style { "html, body {{ margin: 0; padding: 0; overflow: hidden; width: 100%; height: 100%; }}" }
        div {
            style: "height: 100vh; background: #080808; color: #fff; display: flex; flex-direction: column; overflow: hidden; font-family: system-ui, -apple-system, sans-serif;",
            autofocus: true,
            tabindex: "0",
            prevent_default: "ondragover ondrop",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                     video_url.set("".to_string());
                     status.set("拖曳影片至此開始播放".to_string());
                     if let Ok(player) = audio_player_signal.read().lock() {
                         player.send_command(t_player::PlayerCommand::Stop);
                     }
                }
            },
            ondragover: move |_| {},
            ondrop: move |evt| {
                handle_drop(evt);
            },
            
            if !video_url.read().is_empty() {
                Player {
                    video_url: video_url(),
                    on_close: move |_| {
                         video_url.set("".to_string());
                         status.set("拖曳影片至此開始播放".to_string());
                    }
                }
            } else {
                Landing { status: status() }
            }
        }
    }
}
