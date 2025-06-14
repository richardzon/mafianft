# 🚀 Mafia NFT Deployment Guide

## Prerequisites

### 1. Install Required Tools

#### Solana CLI
```bash
# Install Solana CLI
curl -sSfL https://release.solana.com/stable/install | sh

# Add to PATH
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Verify installation
solana --version
```

#### Anchor CLI
```bash
# Install Anchor CLI
npm install -g @coral-xyz/anchor-cli@0.29.0

# Verify installation
anchor --version
```

#### Node.js Dependencies
```bash
# Install project dependencies
npm install
```

### 2. Setup Solana Wallet

```bash
# Generate new keypair (if you don't have one)
solana-keygen new --outfile ~/.config/solana/id.json

# Or use existing keypair
# cp your-keypair.json ~/.config/solana/id.json

# Check wallet address
solana address
```

## Deployment Process

### Step 1: Deploy Smart Contracts to Devnet

```bash
# Run the automated deployment script
./scripts/deploy-to-devnet.sh
```

This script will:
- ✅ Check Solana and Anchor CLI installation
- ✅ Configure Solana for devnet
- ✅ Request SOL airdrop if needed
- ✅ Build all Anchor programs
- ✅ Deploy programs to devnet
- ✅ Update program IDs in configuration files
- ✅ Run basic deployment tests
- ✅ Generate deployment report

### Step 2: Initialize Programs

```bash
# Initialize all deployed programs
node scripts/initialize-programs.js devnet
```

This will:
- ✅ Create token mints (MOB, FAM)
- ✅ Setup NFT collections (Characters, Weapons, Vehicles, Territories)
- ✅ Configure program parameters
- ✅ Generate initialization report

### Step 3: Generate NFT Assets

```bash
# Generate character NFTs (start with 1000 for testing)
node scripts/generate-ai-art.js --count 1000 --batch 50

# For full collection (10,000 characters)
node scripts/generate-ai-art.js --count 10000 --batch 100
```

This will:
- ✅ Generate unique character combinations
- ✅ Create metadata JSON files
- ✅ Apply rarity distribution
- ✅ Generate collection statistics

### Step 4: Test Deployment

```bash
# Run Anchor tests
anchor test --skip-local-validator

# Test specific functionality
npm run test:character-mint
npm run test:token-operations
npm run test:territory-system
```

### Step 5: Deploy Backend API

```bash
# Navigate to backend directory
cd backend

# Install dependencies
npm install

# Setup environment variables
cp .env.example .env.devnet

# Update .env.devnet with deployed program IDs
# (These will be automatically updated by the deployment script)

# Run database migrations
npm run migration:run

# Start development server
npm run start:dev

# Or deploy to staging
npm run deploy:staging
```

### Step 6: Setup Unity Client

```bash
# Setup Unity project structure
node scripts/setup-unity-simple.js

# The deployment script automatically updates Constants.cs
# with the correct program IDs
```

Then in Unity:
1. Open Unity Hub
2. Add the `app/` folder as a project
3. Install Solana Unity SDK
4. Configure wallet adapters
5. Test wallet connection

## Verification Steps

### 1. Verify Smart Contract Deployment

```bash
# Check each program is deployed
solana account <CHARACTER_NFT_PROGRAM_ID>
solana account <MOB_TOKEN_PROGRAM_ID>
solana account <FAM_TOKEN_PROGRAM_ID>
solana account <ITEM_VAULT_PROGRAM_ID>
solana account <TURF_CONTROL_PROGRAM_ID>
solana account <GAME_TREASURY_PROGRAM_ID>

# View on Solana Explorer
# https://explorer.solana.com/address/<PROGRAM_ID>?cluster=devnet
```

### 2. Test Core Functionality

```bash
# Test character minting
anchor run test-character-mint

# Test token operations
anchor run test-token-ops

# Test territory system
anchor run test-territory
```

### 3. Verify Backend API

