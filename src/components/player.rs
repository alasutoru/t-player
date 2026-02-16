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

            // ── Resume from last position ──
            const url = new URL(video.src);
            const filePath = url.searchParams.get('file');
            const storageKey = 'tplayer_' + filePath;

            const savedPos = localStorage.getItem(storageKey);
            if (savedPos) {
                video.currentTime = parseFloat(savedPos);
            }

            // Save position every 3s (skip if near end)
            let lastSave = 0;
            video.ontimeupdate = () => {
                const now = Date.now();
                if (now - lastSave > 3000) {
                    if (video.duration - video.currentTime > 5) {
                        localStorage.setItem(storageKey, video.currentTime);
                    }
                    lastSave = now;
                }
            };

            // Clear on ended (next play starts fresh)
            video.onended = () => {
                localStorage.removeItem(storageKey);
            };


            // ── Playback speed control (Arrow keys) ──
            const speeds = [0.5, 1.0, 1.25, 1.5, 2.0];
            let speedIndex = 1;

            function showSpeed(rate) {
                let el = document.getElementById('speed-osd');
                if (!el) {
                    el = document.createElement('div');
                    el.id = 'speed-osd';
                    el.style.cssText = 'position:fixed;top:20px;left:50%;transform:translateX(-50%);background:rgba(0,0,0,0.75);color:white;padding:8px 24px;border-radius:8px;font-size:20px;font-family:monospace;z-index:100;transition:opacity 0.5s;pointer-events:none;';
                    document.body.appendChild(el);
                }
                el.textContent = rate + 'x';
                el.style.opacity = '1';
                clearTimeout(el._t);
                el._t = setTimeout(() => el.style.opacity = '0', 1000);
            }

            document.addEventListener('keydown', (e) => {
                if (e.key === ']') {
                    if (speedIndex < speeds.length - 1) speedIndex++;
                    video.playbackRate = speeds[speedIndex];
                    showSpeed(speeds[speedIndex]);
                }
                if (e.key === '[') {
                    if (speedIndex > 0) speedIndex--;
                    video.playbackRate = speeds[speedIndex];
                    showSpeed(speeds[speedIndex]);
                }
                if (e.key === 'ArrowRight') {
                    e.preventDefault();
                    video.currentTime = Math.min(video.currentTime + 15, video.duration);
                }
                if (e.key === 'ArrowLeft') {
                    e.preventDefault();
                    video.currentTime = Math.max(video.currentTime - 15, 0);
                }
                if (e.code === 'Space') {
                    e.preventDefault();
                    if (video.paused) video.play(); else video.pause();
                }
            });

            // ── Resize window + auto-load subtitle ──
            video.onloadedmetadata = () => {
                dioxus.send({ type: 'resize', width: video.videoWidth, height: video.videoHeight });

                // Auto-load same-name subtitle (.srt/.vtt)
                const subUrl = video.src.replace('/stream?', '/subtitle?');
                const track = document.createElement('track');
                track.kind = 'subtitles';
                track.label = 'Subtitles';
                track.src = subUrl;
                track.default = true;
                video.appendChild(track);
                track.track.mode = 'showing';
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
