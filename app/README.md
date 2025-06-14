# Mafia NFT Unity Client

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
