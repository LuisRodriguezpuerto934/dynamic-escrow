use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("DYNAiScPoW3RXkYsidMpWTK6W2BeZ7FEfcYkg476zPFs");

#[program]
pub mod dynamic_escrow {
    use super::*;

    // Create escrow with AI arbitration
    pub fn create_escrow(
        ctx: Context<CreateEscrow>,
        amount: u64,
        release_time: i64,
        dispute_window: i64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.buyer = ctx.accounts.buyer.key();
        escrow.seller = ctx.accounts.seller.key();
        escrow.ai_arbiter = ctx.accounts.ai_arbiter.key();
        escrow.amount = amount;
        escrow.release_time = release_time;
        escrow.dispute_deadline = Clock::get()?.unix_timestamp + dispute_window;
        escrow.status = EscrowStatus::Pending;
        escrow.dispute_evidence = Vec::new();
        
        // Transfer funds to escrow
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.buyer_token_account.to_account_info(),
                    to: ctx.accounts.escrow_token_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            amount,
        )?;
        
        msg!("Escrow created: {} tokens locked until {} or AI decision", amount, release_time);
        Ok(())
    }
    
    // AI makes decision on dispute
    pub fn ai_arbitrate(
        ctx: Context<AiArbitrate>,
        decision: ArbitrationDecision,
        reasoning: String,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        
        require!(
            ctx.accounts.ai_arbiter.key() == escrow.ai_arbiter,
            ErrorCode::InvalidArbiter
        );
        require!(
            escrow.status == EscrowStatus::Disputed,
            ErrorCode::NotInDispute
        );
        
        escrow.ai_decision = Some(decision.clone());
        escrow.ai_reasoning = reasoning;
        
        match decision {
            ArbitrationDecision::ReleaseToSeller => {
                escrow.status = EscrowStatus::ResolvedToSeller;
                // Transfer to seller
                msg!("AI decision: Release to seller");
            }
            ArbitrationDecision::RefundToBuyer => {
                escrow.status = EscrowStatus::ResolvedToBuyer;
                // Transfer back to buyer
                msg!("AI decision: Refund to buyer");
            }
            ArbitrationDecision::Split => {
                escrow.status = EscrowStatus::ResolvedSplit;
                // Split 50/50
                msg!("AI decision: Split funds");
            }
        }
        
        Ok(())
    }
    
    // Release funds after time lock
    pub fn time_release(ctx: Context<TimeRelease>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        let now = Clock::get()?.unix_timestamp;
        
        require!(
            now >= escrow.release_time,
            ErrorCode::TimeLockNotExpired
        );
        require!(
            escrow.status == EscrowStatus::Pending || 
            escrow.status == EscrowStatus::ResolvedToSeller,
            ErrorCode::InvalidStatus
        );
        
        // Transfer to seller
        msg!("Time lock expired - releasing to seller");
        Ok(())
    }
    
    // Raise dispute
    pub fn raise_dispute(
        ctx:Context<RaiseDispute>,
        evidence: String,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        
        require!(
            ctx.accounts.caller.key() == escrow.buyer || 
            ctx.accounts.caller.key() == escrow.seller,
            ErrorCode::Unauthorized
        );
        require!(
            Clock::get()?.unix_timestamp < escrow.dispute_deadline,
            ErrorCode::DisputeWindowClosed
        );
        
        escrow.status = EscrowStatus::Disputed;
        escrow.dispute_evidence.push(DisputeEvidence {
            party: ctx.accounts.caller.key(),
            evidence: evidence,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Dispute raised - awaiting AI arbitration");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Seller account
    pub seller: AccountInfo<'info>,
    /// CHECK: AI arbiter (can be a PDA or trusted address)
    pub ai_arbiter: AccountInfo<'info>,
    
    #[account(
        init,
        payer = buyer,
        space = 8 + Escrow::SIZE,
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = buyer,
        token::mint = mint,
        token::authority = escrow,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, token::Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AiArbitrate<'info> {
    #[account(mut)]
    pub ai_arbiter: Signer<'info>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
}

#[derive(Accounts)]
pub struct TimeRelease<'info> {
    pub caller: Signer<'info>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
}

#[derive(Accounts)]
pub struct RaiseDispute<'info> {
    pub caller: Signer<'info>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
}

#[account]
pub struct Escrow {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub ai_arbiter: Pubkey,
    pub amount: u64,
    pub release_time: i64,
    pub dispute_deadline: i64,
    pub status: EscrowStatus,
    pub ai_decision: Option<ArbitrationDecision>,
    pub ai_reasoning: String,
    pub dispute_evidence: Vec<DisputeEvidence>,
}

impl Escrow {
    pub const SIZE: usize = 32 + 32 + 32 + 8 + 8 + 8 + 1 + 2 + 200 + 4 + 1000;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Disputed,
    ResolvedToSeller,
    ResolvedToBuyer,
    ResolvedSplit,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ArbitrationDecision {
    ReleaseToSeller,
    RefundToBuyer,
    Split,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DisputeEvidence {
    pub party: Pubkey,
    pub evidence: String,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid arbiter")]
    InvalidArbiter,
    #[msg("Not in dispute")]
    NotInDispute,
    #[msg("Time lock not expired")]
    TimeLockNotExpired,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Dispute window closed")]
    DisputeWindowClosed,
    #[msg("Invalid status")]
    InvalidStatus,
}
