# AutoClickerRS
A simple auto-clicker written in Rust that automatically clicks at a selected screen position at specified time intervals. The program listens for a left mouse click to set the click point and the `ESC` key to exit.

---

## Features

- Listens for left mouse click to set the auto-click point.
- Automatically clicks at the specified point at given intervals (in seconds).
- Allows stopping the program anytime by pressing `ESC`.
- Uses the `enigo` crate to control the mouse.
- Captures global mouse and keyboard events via `rdev`.

---

## Requirements

- Rust (recommended version 1.65 or newer)
- Operating system supported by `enigo` and `rdev` crates (e.g., Windows, Linux, macOS)

---

## Installation and Running

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/rust-autoclicker.git
   cd rust-autoclicker

2. Build the project:

```bash
cargo build --release
```

3. Run the program with the required interval argument (seconds):
```bash
cargo run --release -- --interval_time 2
```
Replace 2 with your desired click interval in seconds

## Usage

1. After starting, the program will print:
```bash
Please left-click to set the click point.
Press ESC to quit the program at any time.
```

2. Left-click anywhere on your screen where you want automatic clicks to occur.
3. The program will start clicking at that position every specified interval.
4. Press the ESC key anytime to stop and exit the program.

## Example

Run with 1-second intervals:
```bash
cargo run --release -- --interval_time 1
```


## Project Structure

- main.rs — program entry point, argument parsing, and auto-clicker initialization.
- autoclicker.rs — core auto-clicker logic including event listening and click loop.
- config.rs — parsing configuration from command-line arguments.
- mousecontroller.rs — abstraction for mouse control and Enigo-based implementation.

## Dependencies

- enigo — for mouse and keyboard control.
- rdev — for global mouse and keyboard event listening.
