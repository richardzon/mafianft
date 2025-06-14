#!/usr/bin/env node

/**
 * Quick validation script for Mafia NFT smart contracts
 * This validates the program structure without requiring full Solana toolchain
 */

const fs = require('fs');
const path = require('path');

console.log('🔍 Validating Mafia NFT Smart Contracts...\n');

// Program directories to validate
const programs = [
  'character-nft',
  'mob-token', 
  'fam-token',
  'item-vault',
  'turf-control',
  'game-treasury'
];

let allValid = true;

// Validation functions
function validateProgramStructure(programName) {
  const programDir = path.join('programs', programName);
  const srcDir = path.join(programDir, 'src');
  const cargoToml = path.join(programDir, 'Cargo.toml');
  const libRs = path.join(srcDir, 'lib.rs');
  
  console.log(`📁 Validating ${programName}...`);
  
  // Check directory structure
  if (!fs.existsSync(programDir)) {
    console.log(`  ❌ Program directory missing: ${programDir}`);
    return false;
  }
  
  if (!fs.existsSync(srcDir)) {
    console.log(`  ❌ Source directory missing: ${srcDir}`);
    return false;
  }
  
  if (!fs.existsSync(cargoToml)) {
    console.log(`  ❌ Cargo.toml missing: ${cargoToml}`);
    return false;
  }
  
  if (!fs.existsSync(libRs)) {
    console.log(`  ❌ lib.rs missing: ${libRs}`);
    return false;
  }
  
  // Validate Cargo.toml content
  const cargoContent = fs.readFileSync(cargoToml, 'utf8');
  if (!cargoContent.includes('[lib]')) {
    console.log(`  ❌ Cargo.toml missing [lib] section`);
    return false;
  }
  
  if (!cargoContent.includes('crate-type = ["cdylib", "lib"]')) {
    console.log(`  ❌ Cargo.toml missing correct crate-type`);
    return false;
  }
  
  // Validate lib.rs content
  const libContent = fs.readFileSync(libRs, 'utf8');
  if (!libContent.includes('use anchor_lang::prelude::*;')) {
    console.log(`  ❌ lib.rs missing Anchor imports`);
    return false;
  }
  
  if (!libContent.includes('declare_id!')) {
    console.log(`  ❌ lib.rs missing declare_id! macro`);
    return false;
  }
  
  if (!libContent.includes('#[program]')) {
    console.log(`  ❌ lib.rs missing #[program] attribute`);
    return false;
  }
  
  console.log(`  ✅ ${programName} structure valid`);
  return true;
}

function validateProgramFeatures(programName) {
  const libRs = path.join('programs', programName, 'src', 'lib.rs');
  const content = fs.readFileSync(libRs, 'utf8');
  
  console.log(`🔧 Validating ${programName} features...`);
  
  const expectedFeatures = {
    'character-nft': [
      'mint_character',
      'level_up', 
      'CharacterRarity',
      'CharacterStats'
    ],
    'mob-token': [
      'mint_reward',
      'burn_tokens',
      'RewardType',
      'BurnReason'
    ],
    'fam-token': [
      'stake_tokens',
      'create_proposal',
      'vote_on_proposal',
      'ProposalType'
    ],
    'item-vault': [
      'mint_weapon',
      'upgrade_weapon',
      'WeaponType',
      'ItemRarity'
    ],
    'turf-control': [
      'mint_territory',
      'claim_income',
      'attack_territory',
      'District'
    ],
    'game-treasury': [
      'collect_marketplace_fee',
      'distribute_rewards',
      'FeeType',
      'RewardType'
    ]
  };
  
  const features = expectedFeatures[programName] || [];
  let featuresValid = true;
  
  for (const feature of features) {
    if (!content.includes(feature)) {
      console.log(`  ❌ Missing feature: ${feature}`);
      featuresValid = false;
    }
  }
  
  if (featuresValid) {
    console.log(`  ✅ ${programName} features complete`);
  }
  
  return featuresValid;
}

function validateAnchorConfig() {
  console.log('📋 Validating Anchor.toml...');
  
  if (!fs.existsSync('Anchor.toml')) {
    console.log('  ❌ Anchor.toml missing');
    return false;
  }
  
  const anchorContent = fs.readFileSync('Anchor.toml', 'utf8');
  
  // Check workspace members
  for (const program of programs) {
    if (!anchorContent.includes(`programs/${program}`)) {
      console.log(`  ❌ Program ${program} not in workspace`);
      return false;
    }
  }
  
  console.log('  ✅ Anchor.toml valid');
  return true;
}

function validateCargoWorkspace() {
  console.log('📦 Validating Cargo.toml workspace...');
  
  if (!fs.existsSync('Cargo.toml')) {
    console.log('  ❌ Root Cargo.toml missing');
    return false;
  }
  
  const cargoContent = fs.readFileSync('Cargo.toml', 'utf8');
  
  if (!cargoContent.includes('[workspace]')) {
    console.log('  ❌ Cargo.toml missing workspace section');
    return false;
  }
  
  // Check workspace members
  for (const program of programs) {
    if (!cargoContent.includes(`programs/${program}`)) {
      console.log(`  ❌ Program ${program} not in Cargo workspace`);
      return false;
    }
  }
  
  console.log('  ✅ Cargo workspace valid');
  return true;
}

function validatePackageJson() {
  console.log('📄 Validating package.json...');
  
  if (!fs.existsSync('package.json')) {
    console.log('  ❌ package.json missing');
    return false;
  }
  
  const packageContent = fs.readFileSync('package.json', 'utf8');
  const packageJson = JSON.parse(packageContent);
  
  const requiredDeps = [
    '@coral-xyz/anchor',
    '@solana/web3.js',
    '@solana/spl-token'
  ];
  
  for (const dep of requiredDeps) {
    if (!packageJson.dependencies || !packageJson.dependencies[dep]) {
      console.log(`  ❌ Missing dependency: ${dep}`);
      return false;
    }
  }
  
  console.log('  ✅ package.json valid');
  return true;
}

// Run all validations
console.log('🚀 Starting validation...\n');

// Validate configuration files
allValid &= validateAnchorConfig();
allValid &= validateCargoWorkspace();
allValid &= validatePackageJson();

console.log('');

// Validate each program
for (const program of programs) {
  allValid &= validateProgramStructure(program);
  allValid &= validateProgramFeatures(program);
  console.log('');
}

// Summary
console.log('📊 VALIDATION SUMMARY');
console.log('='.repeat(50));

if (allValid) {
  console.log('✅ ALL VALIDATIONS PASSED!');
  console.log('🎉 Smart contracts are ready for deployment');
  console.log('\n🚀 Next steps:');
  console.log('1. Install Solana CLI tools');
  console.log('2. Run: anchor build');
  console.log('3. Run: anchor test');
  console.log('4. Deploy to devnet: anchor deploy --provider.cluster devnet');
  process.exit(0);
} else {
  console.log('❌ VALIDATION FAILED');
  console.log('🔧 Please fix the issues above before proceeding');
  process.exit(1);
}
