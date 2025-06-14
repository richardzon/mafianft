use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnW");

#[program]
pub mod fam_token {
    use super::*;

    /// Initialize the FAM governance token
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.mint = ctx.accounts.mint.key();
        config.total_supply = 100_000_000_000_000; // 100M tokens with 6 decimals
        config.circulating_supply = 0;
        config.is_active = true;
        config.min_stake_amount = 1_000_000; // 1 FAM minimum stake
        config.voting_period = 259200; // 3 days in seconds
        config.proposal_threshold = 10_000_000; // 10 FAM to create proposal
        
        msg!("FAM governance token initialized");
        Ok(())
    }

    /// Distribute initial FAM tokens (admin only)
    pub fn distribute_tokens(
        ctx: Context<DistributeTokens>,
        amount: u64,
        distribution_type: DistributionType,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(
            ctx.accounts.authority.key() == config.authority,
            ErrorCode::Unauthorized
        );
        
        // Check total supply limit
        require!(
            config.circulating_supply + amount <= config.total_supply,
            ErrorCode::ExceedsMaxSupply
        );
        
        // Validate distribution amount based on type
        validate_distribution(amount, &distribution_type, config.total_supply)?;
        
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
        
        config.circulating_supply += amount;
        
        emit!(TokensDistributed {
            recipient: ctx.accounts.recipient.key(),
            amount,
            distribution_type,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Distributed {} FAM tokens for {:?}", amount, distribution_type);
        Ok(())
    }

    /// Stake FAM tokens for governance voting power
    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        let config = &ctx.accounts.config;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(amount >= config.min_stake_amount, ErrorCode::InsufficientStakeAmount);
        
        let stake_account = &mut ctx.accounts.stake_account;
        let current_time = Clock::get()?.unix_timestamp;
        
        // Transfer tokens to stake account
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.stake_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        // Update stake account
        stake_account.user = ctx.accounts.user.key();
        stake_account.staked_amount += amount;
        stake_account.last_stake_time = current_time;
        stake_account.voting_power = calculate_voting_power(stake_account.staked_amount, current_time);
        
        emit!(TokensStaked {
            user: ctx.accounts.user.key(),
            amount,
            total_staked: stake_account.staked_amount,
            voting_power: stake_account.voting_power,
            timestamp: current_time,
        });
        
        msg!("Staked {} FAM tokens, voting power: {}", amount, stake_account.voting_power);
        Ok(())
    }

    /// Unstake FAM tokens (with cooldown period)
    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
        let config = &ctx.accounts.config;
        let stake_account = &mut ctx.accounts.stake_account;
        let current_time = Clock::get()?.unix_timestamp;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(
            ctx.accounts.user.key() == stake_account.user,
            ErrorCode::Unauthorized
        );
        require!(amount <= stake_account.staked_amount, ErrorCode::InsufficientStakedTokens);
        
        // Check cooldown period (7 days)
        let cooldown_period = 604800; // 7 days in seconds
        require!(
            current_time - stake_account.last_stake_time >= cooldown_period,
            ErrorCode::CooldownPeriodActive
        );
        
        // Transfer tokens back to user
        let seeds = &[
            b"stake_authority",
            &[ctx.bumps.stake_authority],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.stake_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        // Update stake account
        stake_account.staked_amount -= amount;
        stake_account.voting_power = calculate_voting_power(stake_account.staked_amount, current_time);
        
        emit!(TokensUnstaked {
            user: ctx.accounts.user.key(),
            amount,
            remaining_staked: stake_account.staked_amount,
            voting_power: stake_account.voting_power,
            timestamp: current_time,
        });
        
        msg!("Unstaked {} FAM tokens", amount);
        Ok(())
    }

    /// Create a governance proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let stake_account = &ctx.accounts.stake_account;
        
        require!(config.is_active, ErrorCode::ProgramInactive);
        require!(title.len() <= 64, ErrorCode::TitleTooLong);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(
            stake_account.staked_amount >= config.proposal_threshold,
            ErrorCode::InsufficientStakeForProposal
        );
        
        let proposal = &mut ctx.accounts.proposal;
        let current_time = Clock::get()?.unix_timestamp;
        
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title;
        proposal.description = description;
        proposal.proposal_type = proposal_type;
        proposal.created_at = current_time;
        proposal.voting_ends_at = current_time + config.voting_period;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.status = ProposalStatus::Active;
        
        emit!(ProposalCreated {
            proposal_id: proposal.key(),
            proposer: ctx.accounts.proposer.key(),
            title: proposal.title.clone(),
            proposal_type,
            voting_ends_at: proposal.voting_ends_at,
        });
        
        msg!("Created proposal: {}", proposal.title);
        Ok(())
    }

    /// Vote on a governance proposal
    pub fn vote_on_proposal(
        ctx: Context<VoteOnProposal>,
        vote: Vote,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let stake_account = &ctx.accounts.stake_account;
        let vote_record = &mut ctx.accounts.vote_record;
        let current_time = Clock::get()?.unix_timestamp;
        
        require!(
            matches!(proposal.status, ProposalStatus::Active),
            ErrorCode::ProposalNotActive
        );
        require!(
            current_time <= proposal.voting_ends_at,
            ErrorCode::VotingPeriodEnded
        );
        require!(
            stake_account.staked_amount > 0,
            ErrorCode::NoVotingPower
        );
        require!(
            vote_record.voter == Pubkey::default(), // First time voting
            ErrorCode::AlreadyVoted
        );
        
        let voting_power = stake_account.voting_power;
        
        // Record vote
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.proposal = proposal.key();
        vote_record.vote = vote;
        vote_record.voting_power = voting_power;
        vote_record.timestamp = current_time;
        
        // Update proposal vote counts
        match vote {
            Vote::Yes => proposal.yes_votes += voting_power,
            Vote::No => proposal.no_votes += voting_power,
        }
        
        emit!(VoteCast {
            proposal_id: proposal.key(),
            voter: ctx.accounts.voter.key(),
            vote,
            voting_power,
            timestamp: current_time,
        });
        
        msg!("Vote cast: {:?} with power {}", vote, voting_power);
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
        mint::decimals = 6,
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
pub struct DistributeTokens<'info> {
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
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Recipient of tokens
    pub recipient: UncheckedAccount<'info>,
    
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TokenConfig>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = stake_authority,
    )]
    pub stake_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"stake_authority"],
        bump
    )]
    /// CHECK: PDA used as stake authority
    pub stake_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TokenConfig>,
    
    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = stake_authority,
    )]
    pub stake_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"stake_authority"],
        bump
    )]
    /// CHECK: PDA used as stake authority
    pub stake_authority: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, TokenConfig>,
    
    #[account(
        seeds = [b"stake", proposer.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", proposer.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        seeds = [b"stake", voter.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        init,
        payer = voter,
        space = 8 + VoteRecord::INIT_SPACE,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct TokenConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub is_active: bool,
    pub min_stake_amount: u64,
    pub voting_period: i64,
    pub proposal_threshold: u64,
}

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub user: Pubkey,
    pub staked_amount: u64,
    pub voting_power: u64,
    pub last_stake_time: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub proposer: Pubkey,
    #[max_len(64)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    pub proposal_type: ProposalType,
    pub created_at: i64,
    pub voting_ends_at: i64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub status: ProposalStatus,
}

