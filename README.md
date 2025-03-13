![rustc](https://img.shields.io/badge/rustc-1.85.0-blue.svg)
[![codecov](https://codecov.io/gh/averageeucplayer/lost-metrics-data/graph/badge.svg?token=HHRGYYUNM2)](https://codecov.io/gh/averageeucplayer/lost-metrics-data)

# 📖 Lost Metrics Data  

This crate provides a set of utilities to load and manage Lost Ark game data that has been datamined. It parses various game data files (in JSON format) and stores them in Rust's static collections for efficient, on-demand access.

The data is lazily loaded at runtime, meaning it is only read and parsed the first time it is accessed, ensuring minimal overhead.

## 📦 Installation & Setup

### 1️⃣ **Clone the Repository**

```sh
git clone https://github.com/averageeucplayer/lost-metrics-data.git
```

### 2️⃣ Add to Cargo.toml

```toml
[dependencies]
lost-metrics-data = { git = "https://github.com/averageeucplayer/lost-metrics-data" }
lost-metrics-data = { git = "https://github.com/averageeucplayer/lost-metrics-data", branch="main" }
lost-metrics-data = { git = "https://github.com/averageeucplayer/lost-metrics-data", tag="v1.0.0" }
```

## ⚡ Features

This crate supports two data formats for parsing game data:

- `json` (default) – Uses serde_json for parsing standard JSON files.
- `msgpack` – Uses rmp-serde for MessagePack format, which reduces asset size.

```
lost-metrics-data = { git = "https://github.com/averageeucplayer/lost-metrics-data", features=["msgpack"] }
```