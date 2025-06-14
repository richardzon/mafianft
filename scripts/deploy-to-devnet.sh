#!/bin/bash

# Deploy Mafia NFT Smart Contracts to Devnet
# This script handles the complete deployment process

set -e

echo "🚀 Deploying Mafia NFT to Solana Devnet..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NETWORK="devnet"
CLUSTER_URL="https://api.devnet.solana.com"

# Check if Solana CLI is installed
check_solana_cli() {
    echo -e "\n${BLUE}🔍 Checking Solana CLI installation...${NC}"
    
    if ! command -v solana &> /dev/null; then
        echo -e "${RED}❌ Solana CLI not found!${NC}"
        echo -e "${YELLOW}Please install Solana CLI:${NC}"
        echo "curl -sSfL https://release.solana.com/stable/install | sh"
        echo "export PATH=\"\$HOME/.local/share/solana/install/active_release/bin:\$PATH\""
        exit 1
    fi
    
    echo -e "${GREEN}✅ Solana CLI found: $(solana --version)${NC}"
}

# Check if Anchor CLI is installed
check_anchor_cli() {
    echo -e "\n${BLUE}🔍 Checking Anchor CLI installation...${NC}"
    
    if ! command -v anchor &> /dev/null; then
        echo -e "${RED}❌ Anchor CLI not found!${NC}"
        echo -e "${YELLOW}Please install Anchor CLI:${NC}"
        echo "npm install -g @coral-xyz/anchor-cli@0.29.0"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Anchor CLI found: $(anchor --version)${NC}"
}

# Setup Solana configuration
setup_solana_config() {
    echo -e "\n${BLUE}⚙️  Setting up Solana configuration...${NC}"
    
    # Set cluster to devnet
    solana config set --url $CLUSTER_URL
    echo -e "${GREEN}✅ Cluster set to: $CLUSTER_URL${NC}"
    
    # Check if keypair exists, create if not
    if [ ! -f ~/.config/solana/id.json ]; then
        echo -e "${YELLOW}⚠️  No keypair found, generating new one...${NC}"
        solana-keygen new --outfile ~/.config/solana/id.json --no-bip39-passphrase
    fi
    
    # Show current configuration
    echo -e "\n${BLUE}📋 Current Solana Configuration:${NC}"
    solana config get
    
    # Show wallet address
    WALLET_ADDRESS=$(solana address)
    echo -e "\n${BLUE}💳 Wallet Address: ${GREEN}$WALLET_ADDRESS${NC}"
}

# Request airdrop for devnet
request_airdrop() {
    echo -e "\n${BLUE}💰 Requesting SOL airdrop for devnet...${NC}"
    
    BALANCE=$(solana balance --lamports)
    echo -e "Current balance: ${GREEN}$((BALANCE / 1000000000)) SOL${NC}"
    
    if [ $BALANCE -lt 2000000000 ]; then  # Less than 2 SOL
        echo -e "${YELLOW}⚠️  Low balance, requesting airdrop...${NC}"
        solana airdrop 2
        echo -e "${GREEN}✅ Airdrop completed${NC}"
    else
        echo -e "${GREEN}✅ Sufficient balance for deployment${NC}"
    fi
}