#[account]
#[derive(InitSpace)]
pub struct VoteRecord {
    pub voter: Pubkey,
    pub proposal: Pubkey,
    pub vote: Vote,
    pub voting_power: u64,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, InitSpace)]
pub enum DistributionType {
    TeamVesting,
    PublicSale,
    LiquidityBootstrap,
    EcosystemIncentives,
    CommunityTreasury,
    StakingRewards,
    Marketing,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, InitSpace)]
pub enum ProposalType {
    EconomicParameter,
    FeatureDevelopment,
    TreasuryAllocation,
    Partnership,
    Emergency,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, InitSpace)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, InitSpace)]
pub enum Vote {
    Yes,
    No,
}

fn validate_distribution(amount: u64, distribution_type: &DistributionType, total_supply: u64) -> Result<()> {
    let max_percentage = match distribution_type {
        DistributionType::TeamVesting => 20, // 20%
        DistributionType::PublicSale => 15, // 15%
        DistributionType::LiquidityBootstrap => 10, // 10%
        DistributionType::EcosystemIncentives => 10, // 10%
        DistributionType::CommunityTreasury => 30, // 30%
        DistributionType::StakingRewards => 10, // 10%
        DistributionType::Marketing => 5, // 5%
    };
    
    let max_amount = (total_supply * max_percentage) / 100;
    require!(amount <= max_amount, ErrorCode::ExceedsDistributionLimit);
    Ok(())
}

fn calculate_voting_power(staked_amount: u64, _current_time: i64) -> u64 {
    // Simple 1:1 voting power for now
    // Could add time-based multipliers later
    staked_amount
}

#[event]
pub struct TokensDistributed {
    pub recipient: Pubkey,
    pub amount: u64,
    pub distribution_type: DistributionType,
    pub timestamp: i64,
}

#[event]
pub struct TokensStaked {
    pub user: Pubkey,
    pub amount: u64,
    pub total_staked: u64,
    pub voting_power: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensUnstaked {
    pub user: Pubkey,
    pub amount: u64,
    pub remaining_staked: u64,
    pub voting_power: u64,
    pub timestamp: i64,
}

#[event]
pub struct ProposalCreated {
    pub proposal_id: Pubkey,
    pub proposer: Pubkey,
    pub title: String,
    pub proposal_type: ProposalType,
    pub voting_ends_at: i64,
}

#[event]
pub struct VoteCast {
    pub proposal_id: Pubkey,
    pub voter: Pubkey,
    pub vote: Vote,
    pub voting_power: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Program is currently inactive")]
    ProgramInactive,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Amount exceeds maximum supply")]
    ExceedsMaxSupply,
    #[msg("Amount exceeds distribution limit for this type")]
    ExceedsDistributionLimit,
    #[msg("Insufficient stake amount")]
    InsufficientStakeAmount,
    #[msg("Insufficient staked tokens")]
    InsufficientStakedTokens,
    #[msg("Cooldown period is still active")]
    CooldownPeriodActive,
    #[msg("Title too long")]
    TitleTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Insufficient stake to create proposal")]
    InsufficientStakeForProposal,
    #[msg("Proposal is not active")]
    ProposalNotActive,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("No voting power")]
    NoVotingPower,
    #[msg("Already voted on this proposal")]
    AlreadyVoted,
}
