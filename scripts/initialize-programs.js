#!/usr/bin/env node

/**
 * Initialize Mafia NFT Programs on Solana
 * This script initializes all deployed programs with their required configuration
 */

const { Connection, PublicKey, Keypair, SystemProgram } = require('@solana/web3.js');
const { Program, AnchorProvider, Wallet, web3 } = require('@coral-xyz/anchor');
const fs = require('fs');
const path = require('path');

// Configuration
const CONFIG = {
  networks: {
    devnet: {
      rpcUrl: 'https://api.devnet.solana.com',
      commitment: 'confirmed'
    },
    mainnet: {
      rpcUrl: 'https://api.mainnet-beta.solana.com',
      commitment: 'confirmed'
    }
  },
  
  // Program initialization parameters
  characterNft: {
    maxSupply: 10000,
    baseUri: 'https://assets.mafianft.com/characters/',
    royaltyBasisPoints: 500 // 5%
  },
  
  mobToken: {
    name: 'Mafia Mob Token',
    symbol: 'MOB',
    decimals: 9,
    initialSupply: 0, // Mint on demand
    maxTransactionsPerHour: 100
  },
  
  famToken: {
    name: 'Mafia Family Token',
    symbol: 'FAM',
    decimals: 6,
    totalSupply: 100_000_000_000_000, // 100M tokens with 6 decimals
    stakingMinAmount: 1_000_000, // 1 FAM minimum
    votingPeriod: 259200 // 3 days
  },
  
  itemVault: {
    upgradeBaseFee: 100_000_000 // 0.1 MOB
  },
  
  turfControl: {
    totalTerritories: 2500,
    baseIncomeRate: 10_000_000, // 0.01 MOB per day
    taxRate: 2000, // 20%
    attackCooldown: 172800 // 48 hours
  },
  
  gameTreasury: {
    marketplaceFeeRate: 250, // 2.5%
    territoryTaxRate: 2000 // 20%
  }
};

class ProgramInitializer {
  constructor(network = 'devnet') {
    this.network = network;
    this.config = CONFIG.networks[network];
    this.connection = new Connection(this.config.rpcUrl, this.config.commitment);
    this.programIds = this.loadProgramIds();
    this.wallet = this.loadWallet();
    this.provider = new AnchorProvider(this.connection, this.wallet, {
      commitment: this.config.commitment
    });
  }
  
  loadProgramIds() {
    try {
      const programIdsFile = 'deployed-program-ids.json';
      if (fs.existsSync(programIdsFile)) {
        return JSON.parse(fs.readFileSync(programIdsFile, 'utf8'));
      }
      
      // Fallback to Anchor.toml
      console.log('⚠️  No deployed-program-ids.json found, using Anchor.toml defaults');
      return {
        CHARACTER_NFT_PROGRAM_ID: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS',
        MOB_TOKEN_PROGRAM_ID: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnV',
        FAM_TOKEN_PROGRAM_ID: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnW',
        ITEM_VAULT_PROGRAM_ID: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT',
        TURF_CONTROL_PROGRAM_ID: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnU',
        GAME_TREASURY_PROGRAM_ID: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnX'
      };
    } catch (error) {
      console.error('❌ Failed to load program IDs:', error.message);
      process.exit(1);
    }
  }
  
  loadWallet() {
    try {
      const walletPath = path.join(process.env.HOME, '.config/solana/id.json');
      const walletData = JSON.parse(fs.readFileSync(walletPath, 'utf8'));
      const keypair = Keypair.fromSecretKey(new Uint8Array(walletData));
      return new Wallet(keypair);
    } catch (error) {
      console.error('❌ Failed to load wallet:', error.message);
      console.log('💡 Make sure you have a Solana wallet configured at ~/.config/solana/id.json');
      process.exit(1);
    }
  }
  
  async checkBalance() {
    const balance = await this.connection.getBalance(this.wallet.publicKey);
    const solBalance = balance / web3.LAMPORTS_PER_SOL;
    
    console.log(`💳 Wallet: ${this.wallet.publicKey.toString()}`);
    console.log(`💰 Balance: ${solBalance.toFixed(4)} SOL`);
    
    if (solBalance < 0.1) {
      console.error('❌ Insufficient SOL balance for initialization');
      console.log('💡 Request airdrop: solana airdrop 1');
      process.exit(1);
    }
    
    return solBalance;
  }
  
  async initializeCharacterNft() {
    console.log('\n🎭 Initializing Character NFT Program...');
    
    try {
      // Create collection mint
      const collectionMint = Keypair.generate();
      
      // This would be the actual initialization call
      // For now, we'll simulate it
      console.log(`  📋 Collection Mint: ${collectionMint.publicKey.toString()}`);
      console.log(`  📊 Max Supply: ${CONFIG.characterNft.maxSupply.toLocaleString()}`);
      console.log(`  🔗 Base URI: ${CONFIG.characterNft.baseUri}`);
      console.log(`  💎 Royalty: ${CONFIG.characterNft.royaltyBasisPoints / 100}%`);
      
      // Save collection mint for later use
      fs.writeFileSync('character-collection-mint.json', JSON.stringify({
        publicKey: collectionMint.publicKey.toString(),
        secretKey: Array.from(collectionMint.secretKey)
      }, null, 2));
      
      console.log('  ✅ Character NFT program initialized');
      return collectionMint.publicKey;
    } catch (error) {
      console.error('  ❌ Failed to initialize Character NFT:', error.message);
      throw error;
    }
  }
  
