#!/bin/bash

# GitHub Repository Setup Script for Mafia NFT
# This script helps you upload your project to GitHub

set -e

echo "🚀 Setting up Mafia NFT on GitHub..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_NAME="mafianft"
GITHUB_USERNAME="richardzon"
REPO_URL="https://github.com/${GITHUB_USERNAME}/${REPO_NAME}.git"

# Check if git is installed
check_git() {
    echo -e "\n${BLUE}🔍 Checking Git installation...${NC}"
    
    if ! command -v git &> /dev/null; then
        echo -e "${RED}❌ Git not found!${NC}"
        echo -e "${YELLOW}Please install Git: https://git-scm.com/downloads${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Git found: $(git --version)${NC}"
}

# Check if we're in the right directory
check_directory() {
    echo -e "\n${BLUE}📁 Checking project directory...${NC}"
    
    if [ ! -f "Anchor.toml" ] || [ ! -d "programs" ]; then
        echo -e "${RED}❌ Not in Mafia NFT project directory!${NC}"
        echo -e "${YELLOW}Please run this script from the project root directory${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ In correct project directory${NC}"
}

# Initialize git repository
init_git() {
    echo -e "\n${BLUE}🔧 Initializing Git repository...${NC}"
    
    if [ -d ".git" ]; then
        echo -e "${YELLOW}⚠️  Git repository already exists${NC}"
        return
    fi
    
    git init
    echo -e "${GREEN}✅ Git repository initialized${NC}"
}

# Configure git user (if not already configured)
configure_git() {
    echo -e "\n${BLUE}👤 Configuring Git user...${NC}"
    
    # Check if user.name is configured
    if ! git config user.name &> /dev/null; then
        echo -e "${YELLOW}⚠️  Git user.name not configured${NC}"
        echo -e "${BLUE}Please enter your name:${NC}"
        read -r git_name
        git config user.name "$git_name"
        echo -e "${GREEN}✅ Git user.name set to: $git_name${NC}"
    else
        echo -e "${GREEN}✅ Git user.name: $(git config user.name)${NC}"
    fi
    
    # Check if user.email is configured
    if ! git config user.email &> /dev/null; then
        echo -e "${YELLOW}⚠️  Git user.email not configured${NC}"
        echo -e "${BLUE}Please enter your email:${NC}"
        read -r git_email
        git config user.email "$git_email"
        echo -e "${GREEN}✅ Git user.email set to: $git_email${NC}"
    else
        echo -e "${GREEN}✅ Git user.email: $(git config user.email)${NC}"
    fi
}

# Add files to git
add_files() {
    echo -e "\n${BLUE}📦 Adding files to Git...${NC}"
    
    # Add all files
    git add .
    
    # Show status
    echo -e "${BLUE}📋 Git status:${NC}"
    git status --short
    
    echo -e "${GREEN}✅ Files added to Git${NC}"
}

# Create initial commit
create_commit() {
    echo -e "\n${BLUE}💾 Creating initial commit...${NC}"
    
    # Check if there are any changes to commit
    if git diff --cached --quiet; then
        echo -e "${YELLOW}⚠️  No changes to commit${NC}"
        return
    fi
    
    # Create commit
    git commit -m "Initial commit: Complete Mafia NFT project

- 6 Anchor smart contracts (Character NFT, MOB Token, FAM Token, Item Vault, Turf Control, Game Treasury)
- Unity game client with Solana wallet integration
- NestJS backend API with authentication and services
- AI-powered NFT generation system (10,000 unique characters)
- Automated deployment scripts and testing framework
- Comprehensive documentation and deployment guides

Ready for devnet deployment and testing."
    
    echo -e "${GREEN}✅ Initial commit created${NC}"
}

