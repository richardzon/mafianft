use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnV");

#[program]
pub mod mob_token {
    use super::*;

    /// Initialize the MOB token program
    pub fn initialize(ctx: Context<Initialize>, daily_emission: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.mint = ctx.accounts.mint.key();
        config.daily_emission = daily_emission;
        config.total_minted = 0;
        config.total_burned = 0;
        config.last_emission_time = Clock::get()?.unix_timestamp;
        config.is_active = true;
        config.anti_bot_threshold = 100; // Max 100 transactions per hour per wallet
        
        msg!("MOB token program initialized with daily emission: {}", daily_emission);
        Ok(())
    }

    /// Mint MOB tokens for rewards
    pub fn mint_reward(
        ctx: Context<MintReward>,
        amount: u64,
        reward_type: RewardType,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        
        // Check daily emission limits
        let current_time = Clock::get()?.unix_timestamp;
        let time_diff = current_time - config.last_emission_time;
        
        if time_diff >= 86400 { // 24 hours
            config.last_emission_time = current_time;
            // Reset daily counters if needed
        }
        
        // Validate reward amount based on type
        validate_reward_amount(amount, &reward_type)?;
        
        // Anti-bot protection
        let player_data = &mut ctx.accounts.player_data;
        check_rate_limit(player_data, current_time)?;
        
        // Mint tokens
        let seeds = &[
            b"mint_authority",
            &[ctx.bumps.mint_authority],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, amount)?;
        
        config.total_minted += amount;
        player_data.total_earned += amount;
        player_data.last_reward_time = current_time;
        player_data.transaction_count += 1;
        
        emit!(RewardMinted {
            recipient: ctx.accounts.recipient.key(),
            amount,
            reward_type,
            timestamp: current_time,
        });
        
        msg!("Minted {} MOB tokens as {:?} reward", amount, reward_type);
        Ok(())
    }

    /// Burn MOB tokens for upgrades/features
    pub fn burn_tokens(
        ctx: Context<BurnTokens>,
        amount: u64,
        burn_reason: BurnReason,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Validate burn amount based on reason
        validate_burn_amount(amount, &burn_reason)?;
        
        // Burn tokens
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::burn(cpi_ctx, amount)?;
        
        config.total_burned += amount;
        
        let player_data = &mut ctx.accounts.player_data;
        player_data.total_burned += amount;
        
        emit!(TokensBurned {
            user: ctx.accounts.user.key(),
            amount,
            burn_reason,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Burned {} MOB tokens for {:?}", amount, burn_reason);
        Ok(())
    }

    /// Transfer tokens with anti-bot protection
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Anti-bot protection
        let player_data = &mut ctx.accounts.sender_data;
        let current_time = Clock::get()?.unix_timestamp;
        check_rate_limit(player_data, current_time)?;
        
        // Transfer tokens
        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        player_data.transaction_count += 1;
        player_data.last_transaction_time = current_time;
        
        emit!(TokensTransferred {
            sender: ctx.accounts.sender.key(),
            recipient: ctx.accounts.recipient.key(),
            amount,
            timestamp: current_time,
        });
        
        msg!("Transferred {} MOB tokens", amount);
        Ok(())
    }

    /// Update program configuration (admin only)
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_daily_emission: Option<u64>,
        new_anti_bot_threshold: Option<u32>,
        new_is_active: Option<bool>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(
            ctx.accounts.authority.key() == config.authority,
            ErrorCode::Unauthorized
        );
        
        if let Some(emission) = new_daily_emission {
            config.daily_emission = emission;
        }
        
        if let Some(threshold) = new_anti_bot_threshold {
            config.anti_bot_threshold = threshold;
        }
        
        if let Some(active) = new_is_active {
            config.is_active = active;
        }
        
        msg!("MOB token config updated");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TokenConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TokenConfig>,
    
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    /// CHECK: PDA used as mint authority
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintReward<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TokenConfig>,
    
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    /// CHECK: PDA used as mint authority
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + PlayerData::INIT_SPACE,
        seeds = [b"player", recipient.key().as_ref()],
        bump
    )]
    pub player_data: Account<'info, PlayerData>,
    
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Recipient of the reward
    pub recipient: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TokenConfig>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"player", user.key().as_ref()],
        bump
    )]
    pub player_data: Account<'info, PlayerData>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(
        mut,
        seeds = [b"player", sender.key().as_ref()],
        bump
    )]
    pub sender_data: Account<'info, PlayerData>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = sender,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    /// CHECK: Recipient address
    pub recipient: UncheckedAccount<'info>,
    pub sender: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TokenConfig>,
    
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct TokenConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub daily_emission: u64,
    pub total_minted: u64,
    pub total_burned: u64,
    pub last_emission_time: i64,
    pub is_active: bool,
    pub anti_bot_threshold: u32,
}