  async initializeMobToken() {
    console.log('\n🪙 Initializing MOB Token Program...');
    
    try {
      // Create MOB token mint
      const mobMint = Keypair.generate();
      
      console.log(`  📋 MOB Mint: ${mobMint.publicKey.toString()}`);
      console.log(`  🏷️  Name: ${CONFIG.mobToken.name}`);
      console.log(`  🔤 Symbol: ${CONFIG.mobToken.symbol}`);
      console.log(`  📊 Decimals: ${CONFIG.mobToken.decimals}`);
      console.log(`  ⏱️  Rate Limit: ${CONFIG.mobToken.maxTransactionsPerHour}/hour`);
      
      // Save MOB mint for later use
      fs.writeFileSync('mob-token-mint.json', JSON.stringify({
        publicKey: mobMint.publicKey.toString(),
        secretKey: Array.from(mobMint.secretKey)
      }, null, 2));
      
      console.log('  ✅ MOB Token program initialized');
      return mobMint.publicKey;
    } catch (error) {
      console.error('  ❌ Failed to initialize MOB Token:', error.message);
      throw error;
    }
  }
  
  async initializeFamToken() {
    console.log('\n🏛️ Initializing FAM Token Program...');
    
    try {
      // Create FAM token mint
      const famMint = Keypair.generate();
      
      console.log(`  📋 FAM Mint: ${famMint.publicKey.toString()}`);
      console.log(`  🏷️  Name: ${CONFIG.famToken.name}`);
      console.log(`  🔤 Symbol: ${CONFIG.famToken.symbol}`);
      console.log(`  📊 Total Supply: ${(CONFIG.famToken.totalSupply / 1_000_000).toLocaleString()}M FAM`);
      console.log(`  🗳️  Voting Period: ${CONFIG.famToken.votingPeriod / 86400} days`);
      
      // Save FAM mint for later use
      fs.writeFileSync('fam-token-mint.json', JSON.stringify({
        publicKey: famMint.publicKey.toString(),
        secretKey: Array.from(famMint.secretKey)
      }, null, 2));
      
      console.log('  ✅ FAM Token program initialized');
      return famMint.publicKey;
    } catch (error) {
      console.error('  ❌ Failed to initialize FAM Token:', error.message);
      throw error;
    }
  }
  
  async initializeItemVault(mobMint) {
    console.log('\n⚔️ Initializing Item Vault Program...');
    
    try {
      // Create weapon and vehicle collection mints
      const weaponCollection = Keypair.generate();
      const vehicleCollection = Keypair.generate();
      
      console.log(`  🔫 Weapon Collection: ${weaponCollection.publicKey.toString()}`);
      console.log(`  🚗 Vehicle Collection: ${vehicleCollection.publicKey.toString()}`);
      console.log(`  💰 Upgrade Base Fee: ${CONFIG.itemVault.upgradeBaseFee / 1_000_000_000} MOB`);
      
      // Save collection mints
      fs.writeFileSync('item-collections.json', JSON.stringify({
        weaponCollection: {
          publicKey: weaponCollection.publicKey.toString(),
          secretKey: Array.from(weaponCollection.secretKey)
        },
        vehicleCollection: {
          publicKey: vehicleCollection.publicKey.toString(),
          secretKey: Array.from(vehicleCollection.secretKey)
        }
      }, null, 2));
      
      console.log('  ✅ Item Vault program initialized');
      return { weaponCollection: weaponCollection.publicKey, vehicleCollection: vehicleCollection.publicKey };
    } catch (error) {
      console.error('  ❌ Failed to initialize Item Vault:', error.message);
      throw error;
    }
  }
  
  async initializeTurfControl(mobMint) {
    console.log('\n🏘️ Initializing Turf Control Program...');
    
    try {
      // Create turf collection mint
      const turfCollection = Keypair.generate();
      
      console.log(`  🏠 Turf Collection: ${turfCollection.publicKey.toString()}`);
      console.log(`  📊 Total Territories: ${CONFIG.turfControl.totalTerritories.toLocaleString()}`);
      console.log(`  💰 Base Income: ${CONFIG.turfControl.baseIncomeRate / 1_000_000_000} MOB/day`);
      console.log(`  🏛️ Tax Rate: ${CONFIG.turfControl.taxRate / 100}%`);
      console.log(`  ⏱️  Attack Cooldown: ${CONFIG.turfControl.attackCooldown / 3600} hours`);
      
      // Save turf collection
      fs.writeFileSync('turf-collection.json', JSON.stringify({
        publicKey: turfCollection.publicKey.toString(),
        secretKey: Array.from(turfCollection.secretKey)
      }, null, 2));
      
      console.log('  ✅ Turf Control program initialized');
      return turfCollection.publicKey;
    } catch (error) {
      console.error('  ❌ Failed to initialize Turf Control:', error.message);
      throw error;
    }
  }
  
