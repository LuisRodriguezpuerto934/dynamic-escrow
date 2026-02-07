# ⚖️ DynamicEscrow

**AI-Powered Arbitration Escrow on Solana**

[![Solana](https://img.shields.io/badge/Solana-Devnet-purple)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.32.1-blue)](https://anchor-lang.com)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 🚀 Overview

DynamicEscrow revolutionizes dispute resolution with AI-powered arbitration. A multi-signature escrow system that automatically resolves disputes fairly using on-chain AI judges.

## ✨ Features

- ✅ **Multi-Sig Escrow** - Multiple parties secure funds
- ✅ **AI Arbitration** - Automated dispute resolution
- ✅ **Time-Locked Releases** - Configurable unlock times
- ✅ **Fair Decisions** - Transparent AI logic
- ✅ **Instant Execution** - Automatic fund distribution

## 📦 Installation

```bash
git clone https://github.com/LuisRodriguezpuerto934/dynamic-escrow.git
cd dynamic-escrow
npm install
anchor build
```

## 🏗️ Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Client    │────▶│   Escrow     │────▶│   AI Judge  │
│             │     │   (PDA)      │     │   (Oracle)  │
└─────────────┘     └──────────────┘     └─────────────┘
                            │
                            ▼
                    ┌──────────────┐
                    │  Freelancer  │
                    └──────────────┘
```

## 📝 Usage

### Create Escrow
```rust
let escrow = create_escrow(
    amount: 500_000_000,        // 500 USDC (6 decimals)
    freelancer: freelancer_key,
    milestones: vec!["Design", "Dev", "Deploy"],
    deadline: 7_days,
)?;
```

### Submit Work
```rust
submit_work(
    escrow,
    milestone: 1,
    evidence: "ipfs_hash",
)?;
```

### Raise Dispute
```rust
raise_dispute(
    escrow,
    reason: "Quality issues",
    evidence: "ipfs_hash",
)?;
```

### AI Resolution
```rust
// Automatic after dispute
let decision = ai_judge.analyze(
    escrow.evidence_client,
    escrow.evidence_freelancer,
);

// Execute decision (70/30 split example)
escrow.distribute(
    client: 350_000_000,      // 70%
    freelancer: 150_000_000,  // 30%
)?;
```

## 📊 Escrow Lifecycle

```
1. CREATE → 2. FUND → 3. WORK → 4. REVIEW → 5. RELEASE
                ↓
            [DISPUTE?]
                ↓
         AI ARBITRATION
                ↓
         AUTO-EXECUTION
```

## 🎯 Use Cases

- **Freelance Platforms** - Secure payments
- **E-commerce** - Buyer protection
- **Service Marketplaces** - Trustless transactions
- **Crowdfunding** - Milestone-based releases

## 🔐 Security Features

- **PDA Authority** - Program-controlled accounts
- **Multi-sig Required** - No single point of failure
- **Time Locks** - Prevent premature releases
- **Oracle Integration** - Trusted AI judges
- **Immutable Decisions** - On-chain transparency

## 📈 Cost Structure

| Action | Cost |
|--------|------|
| Create Escrow | ~0.002 SOL |
| AI Arbitration | ~0.001 SOL |
| Release Funds | ~0.0005 SOL |

## 🛠️ Tech Stack

- **Framework:** Anchor 0.32.1
- **Language:** Rust
- **Oracle:** Custom AI integration
- **Storage:** IPFS for evidence

## 🧪 Testing

```bash
# Unit tests
anchor test

# Integration tests
cargo test --features test-bpf
```

## 📄 Smart Contract

**File:** `programs/dynamic_escrow/src/lib.rs`
**Lines:** 250
**Instructions:**
- `create_escrow`
- `fund_escrow`
- `submit_work`
- `raise_dispute`
- `resolve_dispute`
- `release_funds`

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 License

MIT License

## 👤 Author

**Luis Rodriguez Puerto**
- X: [@BrainTease870](https://x.com/BrainTease870)
- GitHub: [@LuisRodriguezpuerto934](https://github.com/LuisRodriguezpuerto934)

## 🏆 Hackathon

Submitted to **USDC Agent Hackathon 2026**
- Track: Most Novel Smart Contract
- Prize Pool: $30,000 USDC

---

**Fair, Fast, and Fully On-Chain** ⚖️
