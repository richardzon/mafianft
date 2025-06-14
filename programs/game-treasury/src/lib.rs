use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnX");

#[program]
pub mod game_treasury {
    use super::*;

    /// Initialize the game treasury
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.authority = ctx.accounts.authority.key();
        treasury.mob_mint = ctx.accounts.mob_mint.key();
        treasury.fam_mint = ctx.accounts.fam_mint.key();
        treasury.total_fees_collected = 0;
        treasury.total_rewards_distributed = 0;
        treasury.marketplace_fee_rate = 250; // 2.5%
        treasury.territory_tax_rate = 2000; // 20%
        treasury.is_active = true;
        
        msg!("Game treasury initialized");
        Ok(())
    }

    /// Collect marketplace fees (called by other programs)
    pub fn collect_marketplace_fee(
        ctx: Context<CollectMarketplaceFee>,
        amount: u64,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        
        require!(treasury.is_active, ErrorCode::TreasuryInactive);
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Transfer MOB tokens to treasury
        let cpi_accounts = Transfer {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        treasury.total_fees_collected += amount;
        
        emit!(FeeCollected {
            fee_type: FeeType::Marketplace,
            amount,
            payer: ctx.accounts.payer.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Collected marketplace fee: {} MOB", amount);
        Ok(())
    }

    /// Collect territory tax (automatic from territory income)
    pub fn collect_territory_tax(
        ctx: Context<CollectTerritoryTax>,
        territory_income: u64,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        
        require!(treasury.is_active, ErrorCode::TreasuryInactive);
        require!(territory_income > 0, ErrorCode::InvalidAmount);
        
        // Calculate tax amount (20% of territory income)
        let tax_amount = (territory_income * treasury.territory_tax_rate) / 10000;
        
        // Transfer tax to treasury
        let cpi_accounts = Transfer {
            from: ctx.accounts.territory_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.territory_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, tax_amount)?;
        
        treasury.total_fees_collected += tax_amount;
        
        emit!(FeeCollected {
            fee_type: FeeType::TerritoryTax,
            amount: tax_amount,
            payer: ctx.accounts.territory_owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Collected territory tax: {} MOB", tax_amount);
        Ok(())
    }

    /// Distribute rewards to players
    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        amount: u64,
        reward_type: RewardType,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        
        require!(treasury.is_active, ErrorCode::TreasuryInactive);
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Validate reward amount based on type
        validate_reward_distribution(amount, &reward_type)?;
        
        // Transfer rewards from treasury
        let seeds = &[
            b"treasury_authority",
            &[ctx.bumps.treasury_authority],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.treasury_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        treasury.total_rewards_distributed += amount;
        
        emit!(RewardDistributed {
            recipient: ctx.accounts.recipient.key(),
            amount,
            reward_type,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Distributed {} MOB as {:?} reward", amount, reward_type);
        Ok(())
    }

    /// Distribute FAM staking rewards
    pub fn distribute_staking_rewards(
        ctx: Context<DistributeStakingRewards>,
        amount: u64,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        
        require!(treasury.is_active, ErrorCode::TreasuryInactive);
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Transfer FAM tokens from treasury
        let seeds = &[
            b"treasury_authority",
            &[ctx.bumps.treasury_authority],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.treasury_fam_account.to_account_info(),
            to: ctx.accounts.recipient_fam_account.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        emit!(StakingRewardDistributed {
            recipient: ctx.accounts.recipient.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Distributed {} FAM as staking reward", amount);
        Ok(())
    }

    /// Emergency withdraw (admin only)
    pub fn emergency_withdraw(
        ctx: Context<EmergencyWithdraw>,
        amount: u64,
        token_type: TokenType,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        
        require!(
            ctx.accounts.authority.key() == treasury.authority,
            ErrorCode::Unauthorized
        );
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        let seeds = &[
            b"treasury_authority",
            &[ctx.bumps.treasury_authority],
        ];
        let signer = &[&seeds[..]];
        
        match token_type {
            TokenType::MOB => {
                let cpi_accounts = Transfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: ctx.accounts.authority_token_account.to_account_info(),
                    authority: ctx.accounts.treasury_authority.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
                
                token::transfer(cpi_ctx, amount)?;
            }
            TokenType::FAM => {
                let cpi_accounts = Transfer {
                    from: ctx.accounts.treasury_fam_account.to_account_info(),
                    to: ctx.accounts.authority_fam_account.to_account_info(),
                    authority: ctx.accounts.treasury_authority.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
                
                token::transfer(cpi_ctx, amount)?;
            }
        }
        
        emit!(EmergencyWithdrawal {
            authority: ctx.accounts.authority.key(),
            amount,
            token_type,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Emergency withdrawal: {} {:?}", amount, token_type);
        Ok(())
    }

    /// Update treasury configuration
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_marketplace_fee_rate: Option<u16>,
        new_territory_tax_rate: Option<u16>,
        new_is_active: Option<bool>,
    ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        
        require!(
            ctx.accounts.authority.key() == treasury.authority,
            ErrorCode::Unauthorized
        );
        
        if let Some(fee_rate) = new_marketplace_fee_rate {
            require!(fee_rate <= 1000, ErrorCode::ExcessiveFeeRate); // Max 10%
            treasury.marketplace_fee_rate = fee_rate;
        }
        
        if let Some(tax_rate) = new_territory_tax_rate {
            require!(tax_rate <= 5000, ErrorCode::ExcessiveTaxRate); // Max 50%
            treasury.territory_tax_rate = tax_rate;
        }
        
        if let Some(active) = new_is_active {
            treasury.is_active = active;
        }
        
        msg!("Treasury configuration updated");
        Ok(())
    }

    /// Get treasury statistics
    pub fn get_treasury_stats(ctx: Context<GetTreasuryStats>) -> Result<TreasuryStats> {
        let treasury = &ctx.accounts.treasury;
        
        let mob_balance = ctx.accounts.treasury_token_account.amount;
        let fam_balance = ctx.accounts.treasury_fam_account.amount;
        
        let stats = TreasuryStats {
            total_fees_collected: treasury.total_fees_collected,
            total_rewards_distributed: treasury.total_rewards_distributed,
            current_mob_balance: mob_balance,
            current_fam_balance: fam_balance,
            marketplace_fee_rate: treasury.marketplace_fee_rate,
            territory_tax_rate: treasury.territory_tax_rate,
            is_active: treasury.is_active,
        };
        
        emit!(TreasuryStatsRequested {
            requester: ctx.accounts.requester.key(),
            stats: stats.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(stats)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        seeds = [b"treasury_authority"],
        bump
    )]
    /// CHECK: PDA used as treasury authority
    pub treasury_authority: UncheckedAccount<'info>,
    
    pub mob_mint: Account<'info, Mint>,
    pub fam_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CollectMarketplaceFee<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"treasury_authority"],
        bump
    )]
    /// CHECK: PDA used as treasury authority
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = payer,
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    
    pub mob_mint: Account<'info, Mint>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CollectTerritoryTax<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"treasury_authority"],
        bump
    )]
    /// CHECK: PDA used as treasury authority
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = territory_authority,
    )]
    pub territory_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Territory authority PDA
    pub territory_authority: UncheckedAccount<'info>,
    
    /// CHECK: Territory owner
    pub territory_owner: UncheckedAccount<'info>,
    
    pub mob_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"treasury_authority"],
        bump
    )]
    /// CHECK: PDA used as treasury authority
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Reward recipient
    pub recipient: UncheckedAccount<'info>,
    
    pub mob_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DistributeStakingRewards<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        associated_token::mint = fam_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_fam_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"treasury_authority"],
        bump
    )]
    /// CHECK: PDA used as treasury authority
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = fam_mint,
        associated_token::authority = recipient,
    )]
    pub recipient_fam_account: Account<'info, TokenAccount>,
    
    /// CHECK: Reward recipient
    pub recipient: UncheckedAccount<'info>,
    
    pub fam_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = fam_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_fam_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"treasury_authority"],
        bump
    )]
    /// CHECK: PDA used as treasury authority
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mob_mint,
        associated_token::authority = authority,
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = fam_mint,
        associated_token::authority = authority,
    )]
    pub authority_fam_account: Account<'info, TokenAccount>,
    
    pub mob_mint: Account<'info, Mint>,
    pub fam_mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetTreasuryStats<'info> {
    #[account(
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        associated_token::mint = mob_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        associated_token::mint = fam_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_fam_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"treasury_authority"],
        bump
    )]
    /// CHECK: PDA used as treasury authority
    pub treasury_authority: UncheckedAccount<'info>,
    
    pub mob_mint: Account<'info, Mint>,
    pub fam_mint: Account<'info, Mint>,
    pub requester: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub authority: Pubkey,
    pub mob_mint: Pubkey,
    pub fam_mint: Pubkey,
    pub total_fees_collected: u64,
    pub total_rewards_distributed: u64,
    pub marketplace_fee_rate: u16, // Basis points (100 = 1%)
    pub territory_tax_rate: u16,   // Basis points (100 = 1%)
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TreasuryStats {
    pub total_fees_collected: u64,
    pub total_rewards_distributed: u64,
    pub current_mob_balance: u64,
    pub current_fam_balance: u64,
    pub marketplace_fee_rate: u16,
    pub territory_tax_rate: u16,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum FeeType {
    Marketplace,
    TerritoryTax,
    WeaponUpgrade,
    CharacterRespawn,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum RewardType {
    Mission,
    PvpVictory,
    TerritoryDefense,
    Achievement,
    Daily,
    Tournament,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum TokenType {
    MOB,
    FAM,
}

fn validate_reward_distribution(amount: u64, reward_type: &RewardType) -> Result<()> {
    let max_amount = match reward_type {
        RewardType::Mission => 2_500_000_000, // 2.5 MOB
        RewardType::PvpVictory => 2_000_000_000, // 2 MOB
        RewardType::TerritoryDefense => 1_000_000_000, // 1 MOB
        RewardType::Achievement => 5_000_000_000, // 5 MOB
        RewardType::Daily => 1_000_000_000, // 1 MOB
        RewardType::Tournament => 10_000_000_000, // 10 MOB
    };
    
    require!(amount <= max_amount, ErrorCode::ExcessiveRewardAmount);
    Ok(())
}

#[event]
pub struct FeeCollected {
    pub fee_type: FeeType,
    pub amount: u64,
    pub payer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct RewardDistributed {
    pub recipient: Pubkey,
    pub amount: u64,
    pub reward_type: RewardType,
    pub timestamp: i64,
}

#[event]
pub struct StakingRewardDistributed {
    pub recipient: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct EmergencyWithdrawal {
    pub authority: Pubkey,
    pub amount: u64,
    pub token_type: TokenType,
    pub timestamp: i64,
}

#[event]
pub struct TreasuryStatsRequested {
    pub requester: Pubkey,
    pub stats: TreasuryStats,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Treasury is currently inactive")]
    TreasuryInactive,
    #[msg("Invalid amount specified")]
    InvalidAmount,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Excessive reward amount")]
    ExcessiveRewardAmount,
    #[msg("Excessive fee rate")]
    ExcessiveFeeRate,
    #[msg("Excessive tax rate")]
    ExcessiveTaxRate,
}
