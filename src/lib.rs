use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use anyhow::{Result, anyhow};
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    Seek(f64), // Seconds
    Volume(f32),
}

#[derive(Clone, Debug)]
pub enum PlayerEvent {
    TimeUpdate(f64),
    Duration(f64),
    Ended,
    Error(String),
}

pub struct PlayerState {
    pub volume: Arc<Mutex<f32>>,
    pub is_playing: Arc<AtomicBool>,
    pub position: Arc<Mutex<f64>>,
    pub duration: Arc<Mutex<f64>>,
}

pub struct AudioPlayer {
    command_tx: Option<mpsc::UnboundedSender<PlayerCommand>>,
    state: Arc<PlayerState>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            state: Arc::new(PlayerState {
                volume: Arc::new(Mutex::new(1.0)),
                is_playing: Arc::new(AtomicBool::new(false)),
                position: Arc::new(Mutex::new(0.0)),
                duration: Arc::new(Mutex::new(0.0)),
            }),
        }
    }

    pub fn load_file(&mut self, path: String, event_tx: mpsc::UnboundedSender<PlayerEvent>) -> Result<()> {
        // Stop existing playback
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(PlayerCommand::Stop);
        }

        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel();
        self.command_tx = Some(cmd_tx);
        
        let state = self.state.clone();
        
        thread::spawn(move || {
            if let Err(e) = run_audio_engine(path, state, &mut cmd_rx, event_tx.clone()) {
                let _ = event_tx.send(PlayerEvent::Error(e.to_string()));
            }
        });

        Ok(())
    }

    pub fn send_command(&self, cmd: PlayerCommand) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(cmd);
        }
    }
}

// Low-level audio engine loop
fn run_audio_engine(
    path: String, 
    state: Arc<PlayerState>, 
    cmd_rx: &mut mpsc::UnboundedReceiver<PlayerCommand>,
    event_tx: mpsc::UnboundedSender<PlayerEvent>,
) -> Result<()> {
    // 1. Setup Audio Device
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| anyhow!("No audio output device found"))?;
    let config = device.default_output_config()?;
    let _channels = config.channels() as usize; // Prefixed with underscore to suppress unused variable warning

    // 2. Open File & Decode
    let src = File::open(&path)?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension(std::path::Path::new(&path).extension().and_then(|s| s.to_str()).unwrap_or(""));

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())?;

    let mut format = probed.format;
    let track = format.tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| anyhow!("No audio track found"))?;

    let track_id = track.id;
    let track_params = track.codec_params.clone();
    
    // Send Duration
    if let Some(n_frames) = track_params.n_frames {
        if let Some(tb) = track_params.time_base {
            let duration = tb.calc_time(n_frames).seconds as f64;
            *state.duration.lock().unwrap() = duration;
            let _ = event_tx.send(PlayerEvent::Duration(duration));
        }
    }

    let mut decoder = symphonia::default::get_codecs()
        .make(&track_params, &DecoderOptions::default())?;

    // 3. Audio Streaming Logic
    let (audio_tx, audio_rx) = std::sync::mpsc::sync_channel::<Vec<f32>>(2); // Double buffer
    let volume_clone = state.volume.clone();
    let is_playing_clone = state.is_playing.clone();

    // Start CPAL stream
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            if !is_playing_clone.load(Ordering::SeqCst) {
                 for s in data.iter_mut() { *s = 0.0; }
                 return;
            }

            // Ideally we pull from a ring buffer here. 
            // For simplicity in this v1 architecture, we use a simple blocking receive 
            // which might cause glitches if decoding is slow.
            // In a production-grade engine, we would use a lock-free ring buffer (e.g. `ringbuf`).
            if let Ok(samples) = audio_rx.try_recv() {
                let vol = *volume_clone.lock().unwrap();
                for (i, sample) in data.iter_mut().enumerate() {
                    if i < samples.len() {
                        *sample = samples[i] * vol;
                    } else {
                        *sample = 0.0;
                    }
                }
            } else {
                 // Underrun
                 for s in data.iter_mut() { *s = 0.0; }
            }
        },
        move |err| eprintln!("Audio stream error: {}", err),
        None
    )?;

    stream.play()?;
    
    // Main Decoding Loop
    let mut paused = false;
    state.is_playing.store(true, Ordering::SeqCst);
    let _ = event_tx.send(PlayerEvent::TimeUpdate(0.0));

    loop {
        // Check Commands
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                PlayerCommand::Stop => {
                    return Ok(());
                }
                PlayerCommand::Pause => {
                    paused = true;
                    state.is_playing.store(false, Ordering::SeqCst);
                }
                PlayerCommand::Play => {
                    paused = false;
                    state.is_playing.store(true, Ordering::SeqCst);
                }
                PlayerCommand::Volume(v) => {
                    *state.volume.lock().unwrap() = v;
                }
                PlayerCommand::Seek(_) => {
                    // Seek implementation requires format seeking support (complex)
                    // For now, restarting/skipping is safer or we implement later
                }
            }
        }

        if paused {
            thread::sleep(Duration::from_millis(50));
            continue;
        }

        // Decode Next Packet
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(_)) => {
                let _ = event_tx.send(PlayerEvent::Ended);
                break; 
            }
            Err(_) => break,
        };

        if packet.track_id() != track_id { continue; }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                let duration = decoded.capacity() as u64;
                let mut sample_buffer = SampleBuffer::<f32>::new(duration, spec);
                sample_buffer.copy_interleaved_ref(decoded);
                
                let samples = sample_buffer.samples().to_vec();
                
                // Blocking send to audio thread (backpressure)
                if audio_tx.send(samples).is_err() { break; }

                // Update Progress
                 let ts = packet.ts();
                 let tb = track_params.time_base.unwrap();
                 let t = tb.calc_time(ts).seconds as f64;
                 let _ = event_tx.send(PlayerEvent::TimeUpdate(t));
            }
            Err(_) => break,
        }
    }

    Ok(())
}