```bash
# Health check
curl http://localhost:3000/health

# Test authentication
curl -X POST http://localhost:3000/api/v1/auth/nonce \
  -H "Content-Type: application/json" \
  -d '{"walletAddress": "YOUR_WALLET_ADDRESS"}'
```

### 4. Test Unity Client

1. Open Unity project
2. Enter Play Mode
3. Test wallet connection
4. Verify program ID constants are correct
5. Test basic UI functionality

## Mainnet Deployment

⚠️ **Only deploy to mainnet after thorough testing on devnet**

### 1. Prepare for Mainnet

```bash
# Switch to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Ensure you have sufficient SOL for deployment
solana balance

# If needed, transfer SOL to your deployment wallet
```

### 2. Deploy to Mainnet

```bash
# Deploy smart contracts
anchor deploy --provider.cluster mainnet

# Initialize programs
node scripts/initialize-programs.js mainnet

# Update all configuration files with mainnet program IDs
```

### 3. Production Setup

```bash
# Deploy backend to production
npm run deploy:production

# Build Unity client for production
# (Follow Unity build instructions in app/README.md)

# Upload NFT assets to permanent storage (IPFS/Arweave)
# Update metadata URIs
```

## Troubleshooting

### Common Issues

#### 1. Insufficient SOL Balance
```bash
# Check balance
solana balance

# Request airdrop (devnet only)
solana airdrop 2

# For mainnet, transfer SOL from exchange
```

#### 2. Program Deployment Fails
```bash
# Check program size (max 10MB)
ls -lh target/deploy/*.so

# Increase compute budget if needed
# (This is handled automatically in our programs)

# Check for duplicate program IDs
anchor keys list
```

#### 3. Anchor Build Errors
```bash
# Clean and rebuild
anchor clean
anchor build

# Check Rust version
rustc --version

# Update if needed
rustup update
```

#### 4. Node.js Dependency Issues
```bash
# Clear npm cache
npm cache clean --force

# Delete node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Getting Help

- **Discord**: https://discord.gg/mafianft
- **GitHub Issues**: https://github.com/richardzon/mafianft/issues
- **Documentation**: https://docs.mafianft.com
- **Solana Docs**: https://docs.solana.com

## File Structure After Deployment

```
mafianft/
├── target/deploy/                 # Compiled programs
├── deployed-program-ids.json      # Program IDs from deployment
├── character-collection-mint.json # Character collection mint
├── mob-token-mint.json            # MOB token mint
├── fam-token-mint.json            # FAM token mint
├── item-collections.json          # Weapon/vehicle collections
├── turf-collection.json           # Territory collection
├── deployment-report-*.md         # Deployment reports
├── initialization-summary-*.json  # Initialization summaries
├── assets/generated/              # Generated NFT assets
└── assets/metadata/               # NFT metadata files
```

## Security Considerations

### 1. Wallet Security
- ✅ Use hardware wallet for mainnet deployments
- ✅ Keep private keys secure and backed up
- ✅ Use different wallets for development and production

### 2. Program Security
- ✅ All programs include ownership checks
- ✅ Rate limiting implemented for anti-bot protection
- ✅ Input validation on all instructions
- ✅ PDA (Program Derived Address) security

### 3. Economic Security
- ✅ Token emission controls
- ✅ Maximum supply limits
- ✅ Burn mechanisms for sustainability
- ✅ Treasury management with multi-sig (recommended for mainnet)

## Next Steps After Deployment

1. **Community Building**
   - Launch Discord server
   - Start social media campaigns
   - Engage with Solana NFT communities

2. **Marketing**
   - Create demo videos
   - Write technical blog posts
   - Partner with other Solana projects

3. **Feature Development**
   - Implement advanced game mechanics
   - Add social features
   - Develop mobile apps

4. **Governance**
   - Activate DAO functionality
   - Enable community proposals
   - Transition to community control

🎉 **Congratulations! Your Mafia NFT project is now deployed and ready for users!**
