#!/usr/bin/env node

/**
 * Simple Unity Client Setup Script for Mafia NFT
 */

const fs = require('fs');
const path = require('path');

console.log('🎮 Setting up Unity Client for Mafia NFT...\n');

// Create Unity project directories
function createUnityDirectories() {
  console.log('📁 Creating Unity project directories...');
  
  const dirs = [
    'app/Assets/Scripts/Blockchain',
    'app/Assets/Scripts/Game', 
    'app/Assets/Scripts/UI',
    'app/Assets/Scripts/Utils',
    'app/Assets/Scenes',
    'app/Assets/Prefabs',
    'app/Assets/Materials',
    'app/Assets/Textures',
    'app/Assets/Audio',
    'app/Packages'
  ];
  
  dirs.forEach(dir => {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
      console.log(`  ✅ Created: ${dir}`);
    }
  });
}

// Create Unity package manifest
function createPackageManifest() {
  console.log('\n📦 Creating Unity package manifest...');
  
  const manifest = {
    dependencies: {
      "com.unity.ugui": "1.0.0",
      "com.unity.textmeshpro": "3.0.6",
      "com.unity.render-pipelines.universal": "14.0.8",
      "com.unity.addressables": "1.21.14",
      "com.unity.nuget.newtonsoft-json": "3.2.1"
    }
  };
  
  const manifestPath = 'app/Packages/manifest.json';
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
  console.log(`  ✅ Created: ${manifestPath}`);
}

// Create constants file
function createConstantsScript() {
  console.log('\n📋 Creating Constants script...');
  
  const constantsScript = `namespace MafiaNFT.Utils
{
    public static class Constants
    {
        // Network Configuration
        public const string DEVNET_RPC_URL = "https://api.devnet.solana.com";
        public const string MAINNET_RPC_URL = "https://api.mainnet-beta.solana.com";
        
        // Program IDs
        public const string CHARACTER_NFT_PROGRAM_ID = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";
        public const string MOB_TOKEN_PROGRAM_ID = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnV";
        public const string FAM_TOKEN_PROGRAM_ID = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnW";
        public const string ITEM_VAULT_PROGRAM_ID = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT";
        public const string TURF_CONTROL_PROGRAM_ID = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnU";
        public const string GAME_TREASURY_PROGRAM_ID = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnX";
        
        // Game Configuration
        public const int MAX_ENERGY = 100;
        public const int ENERGY_REGEN_TIME = 360; // 6 minutes in seconds
        public const int MAX_CHARACTER_LEVEL = 100;
        
        // Economy
        public const long MOB_DECIMALS = 1_000_000_000; // 9 decimals
        public const long FAM_DECIMALS = 1_000_000; // 6 decimals
        public const long ENERGY_REFILL_COST = 100_000_000; // 0.1 MOB
        
        // Character Rarities
        public enum CharacterRarity
        {
            Common = 0,
            Uncommon = 1,
            Rare = 2,
            Epic = 3,
            Legendary = 4,
            Mythic = 5
        }
        
        // Weapon Types
        public enum WeaponType
        {
            Pistol = 0,
            Rifle = 1,
            Shotgun = 2,
            SMG = 3,
            Sniper = 4,
            Knife = 5,
            Bat = 6,
            Grenade = 7
        }
    }
}`;
  
  const scriptPath = 'app/Assets/Scripts/Utils/Constants.cs';
  fs.writeFileSync(scriptPath, constantsScript);
  console.log(`  ✅ Created: ${scriptPath}`);
}

// Create simple game manager
function createGameManagerScript() {
  console.log('\n🎮 Creating GameManager script...');
  
  const gameManagerScript = `using UnityEngine;

namespace MafiaNFT.Game
{
    public class GameManager : MonoBehaviour
    {
        public static GameManager Instance { get; private set; }
        
        [Header("Game Configuration")]
        [SerializeField] private bool autoConnectWallet = true;
        [SerializeField] private string gameVersion = "1.0.0";
        
        public bool IsGameInitialized { get; private set; }
        
        private void Awake()
        {
            if (Instance == null)
            {
                Instance = this;
                DontDestroyOnLoad(gameObject);
                InitializeGame();
            }
            else
            {
                Destroy(gameObject);
            }
        }
        
        private void InitializeGame()
        {
            Debug.Log("Initializing Mafia NFT Game v" + gameVersion);
            IsGameInitialized = true;
        }
        
        public void StartGame()
        {
            Debug.Log("Starting game...");
        }
        
        public void QuitGame()
        {
            Debug.Log("Quitting game...");
            Application.Quit();
        }
    }
}`;
  
  const scriptPath = 'app/Assets/Scripts/Game/GameManager.cs';
  fs.writeFileSync(scriptPath, gameManagerScript);
  console.log(`  ✅ Created: ${scriptPath}`);
}

// Create README
function createUnityReadme() {
  console.log('\n📖 Creating Unity setup README...');
  
  const readme = `# Mafia NFT Unity Client

## Setup Instructions

### 1. Unity Version
- Required: Unity 2022.3 LTS or later
- Render Pipeline: Universal Render Pipeline (URP)

### 2. Solana Unity SDK
Install the Solana Unity SDK:
1. Download from: https://github.com/allartprotocol/unity-solana-wallet
2. Import the package into your Unity project
3. Configure wallet adapters

### 3. Project Structure
- Assets/Scripts/Blockchain/ - Solana integration
- Assets/Scripts/Game/ - Core game logic  
- Assets/Scripts/UI/ - User interface
- Assets/Scripts/Utils/ - Utilities and constants

### 4. Configuration
Update program IDs in Constants.cs after deploying smart contracts.

### 5. Build Targets
- Android: API Level 24+, ARM64, IL2CPP
- iOS: iOS 12.0+, ARM64, IL2CPP
- WebGL: Brotli compression, 512MB memory

### 6. Next Steps
1. Open Unity Hub and add the app/ folder as a project
2. Install Solana Unity SDK package
3. Configure wallet adapters
4. Test basic wallet connection
5. Build and deploy to target platforms

## Support
- Discord: https://discord.gg/mafianft
- GitHub: https://github.com/richardzon/mafianft
`;
  
  const readmePath = 'app/README.md';
  fs.writeFileSync(readmePath, readme);
  console.log(`  ✅ Created: ${readmePath}`);
}

// Main setup function
function setupUnityClient() {
  console.log('🚀 Starting Unity Client Setup...\n');
  
  createUnityDirectories();
  createPackageManifest();
  createConstantsScript();
  createGameManagerScript();
  createUnityReadme();
  
  console.log('\n🎉 Unity Client Setup Complete!');
  console.log('📊 SETUP SUMMARY');
  console.log('='.repeat(50));
  console.log('✅ Project structure created');
  console.log('✅ Package manifest configured');
  console.log('✅ Core scripts generated');
  console.log('✅ Documentation created');
  
  console.log('\n🚀 Next Steps:');
  console.log('1. Open Unity Hub and add the app/ folder as a project');
  console.log('2. Install Solana Unity SDK package');
  console.log('3. Configure wallet adapters');
  console.log('4. Test basic wallet connection');
  
  console.log('\n📖 See app/README.md for detailed setup instructions');
}

// Run setup if called directly
if (require.main === module) {
  setupUnityClient();
}

module.exports = { setupUnityClient };
