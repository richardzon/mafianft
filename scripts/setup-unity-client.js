#!/usr/bin/env node

/**
 * Unity Client Setup Script for Mafia NFT
 * Creates the basic Unity project structure and configuration files
 */

const fs = require('fs');
const path = require('path');

console.log('🎮 Setting up Unity Client for Mafia NFT...\n');

// Unity project structure
const UNITY_STRUCTURE = {
  'app/Assets/Scripts/Blockchain': [
    'SolanaWalletManager.cs',
    'TransactionManager.cs',
    'NFTManager.cs',
    'TokenManager.cs'
  ],
  'app/Assets/Scripts/Game': [
    'GameManager.cs',
    'CharacterController.cs',
    'MissionManager.cs',
    'InventoryManager.cs',
    'TerritoryManager.cs'
  ],
  'app/Assets/Scripts/UI': [
    'MainMenuUI.cs',
    'WalletUI.cs',
    'CharacterUI.cs',
    'InventoryUI.cs',
    'MissionUI.cs',
    'TerritoryUI.cs'
  ],
  'app/Assets/Scripts/Utils': [
    'Constants.cs',
    'Extensions.cs',
    'Helpers.cs'
  ],
  'app/Assets/Scenes': [
    'MainMenu.unity',
    'Game.unity',
    'Character.unity'
  ],
  'app/Assets/Prefabs/UI': [],
  'app/Assets/Prefabs/Characters': [],
  'app/Assets/Materials': [],
  'app/Assets/Textures': [],
  'app/Assets/Audio': []
};

// Create Unity project directories
function createUnityDirectories() {
  console.log('📁 Creating Unity project directories...');
  
  Object.keys(UNITY_STRUCTURE).forEach(dir => {
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
      "com.unity.netcode.gameobjects": "1.5.2",
      "com.unity.services.core": "1.10.1",
      "com.unity.services.authentication": "2.6.1",
      "com.unity.nuget.newtonsoft-json": "3.2.1"
    },
    scopedRegistries: [
      {
        name: "Unity NuGet",
        url: "https://unitynuget-registry.azurewebsites.net",
        scopes: ["org.nuget"]
      }
    ]
  };
  
  const manifestPath = 'app/Packages/manifest.json';
  const packagesDir = path.dirname(manifestPath);
  
  if (!fs.existsSync(packagesDir)) {
    fs.mkdirSync(packagesDir, { recursive: true });
  }
  
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
  console.log(`  ✅ Created: ${manifestPath}`);
}

// Create Unity project settings
function createProjectSettings() {
  console.log('\n⚙️  Creating Unity project settings...');
  
  const projectSettings = {
    PlayerSettings: {
      companyName: "Mafia NFT",
      productName: "Mafia NFT Game",
      bundleVersion: "1.0.0",
      buildNumber: {
        iOS: "1",
        Android: "1"
      },
      applicationIdentifier: {
        Android: "com.mafianft.game",
        iOS: "com.mafianft.game"
      }
    },
    GraphicsSettings: {
      renderPipeline: "UniversalRenderPipeline"
    },
    QualitySettings: {
      levels: ["Low", "Medium", "High", "Ultra"]
    }
  };
  
  const settingsDir = 'app/ProjectSettings';
  if (!fs.existsSync(settingsDir)) {
    fs.mkdirSync(settingsDir, { recursive: true });
  }
  
  fs.writeFileSync(
    path.join(settingsDir, 'ProjectSettings.json'),
    JSON.stringify(projectSettings, null, 2)
  );
  console.log(`  ✅ Created: ProjectSettings.json`);
}

