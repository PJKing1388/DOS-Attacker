# 🚀 DOS-Attacker ⚡

A high-performance Rust-based DOS-Attacker tool for **authorized** server resilience testing.

## 🌟 Features
- 🌀 Configurable request flooding
- 🔗 Keep-alive connection testing
- 📊 Success/error/timeout tracking
- 🎛️ Concurrent request control
- ⏱️ Randomized delays
- 📈 Detailed statistics

## ⚠️ Legal Notice
**🚨 Warning :** Only use this tool on systems you own or have **explicit written permission** to test. Unauthorized use may violate computer crime laws in your jurisdiction.

## 🛠️ Installation

### 📦 Prerequisites
- Rust 1.60+ (install via [rustup](https://rustup.rs/)) 🦀
- Linux/MacOS (Windows WSL2 recommended) 💻
- System limits increase :

```bash
ulimit -n 100000  # 📈 Increase file descriptors
