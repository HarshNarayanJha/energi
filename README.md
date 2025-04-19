# Energi

Energi shows you what's really going on with your laptop battery. I built it because I was tired of not knowing why my battery was draining so fast. It puts all the important stuff—charge levels, power usage, and how your battery performs over time—right in front of you with simple charts that actually make sense.

All major desktop environments have a energy manager type of thing (like powerdevil on KDE) that shows a nice graph for battery usage.
There was nothing like this on Hyprland and other non DE setups, so I made this!

With Energi, you can:

- View current battery percentage and estimated time remaining
- Monitor power draw in watts
- See historical usage patterns through interactive charts
- Get notifications about critical battery levels
- Track battery health and capacity degradation over time
- Optimize power usage with detailed consumption analytics

## Installation

Currently no builds are available for download. You can build Energi from source by following the instructions below.

### Setup Development Environment

1. Clone the repository:

```bash
git clone https://github.com/HarshNarayanJha/energi.git
cd energi
```

2. Make sure you have the Rust toolchain installed. If not, install it using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install Dependencies:

```bash
bun install
```

### Serving Energi

Run the following command in the root of the project to start developing with the default platform:

```bash
bun run tauri dev
```