// Create basic game manager script
function createGameManagerScript() {
  console.log('\n🎮 Creating GameManager script...');
  
  const gameManagerScript = `using UnityEngine;
using MafiaNFT.Blockchain;

namespace MafiaNFT.Game
{
    /// <summary>
    /// Main game manager that coordinates all game systems
    /// </summary>
    public class GameManager : MonoBehaviour
    {
        [Header("Game Configuration")]
        [SerializeField] private bool autoConnectWallet = true;
        [SerializeField] private string gameVersion = "1.0.0";
        
        [Header("Managers")]
        [SerializeField] private CharacterController characterController;
        [SerializeField] private MissionManager missionManager;
        [SerializeField] private InventoryManager inventoryManager;
        [SerializeField] private TerritoryManager territoryManager;
        
        // Singleton instance
        public static GameManager Instance { get; private set; }
        
        // Game state
        public bool IsGameInitialized { get; private set; }
        public bool IsWalletConnected => SolanaWalletManager.Instance?.IsConnected ?? false;
        
        // Events
        public static event System.Action OnGameInitialized;
        public static event System.Action OnGameStarted;
        public static event System.Action OnGamePaused;
        
        private void Awake()
        {
            // Singleton pattern
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
        
        private void Start()
        {
            if (autoConnectWallet)
            {
                StartCoroutine(AutoConnectWallet());
            }
        }
        
        private void InitializeGame()
        {
            Debug.Log($"[GameManager] Initializing Mafia NFT Game v{gameVersion}");
            
            // Subscribe to wallet events
            SolanaWalletManager.OnWalletConnected += OnWalletConnected;
            SolanaWalletManager.OnWalletDisconnected += OnWalletDisconnected;
            
            IsGameInitialized = true;
            OnGameInitialized?.Invoke();
            
            Debug.Log("[GameManager] Game initialized successfully");
        }
        
        private System.Collections.IEnumerator AutoConnectWallet()
        {
            yield return new WaitForSeconds(1f);
            
            if (!IsWalletConnected)
            {
                Debug.Log("[GameManager] Attempting auto-connect to wallet...");
                // Auto-connect logic would go here
            }
        }
        
        private void OnWalletConnected(Solana.Unity.Wallet.Account account)
        {
            Debug.Log($"[GameManager] Wallet connected: {account.PublicKey.Key}");
            StartGame();
        }
        
        private void OnWalletDisconnected()
        {
            Debug.Log("[GameManager] Wallet disconnected");
            PauseGame();
        }
        
        public void StartGame()
        {
            if (!IsGameInitialized)
            {
                Debug.LogError("[GameManager] Cannot start game - not initialized");
                return;
            }
            
            if (!IsWalletConnected)
            {
                Debug.LogError("[GameManager] Cannot start game - wallet not connected");
                return;
            }
            
            Debug.Log("[GameManager] Starting game...");
            OnGameStarted?.Invoke();
        }
        
        public void PauseGame()
        {
            Debug.Log("[GameManager] Game paused");
            OnGamePaused?.Invoke();
        }
        
        public void QuitGame()
        {
            Debug.Log("[GameManager] Quitting game...");
            
            #if UNITY_EDITOR
                UnityEditor.EditorApplication.isPlaying = false;
            #else
                Application.Quit();
            #endif
        }
        
        private void OnDestroy()
        {
            // Unsubscribe from events
            SolanaWalletManager.OnWalletConnected -= OnWalletConnected;
            SolanaWalletManager.OnWalletDisconnected -= OnWalletDisconnected;
        }
    }
}`;
  
  const scriptPath = 'app/Assets/Scripts/Game/GameManager.cs';
  fs.writeFileSync(scriptPath, gameManagerScript);
  console.log(`  ✅ Created: ${scriptPath}`);
}

// Create constants file
function createConstantsScript() {
  console.log('\n📋 Creating Constants script...');
  
  const constantsScript = `namespace MafiaNFT.Utils
{
    /// <summary>
    /// Game constants and configuration values
    /// </summary>
    public static class Constants
    {
        // Network Configuration
        public const string DEVNET_RPC_URL = "https://api.devnet.solana.com";
        public const string MAINNET_RPC_URL = "https://api.mainnet-beta.solana.com";
        
        // Program IDs (will be updated after deployment)
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
        public const int MAX_WEAPON_UPGRADE_LEVEL = 10;
        
        // UI Configuration
        public const float TRANSACTION_TIMEOUT = 30f; // seconds
        public const float BALANCE_UPDATE_INTERVAL = 30f; // seconds
        public const int MAX_TRANSACTION_HISTORY = 100;
        
        // API Endpoints
        public const string API_BASE_URL_DEV = "https://devnet-api.mafianft.com";
        public const string API_BASE_URL_PROD = "https://api.mafianft.com";
        
        // Asset URLs
        public const string METADATA_BASE_URL = "https://assets.mafianft.com";
        public const string IPFS_GATEWAY = "https://gateway.pinata.cloud/ipfs/";
        
        // Game Balance
        public static class Economy
        {
            public const long MOB_DECIMALS = 1_000_000_000; // 9 decimals
            public const long FAM_DECIMALS = 1_000_000; // 6 decimals
            
            public const long ENERGY_REFILL_COST = 100_000_000; // 0.1 MOB
            public const long CHARACTER_RESPAWN_COST = 500_000_000; // 0.5 MOB
            public const long WEAPON_UPGRADE_BASE_COST = 100_000_000; // 0.1 MOB
        }
        
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
        
        // Districts
        public enum District
        {
            Downtown = 0,
            Industrial = 1,
            Financial = 2,
            Harbor = 3
        }
    }
}`;
  
  const scriptPath = 'app/Assets/Scripts/Utils/Constants.cs';
  fs.writeFileSync(scriptPath, constantsScript);
  console.log(`  ✅ Created: ${scriptPath}`);
}

