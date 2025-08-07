# ⚡️ Solana DEX Router

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Solana](https://img.shields.io/badge/solana-mainnet--beta-success)](https://solana.com)
[![Anchor](https://img.shields.io/badge/anchor-0.29.0-blue)](https://book.anchor-lang.com)

A mid-level Solana smart contract written in **Rust + Anchor** that simulates weighted AMM pools and routing logic. It includes:

- Weighted constant-product swap logic (Balancer-style)
- Real **SPL token integration** via **CPI**
- PDA-managed vaults for secure token custody
- Weighted pool math using fixed-point arithmetic (`u128`)
- Multi-hop routing-ready architecture
- End-to-end Anchor tests with minting, ATAs, and swaps

---

## 🧠 Overview

This protocol simulates the routing engine of a DEX aggregator like **Jupiter** or **Stabble**, using weighted pools and SPL token CPIs to simulate token swaps between two tokens with arbitrary weights (e.g., 80/20 or 60/40 pools).

---

## 📦 Features

✅ Weighted pool routing using: \
`amountOut = y * (1 - (x / (x + amountIn)) ^ (wx / wy))`

✅ Vaults managed by PDA-controlled accounts  
✅ Real SPL token support (via CPI to token program)  
✅ Fully tested using dynamic token minting  
✅ Written in Rust using Anchor framework  

---

## 🛠️ Tech Stack

- 🔁 **Solana Program Library (SPL)**
- 🧵 **Anchor** framework for Solana development
- 🧠 **Fixed-point math** using `u128` precision
- 🧪 **Anchor Mocha tests** (TypeScript)
- 🪙 **SPL Token integration** for real swaps

---

## 📁 Directory Structure

```
├── programs/
│ └── dex_router/
│ ├── src/
│ │ ├── lib.rs # Anchor entrypoint
│ │ ├── context.rs # Account constraints
│ │ ├── errors.rs # Errors
│ │ ├── state.rs # Pool struct and constants
│ │ ├── processor.rs # Instruction logic
│ │ └── math.rs # Fixed-point weighted pool math
├── tests/
│ └── dexrouter.ts # Minting, pool init, weighted swap
├── Anchor.toml
├── Cargo.toml
└── README.md
```

---

## 🚀 Local Setup

```bash
# Clone the repo
git clone https://github.com/tdadadavid/DEXRouter.git
cd DEXRouter

# Install dependencies
anchor build
anchor test
```

---

## 🧪 Tests

Run full tests (mint tokens, setup ATAs, initialize pool, swap):

anchor test

Each test:

- Creates two SPL mints
- Mints tokens to the payer
- Initializes a pool with 50/50 weights
- Executes a CPI-based swap from token A → token B

---

## 📐 Math Model

We implement weighted swap math from the Balancer V1 model:

`
amountOut = tokenOutBalance * (1 - (tokenIn / (tokenIn + amountIn)) ^ (weightIn / weightOut))
`

Implemented via a custom pow_fixed() helper using u128 fixed-point math with 18 decimal precision. Supports stable and safe execution in BPF-constrained environments.

---

## 🔐 Security Notes

- Vault accounts are owned by a deterministic PDA (pool_signer)
- Users only interact with program-controlled SPL token vaults
- Token movements are done via secure CPI to the spl-token program

---

## 🧠 Inspiration

This project is inspired by the routing engines and pool math behind:

- Balancer
- Jupiter Aggregator
- Stabble Protocol