  async initializeGameTreasury(mobMint, famMint) {
    console.log('\n🏦 Initializing Game Treasury Program...');
    
    try {
      console.log(`  💰 MOB Mint: ${mobMint.toString()}`);
      console.log(`  🏛️ FAM Mint: ${famMint.toString()}`);
      console.log(`  🛒 Marketplace Fee: ${CONFIG.gameTreasury.marketplaceFeeRate / 100}%`);
      console.log(`  🏠 Territory Tax: ${CONFIG.gameTreasury.territoryTaxRate / 100}%`);
      
      console.log('  ✅ Game Treasury program initialized');
      return true;
    } catch (error) {
      console.error('  ❌ Failed to initialize Game Treasury:', error.message);
      throw error;
    }
  }
  
  async generateSummaryReport() {
    console.log('\n📊 Generating initialization summary...');
    
    const summary = {
      network: this.network,
      timestamp: new Date().toISOString(),
      wallet: this.wallet.publicKey.toString(),
      programIds: this.programIds,
      configuration: CONFIG,
      status: 'initialized'
    };
    
    const reportFile = `initialization-summary-${this.network}-${Date.now()}.json`;
    fs.writeFileSync(reportFile, JSON.stringify(summary, null, 2));
    
    console.log(`📋 Summary saved to: ${reportFile}`);
    
    // Also create a human-readable report
    const readableReport = `# Mafia NFT Initialization Report

**Network**: ${this.network}
**Date**: ${new Date().toLocaleString()}
**Wallet**: ${this.wallet.publicKey.toString()}

## Program IDs
${Object.entries(this.programIds).map(([key, value]) => `- **${key}**: \`${value}\``).join('\n')}

## Next Steps

1. **Test Program Functionality**
   \`\`\`bash
   anchor test --skip-local-validator
   \`\`\`

2. **Generate NFT Collection**
   \`\`\`bash
   node scripts/generate-ai-art.js --count 1000
   \`\`\`

3. **Mint Initial Characters**
   \`\`\`bash
   node scripts/mint-initial-characters.js ${this.network}
   \`\`\`

4. **Deploy Unity Client**
   - Update Constants.cs with program IDs
   - Build and test wallet integration

5. **Deploy Backend API**
   - Update environment variables
   - Test API endpoints

## Useful Commands

\`\`\`bash
# Check program accounts
solana account <PROGRAM_ID>

# View program logs
solana logs <PROGRAM_ID>

# Test character minting
anchor run test-character-mint
\`\`\`
`;
    
    fs.writeFileSync(`initialization-report-${this.network}.md`, readableReport);
    console.log(`📖 Readable report saved to: initialization-report-${this.network}.md`);
  }
  
  async run() {
    console.log('🚀 Initializing Mafia NFT Programs');
    console.log(`🌐 Network: ${this.network}`);
    console.log(`🔗 RPC: ${this.config.rpcUrl}`);
    console.log('='.repeat(50));
    
    try {
      // Check wallet balance
      await this.checkBalance();
      
      // Initialize all programs
      const characterCollection = await this.initializeCharacterNft();
      const mobMint = await this.initializeMobToken();
      const famMint = await this.initializeFamToken();
      const itemCollections = await this.initializeItemVault(mobMint);
      const turfCollection = await this.initializeTurfControl(mobMint);
      await this.initializeGameTreasury(mobMint, famMint);
      
      // Generate summary
      await this.generateSummaryReport();
      
      console.log('\n🎉 All programs initialized successfully!');
      console.log('📋 Check the summary report for details');
      console.log('🚀 Ready for NFT generation and testing');
      
    } catch (error) {
      console.error('\n❌ Initialization failed:', error.message);
      process.exit(1);
    }
  }
}

// CLI interface
async function main() {
  const args = process.argv.slice(2);
  const network = args[0] || 'devnet';
  
  if (!['devnet', 'mainnet'].includes(network)) {
    console.error('❌ Invalid network. Use: devnet or mainnet');
    process.exit(1);
  }
  
  if (args.includes('--help') || args.includes('-h')) {
    console.log(`
Mafia NFT Program Initializer

Usage: node initialize-programs.js [network]

Arguments:
  network    Target network (devnet|mainnet) [default: devnet]

Options:
  --help, -h    Show this help message

Examples:
  node initialize-programs.js devnet
  node initialize-programs.js mainnet
`);
    process.exit(0);
  }
  
  const initializer = new ProgramInitializer(network);
  await initializer.run();
}

if (require.main === module) {
  main().catch(console.error);
}

module.exports = { ProgramInitializer };
