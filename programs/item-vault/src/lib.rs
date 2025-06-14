use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer, Burn},
};
// Simplified for MVP - metadata will be handled separately

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnT");

#[program]
pub mod item_vault {
    use super::*;

    /// Initialize the item vault program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.weapon_collection = ctx.accounts.weapon_collection.key();
        config.vehicle_collection = ctx.accounts.vehicle_collection.key();
        config.total_weapons_minted = 0;
        config.total_vehicles_minted = 0;
        config.is_active = true;
        config.upgrade_fee_base = 100_000_000; // 0.1 MOB base upgrade fee
        
        msg!("Item vault program initialized");
        Ok(())
    }

    /// Mint a weapon NFT (for mission rewards)
    pub fn mint_weapon(
        ctx: Context<MintWeapon>,
        name: String,
        weapon_type: WeaponType,
        rarity: ItemRarity,
        base_damage: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(name.len() <= 32, ErrorCode::NameTooLong);
        require!(base_damage > 0 && base_damage <= 1000, ErrorCode::InvalidDamage);
        
        // Validate damage based on rarity
        validate_weapon_stats(base_damage, &rarity)?;
        
        // Create weapon data
        let weapon = &mut ctx.accounts.weapon;
        weapon.mint = ctx.accounts.mint.key();
        weapon.owner = ctx.accounts.owner.key();
        weapon.weapon_type = weapon_type;
        weapon.rarity = rarity;
        weapon.base_damage = base_damage;
        weapon.current_damage = base_damage;
        weapon.upgrade_level = 0;
        weapon.durability = 100;
        weapon.is_equipped = false;
        weapon.created_at = Clock::get()?.unix_timestamp;
        
        // Metadata will be handled by external service for MVP

        // Mint the NFT (simplified for MVP)
        let seeds = &[
            b"mint_authority",
            &[ctx.bumps.mint_authority],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, 1)?;
        
        config.total_weapons_minted += 1;
        
        emit!(WeaponMinted {
            mint: ctx.accounts.mint.key(),
            owner: ctx.accounts.owner.key(),
            weapon_type,
            rarity,
            base_damage,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Weapon minted: {} with {} damage", weapon.weapon_type as u8, base_damage);
        Ok(())
    }

    /// Mint a vehicle NFT
    pub fn mint_vehicle(
        ctx: Context<MintVehicle>,
        name: String,
        vehicle_type: VehicleType,
        rarity: ItemRarity,
        speed: u16,
        capacity: u8,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(name.len() <= 32, ErrorCode::NameTooLong);
        require!(speed > 0 && speed <= 1000, ErrorCode::InvalidSpeed);
        require!(capacity > 0 && capacity <= 20, ErrorCode::InvalidCapacity);
        
        // Create vehicle data
        let vehicle = &mut ctx.accounts.vehicle;
        vehicle.mint = ctx.accounts.mint.key();
        vehicle.owner = ctx.accounts.owner.key();
        vehicle.vehicle_type = vehicle_type;
        vehicle.rarity = rarity;
        vehicle.speed = speed;
        vehicle.capacity = capacity;
        vehicle.upgrade_level = 0;
        vehicle.durability = 100;
        vehicle.is_active = false;
        vehicle.created_at = Clock::get()?.unix_timestamp;
        
        // Mint the NFT
        let seeds = &[
            b"mint_authority",
            &[ctx.bumps.mint_authority],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, 1)?;
        
        config.total_vehicles_minted += 1;
        
        emit!(VehicleMinted {
            mint: ctx.accounts.mint.key(),
            owner: ctx.accounts.owner.key(),
            vehicle_type,
            rarity,
            speed,
            capacity,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Vehicle minted: {} with speed {}", vehicle.vehicle_type as u8, speed);
        Ok(())
    }

    /// Upgrade weapon with MOB tokens
    pub fn upgrade_weapon(ctx: Context<UpgradeWeapon>) -> Result<()> {
        let config = &ctx.accounts.config;
        let weapon = &mut ctx.accounts.weapon;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(weapon.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(weapon.upgrade_level < 10, ErrorCode::MaxUpgradeReached);
        require!(weapon.durability > 0, ErrorCode::WeaponBroken);
        
        // Calculate upgrade cost
        let upgrade_cost = calculate_upgrade_cost(weapon.upgrade_level, &weapon.rarity);
        
        // Burn MOB tokens for upgrade
        let cpi_accounts = Burn {
            mint: ctx.accounts.mob_mint.to_account_info(),
            from: ctx.accounts.user_mob_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::burn(cpi_ctx, upgrade_cost)?;
        
        // Apply upgrade
        weapon.upgrade_level += 1;
        let damage_increase = calculate_damage_increase(&weapon.rarity);
        weapon.current_damage += damage_increase;
        
        emit!(WeaponUpgraded {
            mint: weapon.mint,
            owner: ctx.accounts.owner.key(),
            new_level: weapon.upgrade_level,
            new_damage: weapon.current_damage,
            cost: upgrade_cost,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Weapon upgraded to level {} with {} damage", weapon.upgrade_level, weapon.current_damage);
        Ok(())
    }

    /// Equip weapon to character
    pub fn equip_weapon(ctx: Context<EquipWeapon>) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon;
        
        require!(weapon.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(weapon.durability > 0, ErrorCode::WeaponBroken);
        require!(!weapon.is_equipped, ErrorCode::AlreadyEquipped);
        
        weapon.is_equipped = true;
        
        emit!(WeaponEquipped {
            mint: weapon.mint,
            character: ctx.accounts.character.key(),
            owner: ctx.accounts.owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Weapon equipped to character");
        Ok(())
    }

    /// Unequip weapon from character
    pub fn unequip_weapon(ctx: Context<UnequipWeapon>) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon;
        
        require!(weapon.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(weapon.is_equipped, ErrorCode::NotEquipped);
        
        weapon.is_equipped = false;
        
        emit!(WeaponUnequipped {
            mint: weapon.mint,
            character: ctx.accounts.character.key(),
            owner: ctx.accounts.owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Weapon unequipped from character");
        Ok(())
    }

    /// Repair weapon with MOB tokens
    pub fn repair_weapon(ctx: Context<RepairWeapon>) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon;
        
        require!(weapon.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(weapon.durability < 100, ErrorCode::WeaponNotDamaged);
        
        let repair_cost = calculate_repair_cost(weapon.durability, &weapon.rarity);
        
        // Burn MOB tokens for repair
        let cpi_accounts = Burn {
            mint: ctx.accounts.mob_mint.to_account_info(),
            from: ctx.accounts.user_mob_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::burn(cpi_ctx, repair_cost)?;
        
        weapon.durability = 100;
        
        emit!(WeaponRepaired {
            mint: weapon.mint,
            owner: ctx.accounts.owner.key(),
            cost: repair_cost,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Weapon repaired to full durability");
        Ok(())
    }

    /// Transfer item between players
    pub fn transfer_item(ctx: Context<TransferItem>) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon;
        
        require!(weapon.owner == ctx.accounts.current_owner.key(), ErrorCode::NotOwner);
        require!(!weapon.is_equipped, ErrorCode::CannotTransferEquipped);
        
        // Transfer the NFT
        let cpi_accounts = Transfer {
            from: ctx.accounts.current_owner_token_account.to_account_info(),
            to: ctx.accounts.new_owner_token_account.to_account_info(),
            authority: ctx.accounts.current_owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, 1)?;
        
        // Update ownership
        weapon.owner = ctx.accounts.new_owner.key();
        
        emit!(ItemTransferred {
            mint: weapon.mint,
            from: ctx.accounts.current_owner.key(),
            to: ctx.accounts.new_owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Item transferred to new owner");
        Ok(())
    }
}

// Account structures
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ItemConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ItemConfig>,
    
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    /// CHECK: PDA used as mint authority
    pub mint_authority: UncheckedAccount<'info>,
    
    pub weapon_collection: Account<'info, Mint>,
    pub vehicle_collection: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintWeapon<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ItemConfig>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
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
        space = 8 + Weapon::INIT_SPACE,
        seeds = [b"weapon", mint.key().as_ref()],
        bump
    )]
    pub weapon: Account<'info, Weapon>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Owner of the weapon
    pub owner: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintVehicle<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ItemConfig>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
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
        space = 8 + Vehicle::INIT_SPACE,
        seeds = [b"vehicle", mint.key().as_ref()],
        bump
    )]
    pub vehicle: Account<'info, Vehicle>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Owner of the vehicle
    pub owner: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradeWeapon<'info> {
    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ItemConfig>,
    
    #[account(
        mut,
        seeds = [b"weapon", weapon.mint.as_ref()],
        bump
    )]
    pub weapon: Account<'info, Weapon>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = owner,
    )]
    pub user_mob_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mob_mint: Account<'info, Mint>,
    
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EquipWeapon<'info> {
    #[account(
        mut,
        seeds = [b"weapon", weapon.mint.as_ref()],
        bump
    )]
    pub weapon: Account<'info, Weapon>,
    
    /// CHECK: Character account
    pub character: UncheckedAccount<'info>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnequipWeapon<'info> {
    #[account(
        mut,
        seeds = [b"weapon", weapon.mint.as_ref()],
        bump
    )]
    pub weapon: Account<'info, Weapon>,
    
    /// CHECK: Character account
    pub character: UncheckedAccount<'info>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RepairWeapon<'info> {
    #[account(
        mut,
        seeds = [b"weapon", weapon.mint.as_ref()],
        bump
    )]
    pub weapon: Account<'info, Weapon>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = owner,
    )]
    pub user_mob_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mob_mint: Account<'info, Mint>,
    
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferItem<'info> {
    #[account(
        mut,
        seeds = [b"weapon", weapon.mint.as_ref()],
        bump
    )]
    pub weapon: Account<'info, Weapon>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = current_owner,
    )]
    pub current_owner_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = current_owner,
        associated_token::mint = mint,
        associated_token::authority = new_owner,
    )]
    pub new_owner_token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub current_owner: Signer<'info>,
    
    /// CHECK: New owner address
    pub new_owner: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// Data structures
