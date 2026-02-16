# T-Player

A minimalist, high-performance media player built with Rust. Designed for those who appreciate the purity of binary executables.

一個使用 Rust 構建的極簡、高性能媒體播放器。專為熱愛二進位執行檔純粹性的人士設計。

---

## Philosophy / 專案哲學

### English
In an era of bloated software and endless dependencies, T-Player stands for simplicity and control. I believe in the power of **Binary-First** software:
- **Zero Dependencies**: No external runtimes, no complex setups. Just download and run.
- **Performance**: Native code execution, optimized for speed and low resource usage.
- **Portability**: A single executable file that works anywhere.

### 繁體中文
在軟體肥大化與依賴無窮無盡的時代，T-Player 代表了極簡與掌控。我深信 **「二進位至上 (Binary-First)」** 軟體的力量：
- **零依賴**：無需外部執行環境，無需複雜安裝。下載即可執行。
- **高性能**：原生代碼執行，針對速度與低資源消耗進行優化。
- **可攜性**：單一執行檔，隨處運行。

---

## Features / 功能特點

- **Pure Rust Audio Engine**: Custom-built audio playback using `symphonia` for decoding and `cpal` for cross-platform audio output.
  - **純 Rust 音訊引擎**：使用 `symphonia` 解碼與 `cpal` 進行跨平台音訊輸出，不依賴重型媒體框架。
- **Local Video Streaming**: Integrated `axum` HTTP server that streams video content directly to the UI.
  - **本地影片串流**：內建 `axum` HTTP 伺服器，將影片內容直接串流至 UI，確保流暢播放。
- **Drag & Drop Interface**: Simple, intuitive user experience. Just drop your media file, and it plays.
  - **拖放式介面**：簡單直觀的用戶體驗。只需拖入媒體檔案即可開始播放。
- **Highly Optimized**: Built with Link Time Optimization (LTO) and binary stripping.
  - **極致優化**：透過 LTO 連結時間優化與二進位符號移除，實現最大效能與最小體積。

---

## Installation / 安裝說明

### Download Binaries / 下載執行檔
Go to the [Releases](https://github.com/alasutoru/t-player/releases) page and download the latest version for macOS:
- **macOS**: `t-player`

請前往 [發佈頁面 (Releases)](https://github.com/alasutoru/t-player/releases) 下載適用於 macOS 的最新版本。

### Build from Source / 從原始碼編譯
If you prefer to build it yourself, ensure you have Rust installed.
如果您希望自行編譯，請確保已安裝 Rust。

```bash
git clone https://github.com/alasutoru/t-player.git
cd t-player
cargo build --release
```

---

## Usage / 使用方法

1. Launch the application. / 啟動應用程式。
2. Drag and drop any audio (`.mp3`, `.flac`, `.wav`, `.ogg`) or video file into the window. / 將任何音訊或影片檔案拖放入視窗中。
3. Enjoy your media. / 開始享受您的媒體內容。

## Tech Stack / 技術堆疊

- **Language**: Rust 🦀
- **UI Framework**: Dioxus (Desktop)
- **Audio Backend**: Symphonia (Decoding) + CPAL (Output)
- **Video Server**: Axum + Tokio
- **Async Runtime**: Tokio

## License / 授權

This project is open-sourced under the MIT License.
本專案依據 MIT 授權條款開放原始碼。
