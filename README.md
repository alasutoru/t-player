# T-Player

<p align="center">
  <a href="#english">English</a> | <a href="#traditional-chinese">繁體中文</a>
</p>

---

<h1 id="english">English</h1>

A minimalist, high-performance media player built with Rust. Designed for those who appreciate the purity of binary executables.

## Philosophy

In an era of bloated software and endless dependencies, T-Player stands for simplicity and control. I believe in the power of **Binary-First** software:
- **Zero Dependencies**: No external runtimes, no complex setups. Just download and run.
- **Performance**: Native code execution, optimized for speed and low resource usage.
- **Portability**: A single executable file that works anywhere.

## Features

- **Pure Rust Audio Engine**: Custom-built audio playback using `symphonia` for decoding and `cpal` for cross-platform audio output.
- **Local Video Streaming**: Integrated `axum` HTTP server that streams video content directly to the UI.
- **Drag & Drop Interface**: Simple, intuitive user experience. Just drop your media file, and it plays.
- **Highly Optimized**: Built with Link Time Optimization (LTO) and binary stripping.

## Installation

### Download Binaries
Go to the [Releases](https://github.com/alasutoru/t-player/releases) page and download the latest version for macOS:
- **macOS**: `t-player`

### Build from Source
```bash
git clone https://github.com/alasutoru/t-player.git
cd t-player
cargo build --release
```

## Usage
1. Launch the application.
2. Drag and drop any audio (`.mp3`, `.flac`, `.wav`, `.ogg`) or video file into the window.
3. Enjoy your media.

[Back to Top](#t-player)

---

<h1 id="traditional-chinese">繁體中文</h1>

一個使用 Rust 構建的極簡、高性能媒體播放器。專為熱愛二進位執行檔純粹性的人士設計。

## 專案哲學

在軟體肥大化與依賴無窮無盡的時代，T-Player 代表了極簡與掌控。我深信 **「二進位至上 (Binary-First)」** 軟體的力量：
- **零依賴**：無需外部執行環境，無需複雜安裝。下載即可執行。
- **高性能**：原生代碼執行，針對速度與低資源消耗進行優化。
- **可攜性**：單一執行檔，隨處運行。

## 功能特點

- **純 Rust 音訊引擎**：使用 `symphonia` 解碼與 `cpal` 進行跨平台音訊輸出，不依賴重型媒體框架。
- **本地影片串流**：內建 `axum` HTTP 伺服器，將影片內容直接串流至 UI，確保流暢播放。
- **拖放式介面**：簡單直觀的用戶體驗。只需拖入媒體檔案即可開始播放。
- **極致優化**：透過 LTO 連結時間優化與二進位符號移除，實現最大效能與最小體積。

## 安裝說明

### 下載執行檔
請前往 [發佈頁面 (Releases)](https://github.com/alasutoru/t-player/releases) 下載適用於 macOS 的最新版本：
- **macOS**: `t-player`

### 從原始碼編譯
如果您希望自行編譯，請確保已安裝 Rust。
```bash
git clone https://github.com/alasutoru/t-player.git
cd t-player
cargo build --release
```

## 使用方法
1. 啟動應用程式。
2. 將任何音訊 (`.mp3`, `.flac`, `.wav`, `.ogg`) 或影片檔案拖放入視窗中。
3. 開始享受您的媒體內容。

## 技術堆疊 / Tech Stack
- **Language**: Rust 🦀
- **UI Framework**: Dioxus (Desktop)
- **Audio Backend**: Symphonia (Decoding) + CPAL (Output)
- **Video Server**: Axum + Tokio

## 授權 / License
本專案依據 MIT 授權條款開放原始碼。

[回到頂部](#t-player)
