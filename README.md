# 🎬 YouTubeTLDR

![Rust](https://img.shields.io/badge/Rust-lang-000000.svg?style=flat&logo=rust)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/milkshiift/YouTubeTLDR/blob/master/LICENSE)

<div align="center">
<h3>⚡ A lightweight, self-hosted YouTube video summarizer with Ollama & Gemini AI<br>
<sub>Demo: <a href="https://tldr.milkshift.dedyn.io/">https://tldr.milkshift.dedyn.io/</a></sub>
</h3>
<img src="/assets/mainScreenshot.png" width="400" alt="New summary page screenshot">
<img src="/assets/summaryScreenshot.png" width="400" alt="Summary screenshot">
</div>

## ✨ Features

*   🤖 **Dual AI Support:** Choose between Ollama (default, self-hosted) or Gemini AI
*   🎯 **Customizable Prompts:** Tailor the AI's instructions to get summaries in the format you prefer
*   ⚙️ **Model Selection:** Choose any available Ollama or Gemini model
*   📝 **View Transcript:** Access the full, raw video transcript
*   📚 **History:** Your summaries are saved locally in your browser for future reference
*   🔒 **Privacy-Focused:** Simple Rust server that runs on your own machine. Your data stays yours
*   🎨 **Modern UI:** Clean and beautiful user interface

## 🏗️ Philosophy: Minimal by Design

YouTubeTLDR embraces simplicity — maximum functionality with minimal overhead.

*   🪶 **Featherweight & Zero Bloat:** Single binary ~**0.3MB**. No databases, no Tokio, no frameworks
*   ⚡ **Lightning Fast:** Pure Rust + vanilla HTML/JS
*   🔑 **BYOK:** Bring Your Own Key. Use your own Ollama server or Gemini API — no proxies, no data collection
*   🎯 **Single Purpose:** Just generates and saves summaries, that's it

Note: This server is optimized for personal use and utilizes a multithreaded worker pool for concurrency. It is not designed to support hundreds of concurrent users.

## 🚀 Getting Started

### Prerequisites

*   **Option 1 (Default):** [Ollama](https://ollama.com/) installed locally or accessible on your network
*   **Option 2:** A [Google Gemini API Key](https://aistudio.google.com/app/apikey) (Free tier with generous limits)

### Running the Application

1.  Download the [latest release](https://github.com/Milkshiift/YouTubeTLDR/releases/latest) and run the executable from console:
    ```bash
    ./YouTubeTLDR
    ```
2.  Open `http://localhost:8000` in your browser
3.  Select your AI provider (Ollama or Gemini) in Settings
4.  For Gemini: Enter your API key in Settings. Optional for Ollama.
5.  Paste a YouTube URL and click "Summarize"

### Environment Variables

*   `TLDR_IP` - Server IP address (default: `0.0.0.0`)
*   `TLDR_PORT` - Server port (default: `8000`)
*   `TLDR_WORKERS` - Number of worker threads (default: `4`)
*   `OLLAMA_URL` - Ollama server URL (default: `http://localhost:11434`)

**Example:**
```bash
export OLLAMA_URL="http://192.168.1.100:11434"
./YouTubeTLDR
```

## 🔨 Building from Source

1.  Install the **nightly** [Rust toolchain](https://www.rust-lang.org/tools/install)
2.  Clone the repository:
    ```bash
    git clone https://github.com/Milkshiift/YouTubeTLDR.git
    cd YouTubeTLDR
    ```
3.  Build the release binary:
    ```bash
    cargo build --release
    ```
4.  Find your executable at `target/release/YouTubeTLDR`

By default, the native TLS implementation (like openssl) is used. If you want to use rustls build with `--no-default-features --features rustls-tls`