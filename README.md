# 🖥️ Macro Recorder (Global Input Event Logger)

A simple cross-platform global input recorder written in Rust using the [`rdev`](https://crates.io/crates/rdev) crate.

This tool captures and logs keyboard and mouse events along with timestamps, until you press `ESC` to stop.

## ✨ Features

- Records:
  - Keyboard press and release events
  - Mouse button press and release events
  - Mouse movements
- Timestamped events
- Stops recording when `Escape` key is pressed
- Thread-safe with `Arc<Mutex<>>` and `AtomicBool`

## 📸 Example Output



## 🚀 Usage

### 1. Clone the repository

```bash
git clone https://github.com/kant-github/macro-recorder.git
cd macro_recorder
cargo run