// Create basic UI manager
function createUIManagerScript() {
  console.log('\n🖼️  Creating UI Manager script...');
  
  const uiManagerScript = `using UnityEngine;
using UnityEngine.UI;
using TMPro;

namespace MafiaNFT.UI
{
    /// <summary>
    /// Main UI manager for the game interface
    /// </summary>
    public class MainMenuUI : MonoBehaviour
    {
        [Header("Wallet Connection")]
        [SerializeField] private Button connectWalletButton;
        [SerializeField] private Button disconnectWalletButton;
        [SerializeField] private TextMeshProUGUI walletAddressText;
        [SerializeField] private TextMeshProUGUI connectionStatusText;
        
        [Header("Token Balances")]
        [SerializeField] private TextMeshProUGUI solBalanceText;
        [SerializeField] private TextMeshProUGUI mobBalanceText;
        [SerializeField] private TextMeshProUGUI famBalanceText;
        
        [Header("Game Navigation")]
        [SerializeField] private Button playGameButton;
        [SerializeField] private Button characterButton;
        [SerializeField] private Button inventoryButton;
        [SerializeField] private Button territoryButton;
        [SerializeField] private Button settingsButton;
        
        [Header("Loading")]
        [SerializeField] private GameObject loadingPanel;
        [SerializeField] private TextMeshProUGUI loadingText;
        
        private void Start()
        {
            InitializeUI();
            SubscribeToEvents();
        }
        
        private void InitializeUI()
        {
            // Setup button listeners
            connectWalletButton?.onClick.AddListener(OnConnectWalletClicked);
            disconnectWalletButton?.onClick.AddListener(OnDisconnectWalletClicked);
            playGameButton?.onClick.AddListener(OnPlayGameClicked);
            characterButton?.onClick.AddListener(OnCharacterClicked);
            inventoryButton?.onClick.AddListener(OnInventoryClicked);
            territoryButton?.onClick.AddListener(OnTerritoryClicked);
            settingsButton?.onClick.AddListener(OnSettingsClicked);
            
            // Initial UI state
            UpdateWalletUI(false, "");
            UpdateBalanceUI(0, 0, 0);
            SetLoadingState(false, "");
        }
        
        private void SubscribeToEvents()
        {
            // Subscribe to wallet events
            Blockchain.SolanaWalletManager.OnWalletConnected += OnWalletConnected;
            Blockchain.SolanaWalletManager.OnWalletDisconnected += OnWalletDisconnected;
            Blockchain.SolanaWalletManager.OnSolBalanceUpdated += OnSolBalanceUpdated;
            Blockchain.SolanaWalletManager.OnMobBalanceUpdated += OnMobBalanceUpdated;
            Blockchain.SolanaWalletManager.OnFamBalanceUpdated += OnFamBalanceUpdated;
        }
        
        private async void OnConnectWalletClicked()
        {
            SetLoadingState(true, "Connecting to wallet...");
            
            try
            {
                var success = await Blockchain.SolanaWalletManager.Instance.ConnectWallet();
                if (!success)
                {
                    connectionStatusText.text = "Failed to connect wallet";
                    connectionStatusText.color = Color.red;
                }
            }
            catch (System.Exception ex)
            {
                Debug.LogError($"Wallet connection error: {ex.Message}");
                connectionStatusText.text = "Connection error";
                connectionStatusText.color = Color.red;
            }
            finally
            {
                SetLoadingState(false, "");
            }
        }
        
        private async void OnDisconnectWalletClicked()
        {
            await Blockchain.SolanaWalletManager.Instance.DisconnectWallet();
        }
        
        private void OnPlayGameClicked()
        {
            if (!Blockchain.SolanaWalletManager.Instance.IsConnected)
            {
                connectionStatusText.text = "Please connect wallet first";
                connectionStatusText.color = Color.yellow;
                return;
            }
            
            // Load game scene
            UnityEngine.SceneManagement.SceneManager.LoadScene("Game");
        }
        
        private void OnCharacterClicked()
        {
            // Open character management UI
            Debug.Log("Character management clicked");
        }
        
        private void OnInventoryClicked()
        {
            // Open inventory UI
            Debug.Log("Inventory clicked");
        }
        
        private void OnTerritoryClicked()
        {
            // Open territory management UI
            Debug.Log("Territory management clicked");
        }
        
        private void OnSettingsClicked()
        {
            // Open settings UI
            Debug.Log("Settings clicked");
        }
        
        private void OnWalletConnected(Solana.Unity.Wallet.Account account)
        {
            UpdateWalletUI(true, account.PublicKey.Key);
            connectionStatusText.text = "Wallet connected";
            connectionStatusText.color = Color.green;
        }
        
        private void OnWalletDisconnected()
        {
            UpdateWalletUI(false, "");
            UpdateBalanceUI(0, 0, 0);
            connectionStatusText.text = "Wallet disconnected";
            connectionStatusText.color = Color.gray;
        }
        
        private void OnSolBalanceUpdated(decimal balance)
        {
            if (solBalanceText != null)
            {
                solBalanceText.text = $"{balance:F4} SOL";
            }
        }
        
        private void OnMobBalanceUpdated(decimal balance)
        {
            if (mobBalanceText != null)
            {
                mobBalanceText.text = $"{balance:F2} MOB";
            }
        }
        
        private void OnFamBalanceUpdated(decimal balance)
        {
            if (famBalanceText != null)
            {
                famBalanceText.text = $"{balance:F2} FAM";
            }
        }
        
        private void UpdateWalletUI(bool isConnected, string address)
        {
            if (connectWalletButton != null)
                connectWalletButton.gameObject.SetActive(!isConnected);
            
            if (disconnectWalletButton != null)
                disconnectWalletButton.gameObject.SetActive(isConnected);
            
            if (walletAddressText != null)
            {
                walletAddressText.text = isConnected ? 
                    $"{address.Substring(0, 4)}...{address.Substring(address.Length - 4)}" : 
                    "Not connected";
            }
            
            if (playGameButton != null)
                playGameButton.interactable = isConnected;
        }
        
        private void UpdateBalanceUI(decimal sol, decimal mob, decimal fam)
        {
            if (solBalanceText != null)
                solBalanceText.text = $"{sol:F4} SOL";
            
            if (mobBalanceText != null)
                mobBalanceText.text = $"{mob:F2} MOB";
            
            if (famBalanceText != null)
                famBalanceText.text = $"{fam:F2} FAM";
        }
        
        private void SetLoadingState(bool isLoading, string message)
        {
            if (loadingPanel != null)
                loadingPanel.SetActive(isLoading);
            
            if (loadingText != null)
                loadingText.text = message;
        }
        
        private void OnDestroy()
        {
            // Unsubscribe from events
            Blockchain.SolanaWalletManager.OnWalletConnected -= OnWalletConnected;
            Blockchain.SolanaWalletManager.OnWalletDisconnected -= OnWalletDisconnected;
            Blockchain.SolanaWalletManager.OnSolBalanceUpdated -= OnSolBalanceUpdated;
            Blockchain.SolanaWalletManager.OnMobBalanceUpdated -= OnMobBalanceUpdated;
            Blockchain.SolanaWalletManager.OnFamBalanceUpdated -= OnFamBalanceUpdated;
        }
    }
}`;
  
  const scriptPath = 'app/Assets/Scripts/UI/MainMenuUI.cs';
  fs.writeFileSync(scriptPath, uiManagerScript);
  console.log(`  ✅ Created: ${scriptPath}`);
}

