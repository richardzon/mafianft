# NFT Asset Matrix & Schema Design

## Overview
Mafia NFT utilizes Metaplex Certified Collections and SPL Token 2022 standards to create a comprehensive asset ecosystem. The design balances true ownership for valuable assets with cost-effective distribution for common loot.

## Asset Categories

### 1. Character NFTs (Genesis Collection)

**Collection Details:**
- **Supply**: 10,000 unique characters
- **Standard**: Non-fungible 1/1 (Metaplex Certified Collection)
- **Mint Authority**: Game treasury PDA
- **Update Authority**: Multisig controlled by development team

**In-Game Utility:**
- Avatar representation in all game modes
- Base stat modifiers (Strength, Intelligence, Charisma, Luck, Stealth)
- Unique backstories affecting mission availability
- Family lineage determining starting district access

**Rarity Distribution:**
- **Common (C)**: 40% - Basic street criminals
- **Uncommon (U)**: 30% - Experienced operators  
- **Rare (R)**: 20% - Skilled specialists
- **Epic (E)**: 8% - Veteran crime bosses
- **Legendary (L)**: 1.8% - Infamous crime lords
- **Mythic (M)**: 0.2% - Historical crime figures

**Upgrade Mechanics:**
- **Burn/Merge System**: Combine 3 same-tier characters + $MOB to create next tier
- **Stat Enhancement**: Spend $MOB to increase individual attributes (max +20 per stat)
- **Skill Unlocks**: Certain combinations unlock unique abilities
- **Prestige Evolution**: Mythic characters can be "retired" for permanent family bonuses

**Metadata Schema:**
```json
{
  "name": "Vincent 'The Shark' Torrino",
  "description": "A ruthless loan shark from the old country",
  "image": "https://assets.mafianft.com/characters/001.png",
  "attributes": [
    {"trait_type": "Rarity", "value": "Epic"},
    {"trait_type": "Strength", "value": 85},
    {"trait_type": "Intelligence", "value": 92},
    {"trait_type": "Charisma", "value": 78},
    {"trait_type": "Luck", "value": 65},
    {"trait_type": "Stealth", "value": 71},
    {"trait_type": "Origin", "value": "Sicily"},
    {"trait_type": "Specialty", "value": "Financial Crimes"},
    {"trait_type": "Generation", "value": "Genesis"}
  ],
  "properties": {
    "category": "Character",
    "creators": [{"address": "...", "share": 100}]
  }
}
```

### 2. Weapon NFTs (Loot System)

**Collection Details:**
- **Supply**: Open/unlimited minting
- **Standard**: Semi-fungible (SPL Token 2022 with metadata extension)
- **Distribution**: Compressed NFTs for common drops, full NFTs for rare items
- **Upgrade System**: Stat enchantment through $MOB burning

**Weapon Categories:**
- **Melee**: Knives, bats, brass knuckles
- **Handguns**: Pistols, revolvers, compact SMGs
- **Rifles**: Assault rifles, sniper rifles, shotguns
- **Explosives**: Grenades, C4, rocket launchers
- **Non-lethal**: Tasers, pepper spray, tranquilizers

**Rarity & Stats:**
- **Normal (N)**: Base damage 10-25, common drops
- **Rare (R)**: Base damage 26-50, weekly mission rewards
- **Legendary (L)**: Base damage 51-100, monthly event rewards

**Enhancement System:**
- **Damage Mods**: +1-5 damage per enhancement level
- **Special Effects**: Poison, fire, electric, armor-piercing
- **Durability**: Weapons degrade and require $MOB maintenance
- **Customization**: Visual modifications affect resale value

### 3. Vehicle NFTs (Transportation Fleet)

**Collection Details:**
- **Supply**: 5,000 total vehicles across all types
- **Standard**: Non-fungible (Metaplex Certified Collection)
- **Utility**: Speed bonuses, crew capacity, mission access
- **Rarity**: Rare (60%), Legendary (35%), Mythic (5%)

