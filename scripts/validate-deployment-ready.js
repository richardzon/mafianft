#!/usr/bin/env node

/**
 * Validate Deployment Readiness for Mafia NFT
 * This script checks if all components are ready for deployment
 */

const fs = require('fs');
const path = require('path');

console.log('🔍 Validating Deployment Readiness for Mafia NFT...\n');

let allChecksPass = true;

function checkFailed(message) {
  console.log(`❌ ${message}`);
  allChecksPass = false;
}

function checkPassed(message) {
  console.log(`✅ ${message}`);
}

function checkWarning(message) {
  console.log(`⚠️  ${message}`);
}

// Check 1: Smart Contract Structure
console.log('📋 Checking Smart Contract Structure...');

const requiredPrograms = [
  'character-nft',
  'mob-token',
  'fam-token', 
  'item-vault',
  'turf-control',
  'game-treasury'
];

const programsDir = 'programs';
if (!fs.existsSync(programsDir)) {
  checkFailed('Programs directory missing');
} else {
  checkPassed('Programs directory exists');
  
  for (const program of requiredPrograms) {
    const programPath = path.join(programsDir, program);
    const libPath = path.join(programPath, 'src', 'lib.rs');
    const cargoPath = path.join(programPath, 'Cargo.toml');
    
    if (!fs.existsSync(programPath)) {
      checkFailed(`Program ${program} directory missing`);
    } else if (!fs.existsSync(libPath)) {
      checkFailed(`Program ${program} lib.rs missing`);
    } else if (!fs.existsSync(cargoPath)) {
      checkFailed(`Program ${program} Cargo.toml missing`);
    } else {
      // Check if lib.rs has required content
      const libContent = fs.readFileSync(libPath, 'utf8');
      if (!libContent.includes('declare_id!') || !libContent.includes('#[program]')) {
        checkFailed(`Program ${program} missing required Anchor structure`);
      } else {
        checkPassed(`Program ${program} structure valid`);
      }
    }
  }
}

// Check 2: Anchor Configuration
console.log('\n📦 Checking Anchor Configuration...');

if (!fs.existsSync('Anchor.toml')) {
  checkFailed('Anchor.toml missing');
} else {
  const anchorContent = fs.readFileSync('Anchor.toml', 'utf8');
  
  // Check workspace members
  let missingPrograms = 0;
  for (const program of requiredPrograms) {
    if (!anchorContent.includes(`programs/${program}`)) {
      checkFailed(`Program ${program} not in Anchor workspace`);
      missingPrograms++;
    }
  }
  
  if (missingPrograms === 0) {
    checkPassed('All programs in Anchor workspace');
  }
  
  // Check program IDs are valid base58
  const programIdRegex = /= "([A-Za-z0-9]{32,44})"/g;
  const matches = anchorContent.match(programIdRegex);
  if (matches && matches.length >= requiredPrograms.length) {
    checkPassed('Program IDs appear to be valid base58');
  } else {
    checkWarning('Program IDs may need updating after deployment');
  }
}

// Check 3: Package Dependencies
console.log('\n📦 Checking Package Dependencies...');

if (!fs.existsSync('package.json')) {
  checkFailed('package.json missing');
} else {
  const packageContent = fs.readFileSync('package.json', 'utf8');
  const packageJson = JSON.parse(packageContent);
  
  const requiredDeps = [
    '@coral-xyz/anchor',
    '@solana/web3.js',
    '@solana/spl-token'
  ];
  
  for (const dep of requiredDeps) {
    if (!packageJson.dependencies || !packageJson.dependencies[dep]) {
      checkFailed(`Missing dependency: ${dep}`);
    } else {
      checkPassed(`Dependency ${dep} found`);
    }
  }
}

// Check 4: Deployment Scripts
console.log('\n🚀 Checking Deployment Scripts...');

const requiredScripts = [
  'scripts/deploy-to-devnet.sh',
  'scripts/initialize-programs.js',
  'scripts/generate-ai-art.js',
  'scripts/validate-programs.js'
];

for (const script of requiredScripts) {
  if (!fs.existsSync(script)) {
    checkFailed(`Script missing: ${script}`);
  } else {
    // Check if script is executable
    try {
      const stats = fs.statSync(script);
      if (stats.mode & parseInt('111', 8)) {
        checkPassed(`Script ${script} exists and is executable`);
      } else {
        checkWarning(`Script ${script} exists but may not be executable`);
      }
    } catch (error) {
      checkWarning(`Could not check permissions for ${script}`);
    }
  }
}

// Check 5: Unity Client Structure
console.log('\n🎮 Checking Unity Client Structure...');

