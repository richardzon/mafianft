use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{create_master_edition_v3, create_metadata_accounts_v3},
    state::{Creator, DataV2, Collection},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod character_nft {
    use super::*;

    /// Initialize the character NFT program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.collection_mint = ctx.accounts.collection_mint.key();
        config.total_minted = 0;
        config.max_supply = 10_000;
        config.mint_price = 500_000_000; // 0.5 SOL in lamports
        config.is_active = true;
        
        msg!("Character NFT program initialized");
        Ok(())
    }

    /// Mint a new character NFT
    pub fn mint_character(
        ctx: Context<MintCharacter>,
        name: String,
        symbol: String,
        uri: String,
        rarity: CharacterRarity,
        stats: CharacterStats,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::MintingInactive);
        require!(config.total_minted < config.max_supply, ErrorCode::MaxSupplyReached);
        require!(name.len() <= 32, ErrorCode::NameTooLong);
        require!(symbol.len() <= 10, ErrorCode::SymbolTooLong);
        require!(uri.len() <= 200, ErrorCode::UriTooLong);

        // Validate stats based on rarity
        stats.validate_for_rarity(&rarity)?;

        // Create metadata
        let creators = vec![
            Creator {
                address: config.authority,
                verified: true,
                share: 100,
            }
        ];

        let collection = Some(Collection {
            verified: false,
            key: config.collection_mint,
        });

        let data = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 500, // 5% royalty
            creators: Some(creators),
            collection,
            uses: None,
        };

        // Create metadata account
        let metadata_seeds = &[
            b"metadata",
            ctx.accounts.token_metadata_program.key().as_ref(),
            ctx.accounts.mint.key().as_ref(),
        ];
        let (metadata_pda, _) = Pubkey::find_program_address(
            metadata_seeds,
            &ctx.accounts.token_metadata_program.key(),
        );

        let create_metadata_ix = create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            metadata_pda,
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.mint_authority.key(),
            data,
            true,
            true,
            None,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &create_metadata_ix,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
            &[&[
                b"mint_authority",
                &[ctx.bumps.mint_authority],
            ]],
        )?;

        // Create master edition
        let edition_seeds = &[
            b"metadata",
            ctx.accounts.token_metadata_program.key().as_ref(),
            ctx.accounts.mint.key().as_ref(),
            b"edition",
        ];
        let (edition_pda, _) = Pubkey::find_program_address(
            edition_seeds,
            &ctx.accounts.token_metadata_program.key(),
        );

        let create_edition_ix = create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            edition_pda,
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.mint_authority.key(),
            metadata_pda,
            ctx.accounts.payer.key(),
            Some(0), // Max supply of 0 means unlimited prints
        );

        anchor_lang::solana_program::program::invoke_signed(
            &create_edition_ix,
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
            &[&[
                b"mint_authority",
                &[ctx.bumps.mint_authority],
            ]],
        )?;

        // Store character data
        let character = &mut ctx.accounts.character;
        character.mint = ctx.accounts.mint.key();
        character.owner = ctx.accounts.owner.key();
        character.rarity = rarity;
        character.stats = stats;
        character.level = 1;
        character.experience = 0;
        character.last_mission_time = Clock::get()?.unix_timestamp;
        character.is_staked = false;

        config.total_minted += 1;

        msg!("Character NFT minted successfully");
        Ok(())
    }

    /// Level up a character by spending experience
    pub fn level_up(ctx: Context<LevelUp>) -> Result<()> {
        let character = &mut ctx.accounts.character;
        
        require!(character.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(character.level < 100, ErrorCode::MaxLevelReached);
        
        let required_exp = calculate_required_experience(character.level);
        require!(character.experience >= required_exp, ErrorCode::InsufficientExperience);
        
        character.experience -= required_exp;
        character.level += 1;
        
        // Award stat points based on rarity
        let stat_points = match character.rarity {
            CharacterRarity::Common => 2,
            CharacterRarity::Uncommon => 3,
            CharacterRarity::Rare => 4,
            CharacterRarity::Epic => 5,
            CharacterRarity::Legendary => 6,
            CharacterRarity::Mythic => 8,
        };
        
        character.stats.available_points += stat_points;
        
        msg!("Character leveled up to level {}", character.level);
        Ok(())
    }

    /// Merge/burn characters to create higher tier
    pub fn merge_characters(ctx: Context<MergeCharacters>) -> Result<()> {
        // Implementation for burning 3 characters to create next tier
        // This would involve complex logic for rarity upgrades
        msg!("Character merge functionality - to be implemented");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub collection_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintCharacter<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    /// CHECK: PDA used as mint authority
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + Character::INIT_SPACE,
        seeds = [b"character", mint.key().as_ref()],
        bump
    )]
    pub character: Account<'info, Character>,
    
    /// CHECK: Metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    
    /// CHECK: Master edition account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Owner of the NFT
    pub owner: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
    /// CHECK: Metaplex token metadata program
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct LevelUp<'info> {
    #[account(
        mut,
        seeds = [b"character", character.mint.as_ref()],
        bump
    )]
    pub character: Account<'info, Character>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct MergeCharacters<'info> {
    // Accounts for merging characters - to be implemented
    pub owner: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub collection_mint: Pubkey,
    pub total_minted: u32,
    pub max_supply: u32,
    pub mint_price: u64,
    pub is_active: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Character {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub rarity: CharacterRarity,
    pub stats: CharacterStats,
    pub level: u8,
    pub experience: u64,
    pub last_mission_time: i64,
    pub is_staked: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum CharacterRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct CharacterStats {
    pub strength: u8,
    pub intelligence: u8,
    pub charisma: u8,
    pub luck: u8,
    pub stealth: u8,
    pub available_points: u8,
}

impl CharacterStats {
    pub fn validate_for_rarity(&self, rarity: &CharacterRarity) -> Result<()> {
        let total_stats = self.strength + self.intelligence + self.charisma + self.luck + self.stealth;
        
        let (min_total, max_total) = match rarity {
            CharacterRarity::Common => (200, 250),
            CharacterRarity::Uncommon => (251, 300),
            CharacterRarity::Rare => (301, 350),
            CharacterRarity::Epic => (351, 400),
            CharacterRarity::Legendary => (401, 450),
            CharacterRarity::Mythic => (451, 500),
        };
        
        require!(
            total_stats >= min_total && total_stats <= max_total,
            ErrorCode::InvalidStatsForRarity
        );
        
        Ok(())
    }
}

fn calculate_required_experience(current_level: u8) -> u64 {
    // Exponential experience curve
    ((current_level as u64).pow(2) * 100) + (current_level as u64 * 50)
}

#[error_code]
pub enum ErrorCode {
    #[msg("Minting is currently inactive")]
    MintingInactive,
    #[msg("Maximum supply reached")]
    MaxSupplyReached,
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Symbol too long")]
    SymbolTooLong,
    #[msg("URI too long")]
    UriTooLong,
    #[msg("Not the owner of this character")]
    NotOwner,
    #[msg("Character has reached maximum level")]
    MaxLevelReached,
    #[msg("Insufficient experience to level up")]
    InsufficientExperience,
    #[msg("Invalid stats for the specified rarity")]
    InvalidStatsForRarity,
}
