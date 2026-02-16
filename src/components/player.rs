use dioxus::prelude::*;
use dioxus::desktop::{use_window, LogicalSize};

#[component]
pub fn Player(video_url: String, on_close: EventHandler<()>) -> Element {
    let window = use_window();

    // One-time setup: listen for video metadata to resize window
    use_effect(move || {
        let mut handle = eval(r#"
            const video = document.getElementById('main-video');
            if (!video) return;

            video.onloadedmetadata = () => {
                dioxus.send({ type: 'resize', width: video.videoWidth, height: video.videoHeight });
            };
        "#);

        let win = window.clone();
        spawn(async move {
            loop {
                match handle.recv().await {
                    Ok(msg) => {
                        let msg_type = msg["type"].as_str().unwrap_or("");
                        if msg_type == "resize" {
                            let vw = msg["width"].as_f64().unwrap_or(0.0);
                            let vh = msg["height"].as_f64().unwrap_or(0.0);
                            if vw > 0.0 && vh > 0.0 {
                                let aspect = vw / vh;
                                let max_w: f64 = 1440.0;
                                let max_h: f64 = 900.0;
                                let min_w: f64 = 640.0;
                                let min_h: f64 = 400.0;

                                let (mut w, mut h) = (vw, vh);
                                if w > max_w { w = max_w; h = w / aspect; }
                                if h > max_h { h = max_h; w = h * aspect; }
                                if w < min_w { w = min_w; h = w / aspect; }
                                if h < min_h { h = min_h; w = h * aspect; }

                                win.set_inner_size(LogicalSize::new(w, h));
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    });

    rsx! {
        div {
            style: "width: 100%; height: 100%; position: relative; background: black; overflow: hidden;",

            // Native HTML5 video with built-in controls
            video {
                id: "main-video",
                src: "{video_url}",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; object-fit: contain;",
                autoplay: true,
                controls: true,
            }

            // Close Button
            div {
                style: "position: absolute; top: 12px; right: 12px; cursor: pointer; z-index: 20;
                        width: 36px; height: 36px; display: flex; align-items: center; justify-content: center;
                        background: rgba(0,0,0,0.6); border-radius: 50%; color: white; font-size: 18px;
                        transition: background 0.2s;",
                onclick: move |_| on_close.call(()),
                "X"
            }
        }
    }
}
