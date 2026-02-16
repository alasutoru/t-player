use dioxus::prelude::*;

#[component]
pub fn Player(video_url: String, on_close: EventHandler<()>) -> Element {
    let mut is_playing = use_signal(|| true);
    let mut duration = use_signal(|| 0.0f64);
    let mut current_time = use_signal(|| 0.0f64);
    let mut volume = use_signal(|| 1.0f64);
    let mut show_controls = use_signal(|| true);
    let mut is_dragging = use_signal(|| false);

    // ── JS → Rust: One-time event bridge (use_effect runs once on mount) ──
    use_effect(move || {
        let mut handle = eval(r#"
            const video = document.getElementById('main-video');
            if (!video) return;

            video.play().catch(e => console.error(e));
            video.volume = 1.0;

            let lastSend = 0;
            video.ontimeupdate = () => {
                const now = Date.now();
                if (now - lastSend > 500) {
                    dioxus.send({ type: 'time', val: video.currentTime });
                    lastSend = now;
                }
            };
            video.ondurationchange = () => dioxus.send({ type: 'duration', val: video.duration });
            video.onended = () => dioxus.send({ type: 'ended' });
        "#);

        // Spawn async task to drain messages from JS
        spawn(async move {
            loop {
                match handle.recv().await {
                    Ok(msg) => {
                        let msg_type = msg["type"].as_str().unwrap_or("");
                        let val = msg["val"].as_f64().unwrap_or(0.0);
                        match msg_type {
                            "time" => {
                                if !is_dragging() {
                                    current_time.set(val);
                                }
                            }
                            "duration" => duration.set(val),
                            "ended" => is_playing.set(false),
                            _ => {}
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    });

    // ── Rust → JS: Fire-and-forget direct DOM commands ──
    // No shared channel needed. Each eval() call is independent.

    rsx! {
        div {
            style: "width: 100vw; height: 100vh; position: relative; background: black; overflow: hidden;",
            onmousemove: move |_| {
                show_controls.set(true);
            },

            // Video Element
            video {
                id: "main-video",
                src: "{video_url}",
                style: "width: 100%; height: 100%; object-fit: contain;",
                autoplay: true,
            }

            // Close Button
            div {
                style: "position: absolute; top: 20px; right: 20px; cursor: pointer; z-index: 20;
                        width: 40px; height: 40px; display: flex; align-items: center; justify-content: center;
                        background: rgba(0,0,0,0.5); border-radius: 50%; color: white; font-size: 20px;",
                onclick: move |_| on_close.call(()),
                "X"
            }

            // Controls Overlay
            div {
                style: "
                    position: absolute; bottom: 0; left: 0; width: 100%; z-index: 10;
                    background: linear-gradient(to top, rgba(0,0,0,0.9), transparent);
                    padding: 20px 30px 40px 30px;
                    display: flex; flex-direction: column; gap: 15px;
                    transition: opacity 0.3s;
                ",
                opacity: if show_controls() { "1" } else { "0" },

                // Progress Bar
                input {
                    r#type: "range",
                    min: "0",
                    max: "{duration}",
                    value: "{current_time}",
                    step: "0.1",
                    style: "width: 100%; cursor: pointer; accent-color: #e50914; height: 5px;",
                    onmousedown: move |_| is_dragging.set(true),
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<f64>() {
                            current_time.set(val);
                        }
                    },
                    onmouseup: move |_| {
                        is_dragging.set(false);
                        let val = current_time();
                        // Direct DOM seek (fire-and-forget, no channel needed)
                        eval(&format!("document.getElementById('main-video').currentTime = {};", val));
                    },
                }

                // Control Row
                div {
                    style: "display: flex; align-items: center; gap: 20px; color: white;",

                    // Play/Pause
                    div {
                        style: "cursor: pointer; width: 30px; text-align: center;",
                        onclick: move |_| {
                            let playing = is_playing();
                            is_playing.set(!playing);
                            if playing {
                                eval("document.getElementById('main-video').pause();");
                            } else {
                                eval("document.getElementById('main-video').play();");
                            }
                        },
                        if is_playing() {
                            svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "white", path { d: "M6 19h4V5H6v14zm8-14v14h4V5h-4z" } }
                        } else {
                            svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "white", path { d: "M8 5v14l11-7z" } }
                        }
                    }

                    // Time Display
                    div {
                        style: "font-family: monospace; font-size: 14px; color: #ddd;",
                        "{format_time(current_time())} / {format_time(duration())}"
                    }

                    div { style: "flex: 1;" }

                    // Volume
                    div {
                        style: "display: flex; align-items: center; gap: 10px; color: white;",
                        "Vol",
                        input {
                            r#type: "range", min: "0", max: "1", step: "0.05", value: "{volume}",
                            style: "width: 80px; accent-color: white;",
                            oninput: move |evt| {
                                if let Ok(val) = evt.value().parse::<f64>() {
                                    volume.set(val);
                                    eval(&format!("document.getElementById('main-video').volume = {};", val));
                                }
                            }
                        }
                    }

                    // Fullscreen
                    div {
                        style: "cursor: pointer; margin-left: 10px;",
                        onclick: move |_| {
                            eval("if(document.fullscreenElement) document.exitFullscreen(); else document.getElementById('main-video').requestFullscreen();");
                        },
                        "[F]"
                    }
                }
            }
        }
    }
}

fn format_time(seconds: f64) -> String {
    let seconds = seconds as u64;
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}