// Create README for Unity setup
function createUnityReadme() {
  console.log('\n📖 Creating Unity setup README...');
  
  const readme = `# Mafia NFT Unity Client

## Setup Instructions

### 1. Unity Version
- **Required**: Unity 2022.3 LTS or later
- **Render Pipeline**: Universal Render Pipeline (URP)

### 2. Package Dependencies
The following packages will be automatically installed via the Package Manager:
- Universal Render Pipeline
- TextMeshPro
- Addressables
- Netcode for GameObjects
- Unity Services Core
- Authentication
- Newtonsoft JSON

### 3. Solana Unity SDK
Install the Solana Unity SDK:
1. Download from: https://github.com/allartprotocol/unity-solana-wallet
2. Import the package into your Unity project
3. Configure wallet adapters in the SolanaWalletManager

### 4. Project Structure
\`\`\`
app/
├── Assets/
│   ├── Scripts/
│   │   ├── Blockchain/     # Solana integration
│   │   ├── Game/          # Core game logic
│   │   ├── UI/            # User interface
│   │   └── Utils/         # Utilities and constants
│   ├── Scenes/            # Game scenes
│   ├── Prefabs/           # Reusable game objects
│   ├── Materials/         # 3D materials
│   ├── Textures/          # 2D textures and sprites
│   └── Audio/             # Sound effects and music
└── Packages/              # Package dependencies
```

### 5. Build Configuration

#### Android
- **Minimum API Level**: 24 (Android 7.0)
- **Target API Level**: 33 (Android 13)
- **Scripting Backend**: IL2CPP
- **Architecture**: ARM64

#### iOS
- **Minimum iOS Version**: 12.0
- **Architecture**: ARM64
- **Scripting Backend**: IL2CPP

#### WebGL
- **Compression Format**: Brotli
- **Memory Size**: 512 MB
- **Enable Exceptions**: Explicitly Thrown Exceptions Only

### 6. Configuration

#### Program IDs
Update the program IDs in \`Constants.cs\` after deploying smart contracts:
\`\`\`csharp
public const string CHARACTER_NFT_PROGRAM_ID = "YOUR_PROGRAM_ID";
public const string MOB_TOKEN_PROGRAM_ID = "YOUR_PROGRAM_ID";
// ... etc
\`\`\`

#### Network Configuration
Set the appropriate RPC URLs for your target network:
- **Devnet**: https://api.devnet.solana.com
- **Mainnet**: https://api.mainnet-beta.solana.com

### 7. Testing

#### Play Mode Testing
1. Open the MainMenu scene
2. Enter Play Mode
3. Test wallet connection functionality
4. Verify UI responsiveness

#### Build Testing
1. Build for your target platform
2. Test on actual devices
3. Verify wallet integration works correctly
4. Test transaction signing flow

### 8. Deployment

#### Development Builds
- Enable Development Build
- Enable Script Debugging
- Connect Profiler for performance monitoring

#### Production Builds
- Disable Development Build
- Enable optimization settings
- Configure proper signing certificates

### 9. Troubleshooting

#### Common Issues
1. **Wallet Connection Fails**
   - Check network connectivity
   - Verify RPC URL is correct
   - Ensure wallet app is installed

2. **Transaction Errors**
   - Check account has sufficient SOL for fees
   - Verify program IDs are correct
   - Check transaction timeout settings

3. **Build Errors**
   - Ensure all dependencies are installed
   - Check platform-specific settings
   - Verify IL2CPP is configured correctly

### 10. Performance Optimization

#### Mobile Optimization
- Use texture compression
- Optimize mesh complexity
- Implement object pooling
- Use LOD (Level of Detail) systems

#### Memory Management
- Unload unused assets
- Use Addressables for large assets
- Implement proper garbage collection
- Monitor memory usage with Profiler

## Next Steps

1. Import Solana Unity SDK
2. Configure wallet adapters
3. Test basic wallet connection
4. Implement character minting UI
5. Add mission system interface
6. Integrate territory management
7. Add social features
8. Optimize for target platforms

## Support

For technical support and questions:
- Discord: https://discord.gg/mafianft
- Documentation: https://docs.mafianft.com
- GitHub Issues: https://github.com/richardzon/mafianft/issues
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
  createProjectSettings();
  createGameManagerScript();
  createConstantsScript();
  createUIManagerScript();
  createUnityReadme();
  
  console.log('\n🎉 Unity Client Setup Complete!');
  console.log('📊 SETUP SUMMARY');
  console.log('='.repeat(50));
  console.log('✅ Project structure created');
  console.log('✅ Package manifest configured');
  console.log('✅ Core scripts generated');
  console.log('✅ UI framework ready');
  console.log('✅ Documentation created');
  
  console.log('\n🚀 Next Steps:');
  console.log('1. Open Unity Hub and add the app/ folder as a project');
  console.log('2. Install Solana Unity SDK package');
  console.log('3. Configure wallet adapters');
  console.log('4. Test basic wallet connection');
  console.log('5. Build and deploy to target platforms');
  
  console.log('\n📖 See app/README.md for detailed setup instructions');
}

// Run setup if called directly
if (require.main === module) {
  setupUnityClient();
}

module.exports = { setupUnityClient };