#[account]
#[derive(InitSpace)]
pub struct PlayerData {
    pub player: Pubkey,
    pub total_earned: u64,
    pub total_burned: u64,
    pub transaction_count: u32,
    pub last_reward_time: i64,
    pub last_transaction_time: i64,
    pub hourly_transaction_count: u32,
    pub last_hour_reset: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum RewardType {
    Mission,
    PvpVictory,
    TerritoryDefense,
    BusinessIncome,
    Staking,
    Tournament,
    Daily,
    Achievement,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum BurnReason {
    WeaponUpgrade,
    CharacterRespawn,
    BusinessBribe,
    EnergyRefill,
    TerritoryTax,
    MarketplaceFee,
    PremiumFeature,
    Cosmetic,
}

fn validate_reward_amount(amount: u64, reward_type: &RewardType) -> Result<()> {
    let max_amount = match reward_type {
        RewardType::Mission => 2_500_000_000, // 2.5 MOB
        RewardType::PvpVictory => 2_000_000_000, // 2 MOB
        RewardType::TerritoryDefense => 1_000_000_000, // 1 MOB
        RewardType::BusinessIncome => 500_000_000, // 0.5 MOB
        RewardType::Staking => 10_000_000_000, // 10 MOB (daily)
        RewardType::Tournament => 10_000_000_000, // 10 MOB
        RewardType::Daily => 1_000_000_000, // 1 MOB
        RewardType::Achievement => 5_000_000_000, // 5 MOB
    };
    
    require!(amount <= max_amount, ErrorCode::ExcessiveRewardAmount);
    Ok(())
}

fn validate_burn_amount(amount: u64, burn_reason: &BurnReason) -> Result<()> {
    let max_amount = match burn_reason {
        BurnReason::WeaponUpgrade => 5_000_000_000, // 5 MOB
        BurnReason::CharacterRespawn => 2_000_000_000, // 2 MOB
        BurnReason::BusinessBribe => 10_000_000_000, // 10 MOB
        BurnReason::EnergyRefill => 200_000_000, // 0.2 MOB
        BurnReason::TerritoryTax => 100_000_000_000, // 100 MOB
        BurnReason::MarketplaceFee => 50_000_000_000, // 50 MOB
        BurnReason::PremiumFeature => 25_000_000_000, // 25 MOB
        BurnReason::Cosmetic => 5_000_000_000, // 5 MOB
    };
    
    require!(amount <= max_amount, ErrorCode::ExcessiveBurnAmount);
    Ok(())
}

fn check_rate_limit(player_data: &mut PlayerData, current_time: i64) -> Result<()> {
    // Reset hourly counter if an hour has passed
    if current_time - player_data.last_hour_reset >= 3600 {
        player_data.hourly_transaction_count = 0;
        player_data.last_hour_reset = current_time;
    }
    
    require!(
        player_data.hourly_transaction_count < 100,
        ErrorCode::RateLimitExceeded
    );
    
    player_data.hourly_transaction_count += 1;
    Ok(())
}

#[event]
pub struct RewardMinted {
    pub recipient: Pubkey,
    pub amount: u64,
    pub reward_type: RewardType,
    pub timestamp: i64,
}

#[event]
pub struct TokensBurned {
    pub user: Pubkey,
    pub amount: u64,
    pub burn_reason: BurnReason,
    pub timestamp: i64,
}

#[event]
pub struct TokensTransferred {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Program is currently inactive")]
    ProgramInactive,
    #[msg("Invalid amount specified")]
    InvalidAmount,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Excessive reward amount for this type")]
    ExcessiveRewardAmount,
    #[msg("Excessive burn amount for this reason")]
    ExcessiveBurnAmount,
    #[msg("Rate limit exceeded - too many transactions")]
    RateLimitExceeded,
}
