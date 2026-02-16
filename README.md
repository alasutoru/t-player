# T-Player

A minimalist, high-performance media player built with Rust. Designed for those who appreciate the purity of binary executables.

## Philosophy

In an era of bloated software and endless dependencies, T-Player stands for simplicity and control.

I believe in the power of **Binary-First** software:
- **Zero Dependencies**: No external runtimes, no complex setups. Just download and run.
- **Performance**: Native code execution, optimized for speed and low resource usage.
- **Portability**: A single executable file that works anywhere.

This project is an exploration of building robust, standalone desktop applications using modern systems programming languages.

## Features

- **Pure Rust Audio Engine**: Custom-built audio playback using `symphonia` for decoding and `cpal` for cross-platform audio output. No heavy media frameworks required.
- **Local Video Streaming**: Integrated `axum` HTTP server that streams video content directly to the UI, ensuring smooth playback without external players.
- **Drag & Drop Interface**: Simple, intuitive user experience. Just drop your media file, and it plays.
- **Highly Optimized**: Built with Link Time Optimization (LTO) and binary stripping for maximum efficiency and minimal footprint.

## Installation

### Download Binaries
Go to the [Releases](https://github.com/alasutoru/t-player/releases) page and download the latest version for your operating system:
- **macOS**: `t-player`

### Build from Source
If you prefer to build it yourself, ensure you have Rust installed.

```bash
git clone https://github.com/alasutoru/t-player.git
cd t-player
cargo build --release
```
The optimized binary will be located in `target/release/`.

## Usage

1. Launch the application.
2. Drag and drop any audio (`.mp3`, `.flac`, `.wav`, `.ogg`) or video file into the window.
3. Enjoy your media.

## Tech Stack

- **Language**: Rust 🦀
- **UI Framework**: Dioxus (Desktop)
- **Audio Backend**: Symphonia (Decoding) + CPAL (Output)
- **Video Server**: Axum + Tokio
- **Async Runtime**: Tokio

## License

This project is open-sourced under the MIT License.
