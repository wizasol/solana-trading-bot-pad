use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_lang::system_program;

declare_id!("RoyTkn11111111111111111111111111111111111");

#[program]
pub mod royalty_tokenization {
    use super::*;

    /// Initialize a new royalty token project
    pub fn initialize_project(
        ctx: Context<InitializeProject>,
        name: String,
        symbol: String,
        total_supply: u64,
        royalty_percentage: u8, // Percentage of revenue to distribute (0-100)
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let clock = Clock::get()?;

        project.artist = ctx.accounts.artist.key();
        project.mint = ctx.accounts.mint.key();
        project.name = name;
        project.symbol = symbol;
        project.total_supply = total_supply;
        project.royalty_percentage = royalty_percentage;
        project.total_distributed = 0;
        project.created_at = clock.unix_timestamp;
        project.bump = ctx.bumps.project;

        // Initialize treasury PDA (if not already initialized)
        let treasury_bump = ctx.bumps.treasury;
        let treasury_seeds = &[
            b"treasury",
            project.key().as_ref(),
            &[treasury_bump],
        ];
        let treasury_signer = &[&treasury_seeds[..]];

        // Create treasury account if needed (with minimum rent)
        let rent = Rent::get()?;
        let min_rent = rent.minimum_balance(0);
        if ctx.accounts.treasury.lamports() == 0 {
            anchor_lang::solana_program::program::invoke_signed(
                &anchor_lang::solana_program::system_instruction::transfer(
                    &ctx.accounts.artist.key(),
                    &ctx.accounts.treasury.key(),
                    min_rent,
                ),
                &[
                    ctx.accounts.artist.to_account_info(),
                    ctx.accounts.treasury.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                treasury_signer,
            )?;
        }

        msg!("Project initialized: {} ({})", project.name, project.symbol);
        Ok(())
    }

    /// Buy royalty tokens
    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        let project = &ctx.accounts.project;
        
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Transfer SOL from buyer to project treasury
        let treasury_bump = ctx.bumps.treasury;
        let treasury_seeds = &[
            b"treasury",
            project.key().as_ref(),
            &[treasury_bump],
        ];
        let treasury_signer = &[&treasury_seeds[..]];

        anchor_lang::solana_program::program::invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.buyer.key(),
                &ctx.accounts.treasury.key(),
                amount,
            ),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Calculate tokens to mint (1:1 ratio, can be adjusted)
        let tokens_to_mint = amount; // 1 SOL = 1 token (adjustable)

        // Mint tokens to buyer
        let project_seeds = &[
            b"project",
            project.artist.as_ref(),
            &[project.bump],
        ];
        let project_signer = &[&project_seeds[..]];

        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.project.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, project_signer);
        token::mint_to(cpi_ctx, tokens_to_mint)?;

        msg!("Bought {} tokens for {} lamports", tokens_to_mint, amount);
        Ok(())
    }

    /// Distribute royalty payments to token holders
    pub fn distribute_royalties(
        ctx: Context<DistributeRoyalties>,
        amount: u64,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(
            ctx.accounts.payer.key() == project.artist || 
            ctx.accounts.payer.key() == ctx.accounts.oracle.key(),
            ErrorCode::Unauthorized
        );

        // Calculate distribution amount based on royalty percentage
        let distribution_amount = (amount as u128)
            .checked_mul(project.royalty_percentage as u128)
            .and_then(|x| x.checked_div(100))
            .ok_or(ErrorCode::MathOverflow)? as u64;

        // Transfer from payer to treasury
        let treasury_bump = ctx.bumps.treasury;
        let treasury_seeds = &[
            b"treasury",
            project.key().as_ref(),
            &[treasury_bump],
        ];
        let treasury_signer = &[&treasury_seeds[..]];

        anchor_lang::solana_program::program::invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.payer.key(),
                &ctx.accounts.treasury.key(),
                distribution_amount,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        project.total_distributed = project
            .total_distributed
            .checked_add(distribution_amount)
            .ok_or(ErrorCode::MathOverflow)?;

        msg!("Distributed {} lamports in royalties", distribution_amount);
        Ok(())
    }

    /// Claim royalty payments (proportional to token holdings)
    pub fn claim_royalties(ctx: Context<ClaimRoyalties>) -> Result<()> {
        let project = &ctx.accounts.project;
        let token_account = &ctx.accounts.holder_token_account;
        let holder = &ctx.accounts.holder;

        // Get token balance
        let token_balance = token_account.amount;
        require!(token_balance > 0, ErrorCode::NoTokens);

        // Calculate share based on token holdings
        let total_supply = ctx.accounts.mint.supply;
        let treasury_balance = ctx.accounts.treasury.lamports();
        let share = (token_balance as u128)
            .checked_mul(treasury_balance as u128)
            .and_then(|x| x.checked_div(total_supply as u128))
            .ok_or(ErrorCode::MathOverflow)? as u64;

        require!(share > 0, ErrorCode::InsufficientFunds);

        // Transfer SOL from treasury to holder
        let treasury_bump = ctx.bumps.treasury;
        let treasury_seeds = &[
            b"treasury",
            project.key().as_ref(),
            &[treasury_bump],
        ];
        let treasury_signer = &[&treasury_seeds[..]];

        anchor_lang::solana_program::program::invoke_signed(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.treasury.key(),
                &holder.key(),
                share,
            ),
            &[
                ctx.accounts.treasury.to_account_info(),
                holder.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            treasury_signer,
        )?;

        msg!("Claimed {} lamports for {} tokens", share, token_balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProject<'info> {
    #[account(mut)]
    pub artist: Signer<'info>,

    #[account(
        init,
        payer = artist,
        space = 8 + Project::LEN,
        seeds = [b"project", artist.key().as_ref()],
        bump
    )]
    pub project: Account<'info, Project>,

    #[account(
        init,
        payer = artist,
        mint::decimals = 9,
        mint::authority = project,
        seeds = [b"mint", project.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"treasury", project.key().as_ref()],
        bump
    )]
    /// CHECK: Treasury PDA
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        seeds = [b"project", project.artist.as_ref()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        seeds = [b"mint", project.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasury", project.key().as_ref()],
        bump
    )]
    /// CHECK: Treasury PDA
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeRoyalties<'info> {
    #[account(
        seeds = [b"project", project.artist.as_ref()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Oracle account (can be set to allow automated distributions)
    pub oracle: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"treasury", project.key().as_ref()],
        bump
    )]
    /// CHECK: Treasury PDA
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRoyalties<'info> {
    #[account(
        seeds = [b"project", project.artist.as_ref()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        seeds = [b"mint", project.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = holder
    )]
    pub holder_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(
        mut,
        seeds = [b"treasury", project.key().as_ref()],
        bump
    )]
    /// CHECK: Treasury PDA
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Project {
    pub artist: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub royalty_percentage: u8,
    pub total_distributed: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Project {
    pub const LEN: usize = 32 + 32 + 4 + 50 + 4 + 10 + 8 + 1 + 8 + 1; // Adjust based on string sizes
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("No tokens held")]
    NoTokens,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