#[account]
#[derive(InitSpace)]
pub struct ItemConfig {
    pub authority: Pubkey,
    pub weapon_collection: Pubkey,
    pub vehicle_collection: Pubkey,
    pub total_weapons_minted: u32,
    pub total_vehicles_minted: u32,
    pub is_active: bool,
    pub upgrade_fee_base: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Weapon {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub weapon_type: WeaponType,
    pub rarity: ItemRarity,
    pub base_damage: u16,
    pub current_damage: u16,
    pub upgrade_level: u8,
    pub durability: u8,
    pub is_equipped: bool,
    pub created_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Vehicle {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub vehicle_type: VehicleType,
    pub rarity: ItemRarity,
    pub speed: u16,
    pub capacity: u8,
    pub upgrade_level: u8,
    pub durability: u8,
    pub is_active: bool,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum WeaponType {
    Pistol,
    Rifle,
    Shotgun,
    SMG,
    Sniper,
    Knife,
    Bat,
    Grenade,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum VehicleType {
    Motorcycle,
    Car,
    SUV,
    Truck,
    Boat,
    Helicopter,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

// Helper functions
fn validate_weapon_stats(damage: u16, rarity: &ItemRarity) -> Result<()> {
    let (min_damage, max_damage) = match rarity {
        ItemRarity::Common => (10, 25),
        ItemRarity::Uncommon => (26, 40),
        ItemRarity::Rare => (41, 60),
        ItemRarity::Epic => (61, 80),
        ItemRarity::Legendary => (81, 100),
    };
    
    require!(
        damage >= min_damage && damage <= max_damage,
        ErrorCode::InvalidDamageForRarity
    );
    
    Ok(())
}

fn calculate_upgrade_cost(current_level: u8, rarity: &ItemRarity) -> u64 {
    let base_cost = 100_000_000; // 0.1 MOB
    let level_multiplier = (current_level as u64 + 1) * (current_level as u64 + 1);
    let rarity_multiplier = match rarity {
        ItemRarity::Common => 1,
        ItemRarity::Uncommon => 2,
        ItemRarity::Rare => 3,
        ItemRarity::Epic => 5,
        ItemRarity::Legendary => 8,
    };
    
    base_cost * level_multiplier * rarity_multiplier
}

fn calculate_damage_increase(rarity: &ItemRarity) -> u16 {
    match rarity {
        ItemRarity::Common => 2,
        ItemRarity::Uncommon => 3,
        ItemRarity::Rare => 4,
        ItemRarity::Epic => 6,
        ItemRarity::Legendary => 10,
    }
}

fn calculate_repair_cost(current_durability: u8, rarity: &ItemRarity) -> u64 {
    let base_cost = 10_000_000; // 0.01 MOB per durability point
    let durability_lost = 100 - current_durability as u64;
    let rarity_multiplier = match rarity {
        ItemRarity::Common => 1,
        ItemRarity::Uncommon => 2,
        ItemRarity::Rare => 3,
        ItemRarity::Epic => 4,
        ItemRarity::Legendary => 5,
    };
    
    base_cost * durability_lost * rarity_multiplier
}

// Events
#[event]
pub struct WeaponMinted {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub weapon_type: WeaponType,
    pub rarity: ItemRarity,
    pub base_damage: u16,
    pub timestamp: i64,
}

#[event]
pub struct VehicleMinted {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub vehicle_type: VehicleType,
    pub rarity: ItemRarity,
    pub speed: u16,
    pub capacity: u8,
    pub timestamp: i64,
}

#[event]
pub struct WeaponUpgraded {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub new_level: u8,
    pub new_damage: u16,
    pub cost: u64,
    pub timestamp: i64,
}

#[event]
pub struct WeaponEquipped {
    pub mint: Pubkey,
    pub character: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct WeaponUnequipped {
    pub mint: Pubkey,
    pub character: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct WeaponRepaired {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub cost: u64,
    pub timestamp: i64,
}

#[event]
pub struct ItemTransferred {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Program is currently inactive")]
    ProgramInactive,
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Invalid damage value")]
    InvalidDamage,
    #[msg("Invalid speed value")]
    InvalidSpeed,
    #[msg("Invalid capacity value")]
    InvalidCapacity,
    #[msg("Invalid damage for rarity")]
    InvalidDamageForRarity,
    #[msg("Not the owner of this item")]
    NotOwner,
    #[msg("Maximum upgrade level reached")]
    MaxUpgradeReached,
    #[msg("Weapon is broken")]
    WeaponBroken,
    #[msg("Weapon is already equipped")]
    AlreadyEquipped,
    #[msg("Weapon is not equipped")]
    NotEquipped,
    #[msg("Weapon is not damaged")]
    WeaponNotDamaged,
    #[msg("Cannot transfer equipped item")]
    CannotTransferEquipped,
}
