# Mafia NFT - Solana Blockchain RPG

A free-to-play/play-to-earn mafia-themed RPG built on Solana mainnet-beta. Players form syndicates, seize turf, run illegal businesses, and vote on large-scale heists in a gritty neo-noir crypto metropolis.

## 🎮 Game Overview

**Genre**: Mafia-themed RPG with DAO governance  
**Platform**: PC + Mobile  
**Blockchain**: Solana mainnet-beta  
**Inspiration**: SYN CITY's "Mafia-as-a-DAO" concept  

### Core Gameplay Loop
1. **Mission** - Complete daily tasks and heists
2. **Loot** - Earn rewards and NFT drops
3. **Launder** - Convert earnings through businesses
4. **Upgrade** - Enhance characters and equipment
5. **Turf Defense** - Protect territory and expand influence

### Progression Tiers
- **Street Runner** - Starting tier, basic missions
- **Capo** - Mid-tier, access to crew management
- **Underboss** - Advanced tier, district operations
- **Boss** - Elite tier, unlocks new districts and DAO governance

## 🏗️ Technical Architecture

### Blockchain Layer (Solana)
- **Smart Contracts**: Rust + Anchor v0.29
- **NFTs**: Metaplex Certified Collections
- **Tokens**: SPL Token 2022 standard
- **Governance**: Solana Realms integration

### Backend Services
- **API**: Node.js + TypeScript + NestJS
- **Database**: PostgreSQL + Redis
- **Message Queue**: NATS for event processing
- **RPC**: Helius/QuickNode integration

### Game Client
- **Engine**: Unity URP
- **Wallet**: Solana Wallet Adapter (Phantom, Backpack, Glow, Solflare)
- **Networking**: gRPC/WebSocket for real-time gameplay

## 📁 Project Structure

```
mafia-nft/
├── programs/           # Anchor smart contracts
├── app/               # Unity game client
├── backend/           # Node.js API services
├── docs/              # Game design and technical documentation
├── tests/             # Integration and unit tests
├── scripts/           # Deployment and utility scripts
└── assets/            # Game assets and NFT metadata
```

## 🎯 Development Roadmap

### Sprint 1-2: Foundation (6 weeks)
- [ ] Core Anchor programs development
- [ ] NFT collection setup and metadata pipeline
- [ ] Basic Unity client with wallet integration

### Sprint 3-4: Economy & Gameplay (7 weeks)
- [ ] Token economics implementation
- [ ] Game mechanics and progression system
- [ ] PvP and PvE systems

### Sprint 5-6: DAO & Polish (8 weeks)
- [ ] Realms governance integration
- [ ] Security audits and testing
- [ ] Beta testing and community feedback

## 🔐 Security & Compliance

- **Audits**: OtterSec, Sec3, Neodyme
- **Monitoring**: SolanaFM & Helius Webhooks
- **Compliance**: GEO IP + KYC for fiat off-ramps
- **Rate Limiting**: Anti-bot measures and compute budget protection

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Solana CLI 1.16+
- Anchor CLI 0.29+
- Node.js 18+
- Unity 2022.3 LTS

### Quick Start
```bash
# Clone repository
git clone https://github.com/richardzon/mafianft.git
cd mafianft

# Install dependencies
npm install

# Build Anchor programs
anchor build

# Run tests
anchor test

# Start local validator
solana-test-validator

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## 📊 Token Economics

### $MOB Token (Utility)
- **Type**: SPL Token (inflationary)
- **Use Cases**: Weapon mods, bribes, respawns, marketplace fees
- **Emission**: Balanced for 60-90 day ROI

### $FAM Token (Governance)
- **Type**: SPL Token (fixed supply)
- **Use Cases**: DAO voting, staking rewards, marketplace fee sharing
- **Distribution**: Community-driven governance

## 🎨 NFT Collections

| Category | Supply | Standard | Utility | Rarity |
|----------|--------|----------|---------|--------|
| Characters | 10,000 | Metaplex NFT | Avatar + stats | C-M-E-L-M |
| Weapons | Open | SPL Token 2022 | Attack mods | N-R-L |
| Vehicles | 5,000 | Metaplex NFT | Speed/crew | R-L-M |
| Turf Deeds | 2,500 | Metaplex NFT | Income + DAO | Unique |

## 🏛️ DAO Governance

Each Turf Deed NFT creates a Family DAO with:
- **Boss**: Proposal creation rights
- **Consigliere**: Veto power
- **Capos**: Multisig execution
- **Soldiers**: Voting weight based on staked NFTs

## 📈 Analytics & Monitoring

- **User Analytics**: Amplitude/PostHog integration
- **Economy Monitoring**: $MOB velocity tracking
- **A/B Testing**: On-chain config PDAs for tamper-evident parameters

## 🤝 Contributing

Please read our [Contributing Guidelines](./docs/CONTRIBUTING.md) and [Code of Conduct](./docs/CODE_OF_CONDUCT.md).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Game Design Document](./docs/game-design.md)
- [Technical Specifications](./docs/technical-specs.md)
- [API Documentation](./docs/api.md)
- [Smart Contract Documentation](./docs/contracts.md)