**Vehicle Types:**
- **Motorcycles**: High speed, low capacity (1-2 crew)
- **Sports Cars**: Balanced speed/capacity (2-4 crew)
- **SUVs**: High capacity, moderate speed (4-6 crew)
- **Armored Trucks**: Maximum capacity, slow speed (6-8 crew)
- **Boats**: Water-based missions only (4-6 crew)
- **Aircraft**: Fastest travel, highest capacity (8-10 crew)

**Modification System:**
- **Engine Upgrades**: Increase speed and fuel efficiency
- **Armor Plating**: Reduce damage in PvP encounters
- **Storage Compartments**: Increase loot carrying capacity
- **Stealth Technology**: Reduce detection in missions

### 4. Turf Deed NFTs (Territory Control)

**Collection Details:**
- **Supply**: 2,500 unique plots across 4 districts
- **Standard**: Non-fungible (Metaplex Certified Collection)
- **Utility**: Passive income generation, DAO governance rights
- **Rarity**: Each plot is unique with different income potential

**District Distribution:**
- **Downtown**: 1,000 plots - Low income, high security
- **Industrial**: 750 plots - Medium income, medium security  
- **Financial**: 500 plots - High income, low security
- **Harbor**: 250 plots - Premium income, variable security

**Income Mechanics:**
- **Base Rent**: 10-100 $MOB per day based on location
- **Business Multipliers**: Legitimate fronts increase income 2-5x
- **Security Costs**: Higher-value plots require more defense spending
- **Taxation**: City takes 20% of gross income for public services

**DAO Integration:**
- **Voting Power**: 1 vote per plot owned
- **Proposal Rights**: Minimum 5 plots to submit proposals
- **Treasury Access**: Plot owners share in family treasury distributions
- **Emergency Powers**: Large plot holders can trigger security measures

## Compressed NFT Implementation

### Merkle Tree Structure
- **Tree Depth**: 20 levels (supports 1M+ items)
- **Canopy Depth**: 14 levels (reduces proof size)
- **Batch Size**: 1,000 items per batch mint
- **Cost Efficiency**: ~$0.001 per NFT vs $0.02 for standard NFTs

### Common Loot Distribution
- **Daily Missions**: 1-3 compressed weapon/item NFTs
- **Weekly Events**: 5-10 compressed NFTs with rare upgrade materials
- **Seasonal Rewards**: Mix of compressed and full NFTs based on achievement tier

### Upgrade Path to Full NFTs
- **Combination System**: 10 compressed items + $MOB = 1 full NFT
- **Quality Preservation**: Compressed rare items create rare full NFTs
- **Metadata Migration**: All attributes transfer to full NFT version

## Cross-Program Integration

### Character-Weapon Synergy
- **Specialization Bonuses**: Characters get damage bonuses with matching weapon types
- **Skill Requirements**: High-tier weapons require minimum character stats
- **Durability Effects**: Character maintenance skills affect weapon degradation

### Vehicle-Territory Synergy  
- **Access Requirements**: Certain territories require specific vehicle types
- **Income Bonuses**: Luxury vehicles increase business front profitability
- **Defense Advantages**: Armored vehicles provide territory defense bonuses

### Collection Interactions
- **Set Bonuses**: Owning items from multiple collections provides synergies
- **Cross-Upgrades**: Use materials from one collection to enhance another
- **Prestige Systems**: High-level items from all collections unlock exclusive content

## Marketplace Integration

### Primary Sales
- **Character Mint**: Dutch auction starting at 2 SOL, floor at 0.5 SOL
- **Vehicle Drops**: Weekly raffles for $FAM token holders
- **Territory Auctions**: Monthly auctions for premium plots

### Secondary Market
- **Royalties**: 5% to creators, 2.5% to game treasury
- **Trading Fees**: 2.5% in $MOB tokens (burned for deflationary pressure)
- **Escrow System**: Secure P2P trading with dispute resolution

### Rental System
- **Character Lending**: Rent high-tier characters for specific missions
- **Vehicle Sharing**: Temporary access to specialized vehicles
- **Territory Partnerships**: Share income from jointly operated plots

This comprehensive NFT ecosystem creates multiple layers of engagement while maintaining economic sustainability through carefully balanced supply, demand, and utility mechanisms.
