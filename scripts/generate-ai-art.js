#!/usr/bin/env node

/**
 * AI Art Generation Script for Mafia NFT
 * Generates character art, weapons, vehicles, and metadata using AI APIs
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

// Configuration
const CONFIG = {
  // Art generation settings
  TOTAL_CHARACTERS: 10000,
  BATCH_SIZE: 100,
  OUTPUT_DIR: './assets/generated',
  METADATA_DIR: './assets/metadata',
  
  // Rarity distribution (must add up to 100)
  RARITY_DISTRIBUTION: {
    'Common': 40,      // 4,000 NFTs
    'Uncommon': 30,    // 3,000 NFTs  
    'Rare': 20,        // 2,000 NFTs
    'Epic': 8,         // 800 NFTs
    'Legendary': 1.8,  // 180 NFTs
    'Mythic': 0.2      // 20 NFTs
  },
  
  // Character traits and variations
  TRAITS: {
    background: ['City Street', 'Dark Alley', 'Nightclub', 'Casino', 'Warehouse', 'Penthouse'],
    ethnicity: ['Italian', 'Irish', 'Russian', 'Japanese', 'Mexican', 'American'],
    clothing: ['Suit', 'Leather Jacket', 'Trench Coat', 'Casual', 'Formal', 'Street Wear'],
    accessories: ['Sunglasses', 'Hat', 'Cigar', 'Gold Chain', 'Watch', 'Ring'],
    expression: ['Serious', 'Smirking', 'Angry', 'Confident', 'Mysterious', 'Intimidating'],
    weapon: ['None', 'Pistol', 'Knife', 'Baseball Bat', 'Tommy Gun', 'Brass Knuckles']
  }
};

// Ensure output directories exist
function createDirectories() {
  const dirs = [
    CONFIG.OUTPUT_DIR,
    CONFIG.METADATA_DIR,
    path.join(CONFIG.OUTPUT_DIR, 'characters'),
    path.join(CONFIG.OUTPUT_DIR, 'weapons'),
    path.join(CONFIG.OUTPUT_DIR, 'vehicles'),
    path.join(CONFIG.METADATA_DIR, 'characters'),
    path.join(CONFIG.METADATA_DIR, 'weapons'),
    path.join(CONFIG.METADATA_DIR, 'vehicles')
  ];
  
  dirs.forEach(dir => {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
  });
}

// Generate random traits for a character
function generateCharacterTraits(tokenId, rarity) {
  const traits = {};
  
  // Base stats based on rarity
  const statRanges = {
    'Common': { min: 40, max: 50 },
    'Uncommon': { min: 50, max: 60 },
    'Rare': { min: 60, max: 70 },
    'Epic': { min: 70, max: 80 },
    'Legendary': { min: 80, max: 90 },
    'Mythic': { min: 90, max: 100 }
  };
  
  const range = statRanges[rarity];
  
  // Generate stats
  traits.strength = Math.floor(Math.random() * (range.max - range.min + 1)) + range.min;
  traits.intelligence = Math.floor(Math.random() * (range.max - range.min + 1)) + range.min;
  traits.charisma = Math.floor(Math.random() * (range.max - range.min + 1)) + range.min;
  traits.luck = Math.floor(Math.random() * (range.max - range.min + 1)) + range.min;
  traits.stealth = Math.floor(Math.random() * (range.max - range.min + 1)) + range.min;
  
  // Generate visual traits
  Object.keys(CONFIG.TRAITS).forEach(traitType => {
    const options = CONFIG.TRAITS[traitType];
    traits[traitType] = options[Math.floor(Math.random() * options.length)];
  });
  
  return traits;
}

// Determine rarity based on distribution
function determineRarity(tokenId) {
  const random = Math.random() * 100;
  let cumulative = 0;
  
  for (const [rarity, percentage] of Object.entries(CONFIG.RARITY_DISTRIBUTION)) {
    cumulative += percentage;
    if (random <= cumulative) {
      return rarity;
    }
  }
  
  return 'Common'; // Fallback
}

// Generate AI art prompt for character
function generateCharacterPrompt(traits, rarity) {
  const style = rarity === 'Mythic' ? 'hyper-realistic, cinematic lighting' : 
                rarity === 'Legendary' ? 'detailed digital art, dramatic lighting' :
                rarity === 'Epic' ? 'high-quality digital art' :
                'digital art style';
  
  return `A ${traits.ethnicity} mafia character in ${traits.clothing}, ${traits.expression} expression, 
    ${traits.accessories !== 'None' ? `wearing ${traits.accessories},` : ''} 
    ${traits.weapon !== 'None' ? `holding ${traits.weapon},` : ''}
    standing in ${traits.background}, ${style}, portrait orientation, 
    noir atmosphere, high contrast, professional character art`;
}

// Generate metadata for character NFT
function generateCharacterMetadata(tokenId, traits, rarity) {
  const names = [
    'Vincent "The Shark"', 'Tony "Two-Times"', 'Maria "The Rose"', 'Sal "The Fish"',
    'Angelo "Big A"', 'Lucia "Lucky"', 'Marco "The Bull"', 'Sofia "Silk"',
    'Nico "The Ghost"', 'Isabella "Ice"', 'Rocco "The Rock"', 'Valentina "V"'
  ];
  
  const surnames = [
    'Torrino', 'Benedetti', 'Castellano', 'Romano', 'Vitale', 'Genovese',
    'Lucchese', 'Gambino', 'Bonanno', 'Colombo', 'DeCavalcante', 'Patriarca'
  ];
  
  const name = `${names[tokenId % names.length]} ${surnames[tokenId % surnames.length]}`;
  
  return {
    name: `${name} #${tokenId}`,
    description: `A ${rarity} mafia character from the streets of Neo-Crypto City. Each character has unique stats and abilities that affect gameplay in the Mafia NFT universe.`,
    image: `https://assets.mafianft.com/characters/${tokenId}.png`,
    external_url: `https://mafianft.com/character/${tokenId}`,
    attributes: [
      { trait_type: "Rarity", value: rarity },
      { trait_type: "Strength", value: traits.strength },
      { trait_type: "Intelligence", value: traits.intelligence },
      { trait_type: "Charisma", value: traits.charisma },
      { trait_type: "Luck", value: traits.luck },
      { trait_type: "Stealth", value: traits.stealth },
      { trait_type: "Background", value: traits.background },
      { trait_type: "Ethnicity", value: traits.ethnicity },
      { trait_type: "Clothing", value: traits.clothing },
      { trait_type: "Accessories", value: traits.accessories },
      { trait_type: "Expression", value: traits.expression },
      { trait_type: "Weapon", value: traits.weapon }
    ],
    properties: {
      category: "Character",
      creators: [
        {
          address: "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
          share: 100
        }
      ]
    }
  };
}

// Simulate AI art generation (placeholder for actual API calls)
function simulateArtGeneration(prompt, outputPath) {
  console.log(`🎨 Generating art: ${prompt.substring(0, 50)}...`);
  
  // Create a placeholder image file (in real implementation, this would call AI API)
  const placeholderContent = `# AI Generated Art Placeholder
Prompt: ${prompt}
Output: ${outputPath}
Generated: ${new Date().toISOString()}
`;
  
  fs.writeFileSync(outputPath.replace('.png', '.txt'), placeholderContent);
  return true;
}

// Generate a batch of characters
async function generateCharacterBatch(startId, batchSize) {
  console.log(`\n📦 Generating character batch ${startId}-${startId + batchSize - 1}...`);
  
  const characters = [];
  
  for (let i = 0; i < batchSize; i++) {
    const tokenId = startId + i;
    const rarity = determineRarity(tokenId);
    const traits = generateCharacterTraits(tokenId, rarity);
    
    // Generate art prompt
    const prompt = generateCharacterPrompt(traits, rarity);
    const imagePath = path.join(CONFIG.OUTPUT_DIR, 'characters', `${tokenId}.png`);
    
    // Simulate art generation
    simulateArtGeneration(prompt, imagePath);
    
    // Generate metadata
    const metadata = generateCharacterMetadata(tokenId, traits, rarity);
    const metadataPath = path.join(CONFIG.METADATA_DIR, 'characters', `${tokenId}.json`);
    
    fs.writeFileSync(metadataPath, JSON.stringify(metadata, null, 2));
    
    characters.push({
      tokenId,
      rarity,
      traits,
      metadata,
      imagePath,
      metadataPath
    });
    
    if ((i + 1) % 10 === 0) {
      process.stdout.write(`  Generated ${i + 1}/${batchSize} characters...\r`);
    }
  }
  
  console.log(`  ✅ Batch complete: ${batchSize} characters generated`);
  return characters;
}

// Generate collection statistics
function generateCollectionStats(allCharacters) {
  const stats = {
    total: allCharacters.length,
    rarityDistribution: {},
    traitDistribution: {}
  };
  
  // Count rarity distribution
  allCharacters.forEach(char => {
    stats.rarityDistribution[char.rarity] = (stats.rarityDistribution[char.rarity] || 0) + 1;
  });
  
  // Count trait distribution
  Object.keys(CONFIG.TRAITS).forEach(traitType => {
    stats.traitDistribution[traitType] = {};
    allCharacters.forEach(char => {
      const value = char.traits[traitType];
      stats.traitDistribution[traitType][value] = (stats.traitDistribution[traitType][value] || 0) + 1;
    });
  });
  
  return stats;
}

// Main generation function
async function generateAllArt() {
  console.log('🚀 Starting AI Art Generation for Mafia NFT');
  console.log(`📊 Target: ${CONFIG.TOTAL_CHARACTERS} characters`);
  console.log(`📦 Batch size: ${CONFIG.BATCH_SIZE}`);
  console.log('');
  
  createDirectories();
  
  const allCharacters = [];
  const totalBatches = Math.ceil(CONFIG.TOTAL_CHARACTERS / CONFIG.BATCH_SIZE);
  
  // Generate characters in batches
  for (let batch = 0; batch < totalBatches; batch++) {
    const startId = batch * CONFIG.BATCH_SIZE + 1;
    const batchSize = Math.min(CONFIG.BATCH_SIZE, CONFIG.TOTAL_CHARACTERS - (batch * CONFIG.BATCH_SIZE));
    
    const characters = await generateCharacterBatch(startId, batchSize);
    allCharacters.push(...characters);
    
    console.log(`📈 Progress: ${allCharacters.length}/${CONFIG.TOTAL_CHARACTERS} (${((allCharacters.length / CONFIG.TOTAL_CHARACTERS) * 100).toFixed(1)}%)`);
  }
  
  // Generate collection statistics
  const stats = generateCollectionStats(allCharacters);
  const statsPath = path.join(CONFIG.OUTPUT_DIR, 'collection-stats.json');
  fs.writeFileSync(statsPath, JSON.stringify(stats, null, 2));
  
  // Generate collection metadata
  const collectionMetadata = {
    name: "Mafia NFT Characters",
    description: "A collection of 10,000 unique mafia characters living in the Neo-Crypto City underworld.",
    image: "https://assets.mafianft.com/collection.png",
    external_url: "https://mafianft.com",
    seller_fee_basis_points: 500,
    fee_recipient: "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
  };
  
  const collectionPath = path.join(CONFIG.METADATA_DIR, 'collection.json');
  fs.writeFileSync(collectionPath, JSON.stringify(collectionMetadata, null, 2));
  
  console.log('\n🎉 AI Art Generation Complete!');
  console.log('📊 GENERATION SUMMARY');
  console.log('='.repeat(50));
  console.log(`✅ Total characters: ${allCharacters.length}`);
  console.log(`📁 Images: ${CONFIG.OUTPUT_DIR}/characters/`);
  console.log(`📄 Metadata: ${CONFIG.METADATA_DIR}/characters/`);
  console.log(`📈 Stats: ${statsPath}`);
  console.log(`🏷️  Collection: ${collectionPath}`);
  
  console.log('\n🎨 Rarity Distribution:');
  Object.entries(stats.rarityDistribution).forEach(([rarity, count]) => {
    const percentage = ((count / allCharacters.length) * 100).toFixed(1);
    console.log(`  ${rarity}: ${count} (${percentage}%)`);
  });
  
  console.log('\n🚀 Next Steps:');
  console.log('1. Replace placeholder art with actual AI-generated images');
  console.log('2. Upload images and metadata to IPFS/Arweave');
  console.log('3. Update metadata URIs with permanent storage links');
  console.log('4. Deploy NFT collection to Solana');
}

// CLI interface
if (require.main === module) {
  const args = process.argv.slice(2);
  
  if (args.includes('--help') || args.includes('-h')) {
    console.log(`
Mafia NFT AI Art Generator

Usage: node generate-ai-art.js [options]

Options:
  --count <number>    Number of characters to generate (default: ${CONFIG.TOTAL_CHARACTERS})
  --batch <number>    Batch size (default: ${CONFIG.BATCH_SIZE})
  --output <path>     Output directory (default: ${CONFIG.OUTPUT_DIR})
  --help, -h          Show this help message

Examples:
  node generate-ai-art.js --count 1000 --batch 50
  node generate-ai-art.js --output ./custom-assets
`);
    process.exit(0);
  }
  
  // Parse command line arguments
  const countIndex = args.indexOf('--count');
  if (countIndex !== -1 && args[countIndex + 1]) {
    CONFIG.TOTAL_CHARACTERS = parseInt(args[countIndex + 1]);
  }
  
  const batchIndex = args.indexOf('--batch');
  if (batchIndex !== -1 && args[batchIndex + 1]) {
    CONFIG.BATCH_SIZE = parseInt(args[batchIndex + 1]);
  }
  
  const outputIndex = args.indexOf('--output');
  if (outputIndex !== -1 && args[outputIndex + 1]) {
    CONFIG.OUTPUT_DIR = args[outputIndex + 1];
    CONFIG.METADATA_DIR = path.join(CONFIG.OUTPUT_DIR, 'metadata');
  }
  
  generateAllArt().catch(console.error);
}

module.exports = {
  generateAllArt,
  generateCharacterBatch,
  CONFIG
};