const unityDirs = [
  'app/Assets/Scripts/Blockchain',
  'app/Assets/Scripts/Game',
  'app/Assets/Scripts/UI',
  'app/Assets/Scripts/Utils'
];

for (const dir of unityDirs) {
  if (!fs.existsSync(dir)) {
    checkFailed(`Unity directory missing: ${dir}`);
  } else {
    checkPassed(`Unity directory exists: ${dir}`);
  }
}

// Check Unity constants file
const constantsPath = 'app/Assets/Scripts/Utils/Constants.cs';
if (!fs.existsSync(constantsPath)) {
  checkFailed('Unity Constants.cs missing');
} else {
  const constantsContent = fs.readFileSync(constantsPath, 'utf8');
  if (constantsContent.includes('CHARACTER_NFT_PROGRAM_ID') && 
      constantsContent.includes('MOB_TOKEN_PROGRAM_ID')) {
    checkPassed('Unity Constants.cs has program ID placeholders');
  } else {
    checkFailed('Unity Constants.cs missing program ID constants');
  }
}

// Check 6: Backend API Structure
console.log('\n🌐 Checking Backend API Structure...');

const backendDirs = [
  'backend/src/auth',
  'backend/src/game',
  'backend/src/economy',
  'backend/src/dao'
];

for (const dir of backendDirs) {
  if (!fs.existsSync(dir)) {
    checkWarning(`Backend directory missing: ${dir} (optional for MVP)`);
  } else {
    checkPassed(`Backend directory exists: ${dir}`);
  }
}

if (!fs.existsSync('backend/package.json')) {
  checkWarning('Backend package.json missing (optional for MVP)');
} else {
  checkPassed('Backend package.json exists');
}

// Check 7: Documentation
console.log('\n📚 Checking Documentation...');

const requiredDocs = [
  'README.md',
  'docs/deployment-guide.md',
  'docs/fast-launch-plan.md',
  'DEPLOYMENT_STATUS.md'
];

for (const doc of requiredDocs) {
  if (!fs.existsSync(doc)) {
    checkWarning(`Documentation missing: ${doc}`);
  } else {
    checkPassed(`Documentation exists: ${doc}`);
  }
}

// Check 8: AI Art Generation
console.log('\n🎨 Checking AI Art Generation Setup...');

const artScript = 'scripts/generate-ai-art.js';
if (!fs.existsSync(artScript)) {
  checkFailed('AI art generation script missing');
} else {
  const artContent = fs.readFileSync(artScript, 'utf8');
  if (artContent.includes('generateCharacterTraits') && 
      artContent.includes('RARITY_DISTRIBUTION')) {
    checkPassed('AI art generation script has required functions');
  } else {
    checkFailed('AI art generation script incomplete');
  }
}

// Check 9: Test Framework
console.log('\n🧪 Checking Test Framework...');

if (!fs.existsSync('tests')) {
  checkWarning('Tests directory missing');
} else {
  const testFiles = fs.readdirSync('tests');
  if (testFiles.length > 0) {
    checkPassed(`Found ${testFiles.length} test file(s)`);
  } else {
    checkWarning('Tests directory empty');
  }
}

// Check 10: Environment Configuration
console.log('\n⚙️  Checking Environment Configuration...');

// Check for example environment files
const envFiles = [
  '.env.example',
  'backend/.env.example'
];

for (const envFile of envFiles) {
  if (fs.existsSync(envFile)) {
    checkPassed(`Environment template exists: ${envFile}`);
  } else {
    checkWarning(`Environment template missing: ${envFile}`);
  }
}

// Final Summary
console.log('\n📊 DEPLOYMENT READINESS SUMMARY');
console.log('='.repeat(50));

if (allChecksPass) {
  console.log('🎉 ALL CRITICAL CHECKS PASSED!');
  console.log('✅ Ready for deployment to devnet');
  console.log('\n🚀 Next Steps:');
  console.log('1. Install Solana CLI: curl -sSfL https://release.solana.com/stable/install | sh');
  console.log('2. Install Anchor CLI: npm install -g @coral-xyz/anchor-cli@0.29.0');
  console.log('3. Run deployment: ./scripts/deploy-to-devnet.sh');
  console.log('4. Initialize programs: node scripts/initialize-programs.js devnet');
  console.log('5. Generate art: node scripts/generate-ai-art.js --count 1000');
} else {
  console.log('❌ SOME CHECKS FAILED');
  console.log('🔧 Please fix the issues above before deploying');
  console.log('\n💡 Most issues can be resolved by:');
  console.log('1. Running: npm install');
  console.log('2. Ensuring all required files are present');
  console.log('3. Checking file permissions for scripts');
}

console.log('\n📖 For detailed deployment instructions, see: docs/deployment-guide.md');

process.exit(allChecksPass ? 0 : 1);
