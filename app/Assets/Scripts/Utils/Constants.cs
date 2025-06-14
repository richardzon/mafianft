namespace MafiaNFT.Utils
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
}