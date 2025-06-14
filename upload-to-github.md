# 🚀 Complete Mafia NFT Project Upload Guide

## Step 1: Clone Your Empty Repository

```bash
git clone https://github.com/richardzon/mafianft.git
cd mafianft
```

## Step 2: Create All Project Files

### Core Configuration Files

**package.json:**
```json
{
  "name": "mafianft",
  "version": "1.0.0",
  "description": "Solana-based mafia-themed RPG with NFTs, tokens, and DAO governance",
  "main": "index.js",
  "scripts": {
    "test": "anchor test",
    "build": "anchor build",
    "deploy:devnet": "./scripts/deploy-to-devnet.sh",
    "deploy:mainnet": "anchor deploy --provider.cluster mainnet",
    "generate-art": "node scripts/generate-ai-art.js",
    "validate": "node scripts/validate-deployment-ready.js"
  },
  "keywords": ["solana", "nft", "gaming", "defi", "dao"],
  "author": "richardzon",
  "license": "MIT",
  "dependencies": {
    "@coral-xyz/anchor": "^0.29.0",
    "@solana/web3.js": "^1.87.6",
    "@solana/spl-token": "^0.3.9"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "typescript": "^5.0.0"
  }
}
```

**Anchor.toml:**
```toml
[features]
seeds = false
skip-lint = false

[programs.localnet]
character_nft = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
item_vault = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT"
turf_control = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnU"
mob_token = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnV"
fam_token = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnW"
game_treasury = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnX"

[programs.devnet]
character_nft = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
item_vault = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT"
turf_control = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnU"
mob_token = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnV"
fam_token = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnW"
game_treasury = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnX"

[programs.mainnet]
character_nft = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
item_vault = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT"
turf_control = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnU"
mob_token = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnV"
fam_token = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnW"
game_treasury = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnX"

[workspace]
members = [
    "programs/character-nft",
    "programs/item-vault", 
    "programs/turf-control",
    "programs/mob-token",
    "programs/fam-token",
    "programs/game-treasury"
]

[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"

[test.validator]
url = "https://api.devnet.solana.com"
clone = [
    { address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" },
    { address = "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY" },
    { address = "noopb9bkMVfRPU8AQkHtKwMYZiFUjNRtMmV" },
    { address = "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK" },
    { address = "GRoLLzvxpxxu2PGNJMMeZPyMCooCpyPgksTdPGjdhrRE" },
    { address = "So11111111111111111111111111111111111111112" }
]
```

**Cargo.toml:**
```toml
[workspace]
members = [
    "programs/character-nft",
    "programs/item-vault",
    "programs/turf-control", 
    "programs/mob-token",
    "programs/fam-token",
    "programs/game-treasury"
]

[workspace.dependencies]
anchor-lang = "0.29.0"
anchor-spl = "0.29.0"
solana-program = "~1.16.0"
spl-token = "4.0.0"
spl-associated-token-account = "2.2.0"
mpl-token-metadata = "3.2.0"
arrayref = "0.3.7"
borsh = "0.10.3"
solana-security-txt = "1.1.1"
```

**.gitignore:**
```
# Solana/Anchor
target/
.anchor/
test-ledger/
.DS_Store

# Node.js
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.npm
.yarn-integrity
.pnpm-debug.log*

# Environment variables
.env
.env.local
.env.development.local
.env.test.local
.env.production.local
.env.devnet
.env.mainnet

# Logs
logs/
*.log

# Generated files
deployed-program-ids.json
character-collection-mint.json
mob-token-mint.json
fam-token-mint.json
item-collections.json
turf-collection.json
deployment-report-*.md
initialization-summary-*.json

# Generated assets
assets/generated/
assets/metadata/

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Rust
**/*.rs.bk
Cargo.lock
```

## Step 3: Create Directory Structure

```bash
mkdir -p programs/character-nft/src
mkdir -p programs/mob-token/src
mkdir -p programs/fam-token/src
mkdir -p programs/item-vault/src
mkdir -p programs/turf-control/src
mkdir -p programs/game-treasury/src
mkdir -p scripts
mkdir -p docs
mkdir -p app/Assets/Scripts/{Blockchain,Game,UI,Utils}
mkdir -p backend/src/{auth,game,economy,dao}
mkdir -p tests
mkdir -p .github/workflows
```

## Step 4: Upload to GitHub

```bash
git add .
git commit -m "Initial commit: Complete Mafia NFT project structure"
git push origin main
```

## Next: I'll provide all the smart contract files in the next message!