# Build Anchor programs
build_programs() {
    echo -e "\n${BLUE}🔨 Building Anchor programs...${NC}"
    
    # Clean previous builds
    if [ -d "target" ]; then
        rm -rf target
        echo -e "${GREEN}✅ Cleaned previous build artifacts${NC}"
    fi
    
    # Build all programs
    anchor build
    echo -e "${GREEN}✅ All programs built successfully${NC}"
    
    # Show program sizes
    echo -e "\n${BLUE}📊 Program Sizes:${NC}"
    ls -lh target/deploy/*.so | awk '{print $9 ": " $5}'
}

# Deploy programs to devnet
deploy_programs() {
    echo -e "\n${BLUE}🚀 Deploying programs to devnet...${NC}"
    
    # Deploy all programs
    anchor deploy --provider.cluster devnet
    
    echo -e "${GREEN}✅ All programs deployed successfully${NC}"
    
    # Show deployed program IDs
    echo -e "\n${BLUE}📋 Deployed Program IDs:${NC}"
    ls target/deploy/*-keypair.json | while read keypair; do
        program_name=$(basename "$keypair" -keypair.json)
        program_id=$(solana address -k "$keypair")
        echo -e "${GREEN}$program_name: $program_id${NC}"
    done
}

# Update program IDs in configuration files
update_program_ids() {
    echo -e "\n${BLUE}📝 Updating program IDs in configuration files...${NC}"
    
    # Create a temporary file to store program IDs
    PROGRAM_IDS_FILE="deployed-program-ids.json"
    echo "{" > $PROGRAM_IDS_FILE
    
    # Extract program IDs
    ls target/deploy/*-keypair.json | while read keypair; do
        program_name=$(basename "$keypair" -keypair.json)
        program_id=$(solana address -k "$keypair")
        
        # Convert program name to match our naming convention
        case $program_name in
            "character_nft") program_key="CHARACTER_NFT_PROGRAM_ID" ;;
            "mob_token") program_key="MOB_TOKEN_PROGRAM_ID" ;;
            "fam_token") program_key="FAM_TOKEN_PROGRAM_ID" ;;
            "item_vault") program_key="ITEM_VAULT_PROGRAM_ID" ;;
            "turf_control") program_key="TURF_CONTROL_PROGRAM_ID" ;;
            "game_treasury") program_key="GAME_TREASURY_PROGRAM_ID" ;;
            *) program_key="${program_name^^}_PROGRAM_ID" ;;
        esac
        
        echo "  \"$program_key\": \"$program_id\"," >> $PROGRAM_IDS_FILE
    done
    
    # Close JSON file
    sed -i '$ s/,$//' $PROGRAM_IDS_FILE  # Remove last comma
    echo "}" >> $PROGRAM_IDS_FILE
    
    echo -e "${GREEN}✅ Program IDs saved to: $PROGRAM_IDS_FILE${NC}"
    
    # Update Unity Constants.cs file if it exists
    if [ -f "app/Assets/Scripts/Utils/Constants.cs" ]; then
        echo -e "${BLUE}📝 Updating Unity Constants.cs...${NC}"
        
        # Read program IDs and update Constants.cs
        while IFS= read -r line; do
            if [[ $line =~ \"([^\"]+)\":[[:space:]]*\"([^\"]+)\" ]]; then
                key="${BASH_REMATCH[1]}"
                value="${BASH_REMATCH[2]}"
                
                # Update the constant in Constants.cs
                sed -i "s/public const string $key = \"[^\"]*\"/public const string $key = \"$value\"/" app/Assets/Scripts/Utils/Constants.cs
            fi
        done < $PROGRAM_IDS_FILE
        
        echo -e "${GREEN}✅ Unity Constants.cs updated${NC}"
    fi
}

# Run basic tests
run_tests() {
    echo -e "\n${BLUE}🧪 Running deployment tests...${NC}"
    
    # Test program deployment by checking account info
    ls target/deploy/*-keypair.json | while read keypair; do
        program_name=$(basename "$keypair" -keypair.json)
        program_id=$(solana address -k "$keypair")
        
        echo -e "${BLUE}Testing $program_name ($program_id)...${NC}"
        
        # Check if program account exists
        if solana account "$program_id" > /dev/null 2>&1; then
            echo -e "${GREEN}✅ $program_name deployed successfully${NC}"
        else
            echo -e "${RED}❌ $program_name deployment failed${NC}"
        fi
    done
}

# Generate deployment report
generate_report() {
    echo -e "\n${BLUE}📊 Generating deployment report...${NC}"
    
    REPORT_FILE="deployment-report-$(date +%Y%m%d-%H%M%S).md"
    
    cat > $REPORT_FILE << EOF
# Mafia NFT Devnet Deployment Report

**Deployment Date**: $(date)
**Network**: $NETWORK
**Cluster URL**: $CLUSTER_URL
**Deployer**: $(solana address)

## Deployed Programs

EOF
    
    # Add program information
    ls target/deploy/*-keypair.json | while read keypair; do
        program_name=$(basename "$keypair" -keypair.json)
        program_id=$(solana address -k "$keypair")
        program_size=$(ls -lh "target/deploy/${program_name}.so" | awk '{print $5}')
        
        cat >> $REPORT_FILE << EOF
### $program_name
- **Program ID**: \`$program_id\`
- **Size**: $program_size
- **Explorer**: https://explorer.solana.com/address/$program_id?cluster=devnet

EOF
    done
    
    cat >> $REPORT_FILE << EOF

## Next Steps

1. **Test Program Functionality**
   \`\`\`bash
   anchor test --skip-local-validator
   \`\`\`

2. **Initialize Programs**
   \`\`\`bash
   node scripts/initialize-programs.js devnet
   \`\`\`

3. **Generate NFT Collection**
   \`\`\`bash
   node scripts/generate-ai-art.js --count 1000
   \`\`\`

4. **Deploy Unity Client**
   - Update program IDs in Constants.cs
   - Build and test on target platforms

5. **Deploy Backend API**
   - Update environment variables
   - Deploy to staging environment

## Useful Commands

\`\`\`bash
# Check program account
solana account <PROGRAM_ID> --output json

# View program logs
solana logs <PROGRAM_ID>

# Get program data size
solana program show <PROGRAM_ID>
\`\`\`

## Support

- **Discord**: https://discord.gg/mafianft
- **GitHub**: https://github.com/richardzon/mafianft
- **Explorer**: https://explorer.solana.com/?cluster=devnet
EOF
    
    echo -e "${GREEN}✅ Deployment report saved to: $REPORT_FILE${NC}"
}

# Main deployment function
main() {
    echo -e "${GREEN}🎮 Mafia NFT Devnet Deployment${NC}"
    echo -e "${GREEN}================================${NC}"
    
    # Pre-flight checks
    check_solana_cli
    check_anchor_cli
    
    # Setup and deploy
    setup_solana_config
    request_airdrop
    build_programs
    deploy_programs
    update_program_ids
    run_tests
    generate_report
    
    echo -e "\n${GREEN}🎉 Deployment completed successfully!${NC}"
    echo -e "${GREEN}📋 Check the deployment report for details${NC}"
    echo -e "${GREEN}🌐 View on Solana Explorer: https://explorer.solana.com/?cluster=devnet${NC}"
    
    echo -e "\n${BLUE}🚀 Next Steps:${NC}"
    echo -e "1. Run: ${YELLOW}anchor test --skip-local-validator${NC}"
    echo -e "2. Run: ${YELLOW}node scripts/initialize-programs.js devnet${NC}"
    echo -e "3. Run: ${YELLOW}node scripts/generate-ai-art.js --count 1000${NC}"
    echo -e "4. Update Unity client with new program IDs"
    echo -e "5. Deploy backend API to staging environment"
}

# Run main function
main "$@"
