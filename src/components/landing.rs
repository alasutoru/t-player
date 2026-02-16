use dioxus::prelude::*;

#[component]
pub fn Landing(status: String) -> Element {
    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; border: 2px dashed #444; margin: 40px; border-radius: 12px; transition: all 0.3s; background: linear-gradient(135deg, rgba(255,255,255,0.03) 0%, rgba(255,255,255,0.01) 100%);",
            div {
                style: "font-size: 4rem; margin-bottom: 20px; opacity: 0.5;",
                "📼"
            }
            h1 { 
                style: "font-weight: 500; color: #eee; font-size: 1.4rem; letter-spacing: 0.5px;",
                "{status}" 
            }
            p { 
                style: "color: #888; margin-top: 10px; font-size: 0.9rem; font-family: monospace;",
                "支援格式: MP4, MOV, MKV, WebM | MP3, FLAC, WAV, AAC, OGG" 
            }
        }
    }
}
