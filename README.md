
# 🚀 Bonk.fun Sniper Bot — Helius Laserstream gRPC (Solana)

A high-speed **Bonk.fun Sniper Bot** powered by **Helius Laserstream (gRPC)** for real-time Solana transaction streaming and instant token sniping.

Built for ultra-low-latency detection and automated buys/sells on **Bonk.fun** token launches.

---

## ✨ Features
- ⚡ **Real-time transaction stream** via **Helius Laserstream (gRPC)**
- 🎯 Auto-detects early Bonk.fun token launches (bundle-based mints)
- 🤖 Fully automated snipe execution
- 🪙 Customizable buy/sell logic & anti-rug filters
- 🔒 Secure private key usage (no key exposure)
- 📊 Transaction metrics & logs
- 🧩 Modular architecture for extending new heuristics

---

## 🛠 Tech Stack
- **Rust** — Core bot logic & performance
- **Helius Laserstream gRPC** — Live transaction feeds
- **Solana SDK** — Transaction signing & simulation
- **Yellowstone gRPC** (Optional for alternative stream)
- **Config.toml** — Easy environment configuration

---

## 📦 Installation

### 1. Clone the Repo
```bash
git clone https://github.com/vvizardev/bonkfun-sniper-grpc.git
cd bonkfun-sniper-grpc
```

### 2. Setup Environment Variables
Create a `.env` file in the root directory:
```env
HELIUS_API_KEY=your_helius_api_key
PRIVATE_KEY=your_private_key_base58
RPC_URL=https://api.mainnet-beta.solana.com
```

### 3. Install Dependencies
```bash
cargo build --release
```

---

## 🚀 Usage

### Run the Sniper
```bash
cargo run --release
```

### What it does:
1. Connects to Helius Laserstream (gRPC)
2. Monitors Solana transaction bundles related to Bonk.fun
3. Detects launch conditions based on heuristics (liquidity added, bundle triggers, etc.)
4. Executes buy/sell transactions instantly.

---

## 🔧 Configuration Options

You can modify `config.toml` for parameters:
```toml
[snipe]
profit_target = 1.5      # 50% profit target
stop_loss = 0.7          # Stop loss threshold
bundle_detect_threshold = 3  # Wallets in bundle before sniping

[helius]
grpc_url = "grpc.helius.xyz"
```

---

## 🧠 Heuristics Logic (Pluggable)
- **Bundle Pattern Detection**: Wallets funding each other & buying same token.
- **Liquidity Add Detection**: New LP pools in a bundle.
- **Blacklist & Honeypot Check** (Optional extension).
- **Bonk.fun ID detection** (for Bonk.fun tokens specifically).

---

## ⚠️ Disclaimer
> **This bot interacts with live financial markets. Use at your own risk.**
> The authors are not responsible for any financial losses. Ensure compliance with local regulations.

---

## 💡 TODO
- [ ] Add multi-chain support (Sonic, Neon)
- [ ] UI Dashboard (WebSocket-based)
- [ ] Rug detection heuristics
- [ ] Multi-wallet load balancing

---

## 📄 License
MIT License

---

## 🤝 Credits
- [Helius Labs](https://helius.xyz)
- [Solana Labs](https://solana.com)
- [Bonk.fun](https://bonk.fun)
- [Yellowstone gRPC](https://github.com/yellowstone-grpc)