# Add GitHub remote
add_remote() {
    echo -e "\n${BLUE}🔗 Adding GitHub remote...${NC}"
    
    # Check if remote already exists
    if git remote get-url origin &> /dev/null; then
        echo -e "${YELLOW}⚠️  Remote 'origin' already exists${NC}"
        echo -e "${BLUE}Current remote: $(git remote get-url origin)${NC}"
        
        echo -e "${BLUE}Do you want to update it? (y/N):${NC}"
        read -r update_remote
        if [[ "$update_remote" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            git remote set-url origin "$REPO_URL"
            echo -e "${GREEN}✅ Remote updated to: $REPO_URL${NC}"
        fi
    else
        git remote add origin "$REPO_URL"
        echo -e "${GREEN}✅ Remote added: $REPO_URL${NC}"
    fi
}

# Push to GitHub
push_to_github() {
    echo -e "\n${BLUE}🚀 Pushing to GitHub...${NC}"
    
    # Set main branch
    git branch -M main
    
    # Push to GitHub
    echo -e "${BLUE}Pushing to GitHub (you may need to authenticate)...${NC}"
    
    if git push -u origin main; then
        echo -e "${GREEN}✅ Successfully pushed to GitHub!${NC}"
    else
        echo -e "${RED}❌ Failed to push to GitHub${NC}"
        echo -e "${YELLOW}This might be because:${NC}"
        echo -e "${YELLOW}1. The repository doesn't exist on GitHub yet${NC}"
        echo -e "${YELLOW}2. You don't have push permissions${NC}"
        echo -e "${YELLOW}3. Authentication failed${NC}"
        echo -e "\n${BLUE}Please:${NC}"
        echo -e "${BLUE}1. Create the repository on GitHub: https://github.com/new${NC}"
        echo -e "${BLUE}2. Make sure you're authenticated (GitHub CLI or SSH keys)${NC}"
        echo -e "${BLUE}3. Try running: git push -u origin main${NC}"
        return 1
    fi
}

# Show next steps
show_next_steps() {
    echo -e "\n${GREEN}🎉 GitHub setup complete!${NC}"
    echo -e "${GREEN}📋 Repository URL: https://github.com/${GITHUB_USERNAME}/${REPO_NAME}${NC}"
    
    echo -e "\n${BLUE}🚀 Next Steps:${NC}"
    echo -e "1. ${YELLOW}Visit your repository: https://github.com/${GITHUB_USERNAME}/${REPO_NAME}${NC}"
    echo -e "2. ${YELLOW}Verify all files uploaded correctly${NC}"
    echo -e "3. ${YELLOW}Install Solana CLI: curl -sSfL https://release.solana.com/stable/install | sh${NC}"
    echo -e "4. ${YELLOW}Install Anchor CLI: npm install -g @coral-xyz/anchor-cli@0.29.0${NC}"
    echo -e "5. ${YELLOW}Deploy to devnet: ./scripts/deploy-to-devnet.sh${NC}"
    
    echo -e "\n${BLUE}📚 Documentation:${NC}"
    echo -e "- ${YELLOW}Deployment Guide: docs/deployment-guide.md${NC}"
    echo -e "- ${YELLOW}Fast Launch Plan: docs/fast-launch-plan.md${NC}"
    echo -e "- ${YELLOW}Project Status: READY_FOR_DEPLOYMENT.md${NC}"
}

# Main function
main() {
    echo -e "${GREEN}🎮 Mafia NFT GitHub Setup${NC}"
    echo -e "${GREEN}=========================${NC}"
    
    check_git
    check_directory
    init_git
    configure_git
    add_files
    create_commit
    add_remote
    
    echo -e "\n${BLUE}Ready to push to GitHub!${NC}"
    echo -e "${BLUE}Repository: $REPO_URL${NC}"
    echo -e "\n${YELLOW}⚠️  Make sure you've created the repository on GitHub first:${NC}"
    echo -e "${YELLOW}   https://github.com/new${NC}"
    echo -e "${YELLOW}   Repository name: $REPO_NAME${NC}"
    echo -e "${YELLOW}   Don't initialize with README (we already have one)${NC}"
    
    echo -e "\n${BLUE}Continue with push? (y/N):${NC}"
    read -r continue_push
    
    if [[ "$continue_push" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        if push_to_github; then
            show_next_steps
        fi
    else
        echo -e "\n${YELLOW}⏸️  Push cancelled. You can push later with:${NC}"
        echo -e "${YELLOW}   git push -u origin main${NC}"
        echo -e "\n${BLUE}Don't forget to create the repository on GitHub first!${NC}"
    fi
}

# Run main function
main "$@"
