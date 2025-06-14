#!/bin/bash

# Fast deployment script for Mafia NFT
# This script deploys the minimum viable product for immediate launch

set -e

echo "🚀 Starting Fast Deployment for Mafia NFT..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
NETWORK=${1:-devnet}
SKIP_TESTS=${2:-false}

echo -e "${YELLOW}Deploying to: $NETWORK${NC}"
echo -e "${YELLOW}Skip tests: $SKIP_TESTS${NC}"

# Step 1: Install dependencies
echo -e "\n${GREEN}📦 Installing dependencies...${NC}"
npm install
cd backend && npm install && cd ..

# Step 2: Build Anchor programs (only essential ones)
echo -e "\n${GREEN}🔨 Building essential Anchor programs...${NC}"
anchor build

# Step 3: Run tests (if not skipped)
if [ "$SKIP_TESTS" != "true" ]; then
    echo -e "\n${GREEN}🧪 Running critical tests...${NC}"
    anchor test --skip-local-validator
else
    echo -e "\n${YELLOW}⚠️  Skipping tests for fast deployment${NC}"
fi

# Step 4: Deploy to specified network
echo -e "\n${GREEN}🚀 Deploying programs to $NETWORK...${NC}"

if [ "$NETWORK" = "mainnet" ]; then
    echo -e "${RED}⚠️  MAINNET DEPLOYMENT - ARE YOU SURE? (y/N)${NC}"
    read -r response
    if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo "Deployment cancelled."
        exit 1
    fi
    anchor deploy --provider.cluster mainnet
elif [ "$NETWORK" = "devnet" ]; then
    anchor deploy --provider.cluster devnet
else
    echo -e "${RED}❌ Invalid network: $NETWORK${NC}"
    exit 1
fi

# Step 5: Update program IDs in configuration
echo -e "\n${GREEN}📝 Updating program IDs...${NC}"
node scripts/update-program-ids.js $NETWORK

# Step 6: Deploy backend API
echo -e "\n${GREEN}🌐 Deploying backend API...${NC}"
cd backend

# Build backend
npm run build

# Set environment variables based on network
if [ "$NETWORK" = "mainnet" ]; then
    export NODE_ENV=production
    export SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
    export DATABASE_URL=$MAINNET_DATABASE_URL
    export REDIS_URL=$MAINNET_REDIS_URL
else
    export NODE_ENV=development
    export SOLANA_RPC_URL=https://api.devnet.solana.com
    export DATABASE_URL=$DEVNET_DATABASE_URL
    export REDIS_URL=$DEVNET_REDIS_URL
fi

# Run database migrations
npm run migration:run

# Start backend (in background for CI/CD)
if [ "$CI" = "true" ]; then
    npm run start:prod &
    BACKEND_PID=$!
    echo "Backend started with PID: $BACKEND_PID"
else
    echo -e "${YELLOW}Backend ready to start. Run: npm run start:prod${NC}"
fi

cd ..

# Step 7: Generate AI art assets (placeholder)
echo -e "\n${GREEN}🎨 Generating AI art assets...${NC}"
node scripts/generate-ai-art.js

# Step 8: Setup NFT metadata
echo -e "\n${GREEN}📄 Setting up NFT metadata...${NC}"
node scripts/setup-metadata.js $NETWORK

# Step 9: Initialize programs with basic configuration
echo -e "\n${GREEN}⚙️  Initializing programs...${NC}"
node scripts/initialize-programs.js $NETWORK

# Step 10: Create initial NFT collection
echo -e "\n${GREEN}🖼️  Creating initial NFT collections...${NC}"
node scripts/create-collections.js $NETWORK

# Step 11: Setup basic game economy
echo -e "\n${GREEN}💰 Setting up game economy...${NC}"
node scripts/setup-economy.js $NETWORK

# Step 12: Health check
echo -e "\n${GREEN}🏥 Running health checks...${NC}"
node scripts/health-check.js $NETWORK

# Step 13: Generate deployment report
echo -e "\n${GREEN}📊 Generating deployment report...${NC}"
cat > deployment-report.md << EOF
# Mafia NFT Deployment Report

**Network:** $NETWORK
**Deployment Time:** $(date)
**Version:** $(git rev-parse --short HEAD)

## Deployed Programs
- Character NFT: $(solana address -k target/deploy/character_nft-keypair.json)
- MOB Token: $(solana address -k target/deploy/mob_token-keypair.json)
- FAM Token: $(solana address -k target/deploy/fam_token-keypair.json)
- Game Treasury: $(solana address -k target/deploy/game_treasury-keypair.json)

## Backend API
- Status: Deployed
- URL: https://$NETWORK-api.mafianft.com
- Health: $(curl -s https://$NETWORK-api.mafianft.com/health || echo "Not accessible")

## Next Steps
1. Test core functionality
2. Deploy Unity client
3. Setup monitoring
4. Launch marketing campaign

## Quick Start Commands
\`\`\`bash
# Test character minting
npm run test:character-mint

# Test token operations
npm run test:token-ops

# Test game treasury
npm run test:treasury
\`\`\`
EOF

echo -e "\n${GREEN}✅ Fast deployment completed!${NC}"
echo -e "${GREEN}📋 Check deployment-report.md for details${NC}"

# Step 14: Launch monitoring (if available)
if command -v pm2 &> /dev/null; then
    echo -e "\n${GREEN}📊 Starting monitoring...${NC}"
    pm2 start ecosystem.config.js --env $NETWORK
fi

# Step 15: Open relevant URLs
if [ "$CI" != "true" ]; then
    echo -e "\n${GREEN}🌐 Opening relevant URLs...${NC}"
    if command -v open &> /dev/null; then
        open "https://explorer.solana.com/?cluster=$NETWORK"
        open "https://$NETWORK-api.mafianft.com/api/docs"
    fi
fi

echo -e "\n${GREEN}🎉 Mafia NFT is now live on $NETWORK!${NC}"
echo -e "${YELLOW}⚡ Total deployment time: $SECONDS seconds${NC}"

# Cleanup on exit
cleanup() {
    if [ ! -z "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null || true
    fi
}
trap cleanup EXIT
