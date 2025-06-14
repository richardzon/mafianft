use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnU");

#[program]
pub mod turf_control {
    use super::*;

    /// Initialize the turf control program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.turf_collection = ctx.accounts.turf_collection.key();
        config.mob_mint = ctx.accounts.mob_mint.key();
        config.total_territories = 2500;
        config.territories_minted = 0;
        config.base_income_rate = 10_000_000; // 0.01 MOB per day base
        config.tax_rate = 2000; // 20% tax to treasury
        config.attack_cooldown = 172800; // 48 hours
        config.is_active = true;
        
        msg!("Turf control program initialized");
        Ok(())
    }

    /// Mint a territory NFT (initial distribution)
    pub fn mint_territory(
        ctx: Context<MintTerritory>,
        district: District,
        plot_id: u16,
        base_income: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(config.territories_minted < config.total_territories, ErrorCode::MaxTerritoriesReached);
        require!(plot_id > 0 && plot_id <= config.total_territories as u16, ErrorCode::InvalidPlotId);
        require!(base_income > 0, ErrorCode::InvalidIncome);
        
        // Create territory data
        let territory = &mut ctx.accounts.territory;
        territory.mint = ctx.accounts.mint.key();
        territory.owner = ctx.accounts.owner.key();
        territory.district = district;
        territory.plot_id = plot_id;
        territory.base_income = base_income;
        territory.current_income = base_income;
        territory.security_level = 50; // Default security
        territory.last_income_claim = Clock::get()?.unix_timestamp;
        territory.last_attack_time = 0;
        territory.defense_wins = 0;
        territory.attack_wins = 0;
        territory.is_under_attack = false;
        territory.businesses = Vec::new();
        
        // Mint the territory NFT
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
        
        config.territories_minted += 1;
        
        emit!(TerritoryMinted {
            mint: ctx.accounts.mint.key(),
            owner: ctx.accounts.owner.key(),
            district,
            plot_id,
            base_income,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Territory minted: District {:?}, Plot {}", district, plot_id);
        Ok(())
    }

    /// Claim daily income from territory
    pub fn claim_income(ctx: Context<ClaimIncome>) -> Result<()> {
        let config = &ctx.accounts.config;
        let territory = &mut ctx.accounts.territory;
        let current_time = Clock::get()?.unix_timestamp;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(territory.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(!territory.is_under_attack, ErrorCode::TerritoryUnderAttack);
        
        // Check if 24 hours have passed since last claim
        let time_since_last_claim = current_time - territory.last_income_claim;
        require!(time_since_last_claim >= 86400, ErrorCode::ClaimTooEarly); // 24 hours
        
        // Calculate income (can claim multiple days if missed)
        let days_to_claim = (time_since_last_claim / 86400) as u64;
        let total_income = territory.current_income * days_to_claim;
        
        // Calculate tax (20% to treasury)
        let tax_amount = (total_income * config.tax_rate as u64) / 10000;
        let net_income = total_income - tax_amount;
        
        // Mint MOB tokens for income
        let seeds = &[
            b"income_authority",
            &[ctx.bumps.income_authority],
        ];
        let signer = &[&seeds[..]];
        
        // Mint net income to owner
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mob_mint.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.income_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, net_income)?;
        
        // Mint tax to treasury
        let cpi_accounts_tax = MintTo {
            mint: ctx.accounts.mob_mint.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.income_authority.to_account_info(),
        };
        let cpi_ctx_tax = CpiContext::new_with_signer(cpi_program, cpi_accounts_tax, signer);
        
        token::mint_to(cpi_ctx_tax, tax_amount)?;
        
        territory.last_income_claim = current_time;
        
        emit!(IncomeClaimed {
            territory: territory.mint,
            owner: ctx.accounts.owner.key(),
            gross_income: total_income,
            net_income,
            tax_amount,
            days_claimed: days_to_claim,
            timestamp: current_time,
        });
        
        msg!("Income claimed: {} MOB (net), {} MOB (tax)", net_income, tax_amount);
        Ok(())
    }

    /// Upgrade territory security
    pub fn upgrade_security(ctx: Context<UpgradeSecurity>, investment: u64) -> Result<()> {
        let territory = &mut ctx.accounts.territory;
        
        require!(territory.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(investment > 0, ErrorCode::InvalidInvestment);
        require!(territory.security_level < 100, ErrorCode::MaxSecurityReached);
        
        // Burn MOB tokens for security upgrade
        let cpi_accounts = anchor_spl::token::Burn {
            mint: ctx.accounts.mob_mint.to_account_info(),
            from: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::burn(cpi_ctx, investment)?;
        
        // Calculate security increase (1 point per 0.1 MOB invested)
        let security_increase = std::cmp::min(
            (investment / 100_000_000) as u8, // 0.1 MOB = 100M lamports
            100 - territory.security_level
        );
        
        territory.security_level += security_increase;
        
        emit!(SecurityUpgraded {
            territory: territory.mint,
            owner: ctx.accounts.owner.key(),
            investment,
            new_security_level: territory.security_level,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Security upgraded to level {}", territory.security_level);
        Ok(())
    }

    /// Attack another territory (PvP)
    pub fn attack_territory(ctx: Context<AttackTerritory>) -> Result<()> {
        let config = &ctx.accounts.config;
        let attacker_territory = &ctx.accounts.attacker_territory;
        let defender_territory = &mut ctx.accounts.defender_territory;
        let current_time = Clock::get()?.unix_timestamp;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(attacker_territory.owner == ctx.accounts.attacker.key(), ErrorCode::NotOwner);
        require!(attacker_territory.mint != defender_territory.mint, ErrorCode::CannotAttackSelf);
        require!(!defender_territory.is_under_attack, ErrorCode::TerritoryUnderAttack);
        
        // Check attack cooldown
        let time_since_last_attack = current_time - attacker_territory.last_attack_time;
        require!(time_since_last_attack >= config.attack_cooldown, ErrorCode::AttackCooldownActive);
        
        // Mark territory as under attack
        defender_territory.is_under_attack = true;
        
        // Calculate attack success based on security levels and randomness
        let attack_power = 100 - attacker_territory.security_level;
        let defense_power = defender_territory.security_level;
        
        // Simple deterministic "randomness" based on slot and accounts
        let slot = Clock::get()?.slot;
        let random_factor = (slot % 100) as u8;
        
        let attack_success = (attack_power + random_factor) > defense_power;
        
        emit!(TerritoryAttacked {
            attacker: ctx.accounts.attacker.key(),
            attacker_territory: attacker_territory.mint,
            defender_territory: defender_territory.mint,
            attack_power,
            defense_power,
            success: attack_success,
            timestamp: current_time,
        });
        
        if attack_success {
            msg!("Attack successful! Territory captured.");
        } else {
            msg!("Attack failed! Territory defended.");
        }
        
        Ok(())
    }

    /// Resolve territory attack (called after attack period)
    pub fn resolve_attack(ctx: Context<ResolveAttack>, attack_successful: bool) -> Result<()> {
        let attacker_territory = &mut ctx.accounts.attacker_territory;
        let defender_territory = &mut ctx.accounts.defender_territory;
        
        require!(defender_territory.is_under_attack, ErrorCode::NoActiveAttack);
        
        if attack_successful {
            // Transfer territory ownership
            defender_territory.owner = attacker_territory.owner;
            attacker_territory.attack_wins += 1;
            
            // Transfer the NFT
            let cpi_accounts = Transfer {
                from: ctx.accounts.defender_token_account.to_account_info(),
                to: ctx.accounts.attacker_token_account.to_account_info(),
                authority: ctx.accounts.defender.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            token::transfer(cpi_ctx, 1)?;
        } else {
            defender_territory.defense_wins += 1;
        }
        
        defender_territory.is_under_attack = false;
        attacker_territory.last_attack_time = Clock::get()?.unix_timestamp;
        
        emit!(AttackResolved {
            attacker: attacker_territory.owner,
            defender: ctx.accounts.defender.key(),
            territory: defender_territory.mint,
            successful: attack_successful,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Attack resolved: {}", if attack_successful { "Successful" } else { "Failed" });
        Ok(())
    }

    /// Add business to territory (increases income)
    pub fn add_business(
        ctx: Context<AddBusiness>,
        business_type: BusinessType,
        investment: u64,
    ) -> Result<()> {
        let territory = &mut ctx.accounts.territory;
        
        require!(territory.owner == ctx.accounts.owner.key(), ErrorCode::NotOwner);
        require!(investment > 0, ErrorCode::InvalidInvestment);
        require!(territory.businesses.len() < 5, ErrorCode::MaxBusinessesReached);
        
        // Burn MOB tokens for business investment
        let cpi_accounts = anchor_spl::token::Burn {
            mint: ctx.accounts.mob_mint.to_account_info(),
            from: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::burn(cpi_ctx, investment)?;
        
        // Calculate income increase based on business type and investment
        let income_multiplier = match business_type {
            BusinessType::Restaurant => 1.2,
            BusinessType::Nightclub => 1.5,
            BusinessType::Casino => 2.0,
            BusinessType::Construction => 1.3,
            BusinessType::Shipping => 1.8,
        };
        
        let income_increase = ((investment as f64 * income_multiplier) / 1000.0) as u64;
        
        // Add business to territory
        let business = Business {
            business_type,
            investment,
            income_boost: income_increase,
            created_at: Clock::get()?.unix_timestamp,
        };
        
        territory.businesses.push(business);
        territory.current_income += income_increase;
        
        emit!(BusinessAdded {
            territory: territory.mint,
            owner: ctx.accounts.owner.key(),
            business_type,
            investment,
            income_boost: income_increase,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Business added: {:?}, Income boost: {}", business_type, income_increase);
        Ok(())
    }
}

// Account structures
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TurfConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TurfConfig>,
    
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    /// CHECK: PDA used as mint authority
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"income_authority"],
        bump
    )]
    /// CHECK: PDA used for income minting
    pub income_authority: UncheckedAccount<'info>,
    
    pub turf_collection: Account<'info, Mint>,
    pub mob_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTerritory<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TurfConfig>,
    
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
        space = 8 + Territory::INIT_SPACE,
        seeds = [b"territory", mint.key().as_ref()],
        bump
    )]
    pub territory: Account<'info, Territory>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Owner of the territory
    pub owner: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimIncome<'info> {
    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TurfConfig>,
    
    #[account(
        mut,
        seeds = [b"territory", territory.mint.as_ref()],
        bump
    )]
    pub territory: Account<'info, Territory>,
    
    #[account(
        seeds = [b"income_authority"],
        bump
    )]
    /// CHECK: PDA used for income minting
    pub income_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Treasury authority PDA
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub mob_mint: Account<'info, Mint>,
    
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpgradeSecurity<'info> {
    #[account(
        mut,
        seeds = [b"territory", territory.mint.as_ref()],
        bump
    )]
    pub territory: Account<'info, Territory>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mob_mint: Account<'info, Mint>,
    
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AttackTerritory<'info> {
    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TurfConfig>,
    
    #[account(
        seeds = [b"territory", attacker_territory.mint.as_ref()],
        bump
    )]
    pub attacker_territory: Account<'info, Territory>,
    
    #[account(
        mut,
        seeds = [b"territory", defender_territory.mint.as_ref()],
        bump
    )]
    pub defender_territory: Account<'info, Territory>,
    
    pub attacker: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveAttack<'info> {
    #[account(
        mut,
        seeds = [b"territory", attacker_territory.mint.as_ref()],
        bump
    )]
    pub attacker_territory: Account<'info, Territory>,
    
    #[account(
        mut,
        seeds = [b"territory", defender_territory.mint.as_ref()],
        bump
    )]
    pub defender_territory: Account<'info, Territory>,
    
    #[account(
        mut,
        associated_token::mint = defender_territory.mint,
        associated_token::authority = defender,
    )]
    pub defender_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = defender_territory.mint,
        associated_token::authority = attacker_territory.owner,
    )]
    pub attacker_token_account: Account<'info, TokenAccount>,
    
    pub defender: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AddBusiness<'info> {
    #[account(
        mut,
        seeds = [b"territory", territory.mint.as_ref()],
        bump
    )]
    pub territory: Account<'info, Territory>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub mob_mint: Account<'info, Mint>,
    
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Data structures
#[account]
#[derive(InitSpace)]
pub struct TurfConfig {
    pub authority: Pubkey,
    pub turf_collection: Pubkey,
    pub mob_mint: Pubkey,
    pub total_territories: u32,
    pub territories_minted: u32,
    pub base_income_rate: u64,
    pub tax_rate: u16, // Basis points
    pub attack_cooldown: i64, // Seconds
    pub is_active: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Territory {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub district: District,
    pub plot_id: u16,
    pub base_income: u64,
    pub current_income: u64,
    pub security_level: u8,
    pub last_income_claim: i64,
    pub last_attack_time: i64,
    pub defense_wins: u32,
    pub attack_wins: u32,
    pub is_under_attack: bool,
    #[max_len(5)]
    pub businesses: Vec<Business>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Business {
    pub business_type: BusinessType,
    pub investment: u64,
    pub income_boost: u64,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum District {
    Downtown,
    Industrial,
    Financial,
    Harbor,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum BusinessType {
    Restaurant,
    Nightclub,
    Casino,
    Construction,
    Shipping,
}

// Events
#[event]
pub struct TerritoryMinted {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub district: District,
    pub plot_id: u16,
    pub base_income: u64,
    pub timestamp: i64,
}

#[event]
pub struct IncomeClaimed {
    pub territory: Pubkey,
    pub owner: Pubkey,
    pub gross_income: u64,
    pub net_income: u64,
    pub tax_amount: u64,
    pub days_claimed: u64,
    pub timestamp: i64,
}

#[event]
pub struct SecurityUpgraded {
    pub territory: Pubkey,
    pub owner: Pubkey,
    pub investment: u64,
    pub new_security_level: u8,
    pub timestamp: i64,
}

#[event]
pub struct TerritoryAttacked {
    pub attacker: Pubkey,
    pub attacker_territory: Pubkey,
    pub defender_territory: Pubkey,
    pub attack_power: u8,
    pub defense_power: u8,
    pub success: bool,
    pub timestamp: i64,
}

#[event]
pub struct AttackResolved {
    pub attacker: Pubkey,
    pub defender: Pubkey,
    pub territory: Pubkey,
    pub successful: bool,
    pub timestamp: i64,
}

#[event]
pub struct BusinessAdded {
    pub territory: Pubkey,
    pub owner: Pubkey,
    pub business_type: BusinessType,
    pub investment: u64,
    pub income_boost: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Program is currently inactive")]
    ProgramInactive,
    #[msg("Maximum territories reached")]
    MaxTerritoriesReached,
    #[msg("Invalid plot ID")]
    InvalidPlotId,
    #[msg("Invalid income amount")]
    InvalidIncome,
    #[msg("Not the owner of this territory")]
    NotOwner,
    #[msg("Territory is under attack")]
    TerritoryUnderAttack,
    #[msg("Too early to claim income")]
    ClaimTooEarly,
    #[msg("Invalid investment amount")]
    InvalidInvestment,
    #[msg("Maximum security level reached")]
    MaxSecurityReached,
    #[msg("Cannot attack own territory")]
    CannotAttackSelf,
    #[msg("Attack cooldown is active")]
    AttackCooldownActive,
    #[msg("No active attack to resolve")]
    NoActiveAttack,
    #[msg("Maximum businesses reached")]
    MaxBusinessesReached,
}
