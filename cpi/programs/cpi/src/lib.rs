use anchor_lang::prelude::program::invoke_signed;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("7QXGPsDCT7T79ALzP8Ubk4RL2xiMBWM2nrHKy2pXByMf");

#[program]
pub mod cpi {

    use anchor_lang::prelude::instruction::Instruction;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        //get the pda signer seeds
        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

        //prepare instruction account_metas
        let account_metas = vec![
            AccountMeta::new(from_pubkey.key(), true),
            AccountMeta::new(to_pubkey.key(), false),
        ];

        /**
        * 0 = CreateAccount
         1 = Assign
        2 = Transfer
        3 = CreateAccountWithSeed
        4 = AdvanceNonceAccount
        5 = WithdrawNonceAccount
        */
        // SOL transfer instruction discriminator
        let instruction_discriminator: u32 = 2; //2 is opcode -> Transfer

        // Prepare instruction data
        let mut instruction_data = Vec::with_capacity(4 + 8);
        instruction_data.extend_from_slice(&instruction_discriminator.to_le_bytes());
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        // Create instruction
        let instruction = Instruction {
            program_id: program_id.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        // Invoke instruction with PDA signer
        invoke_signed(
            &instruction,
            &[from_pubkey, to_pubkey, program_id],
            signer_seeds,
        )?;
        Ok(())
    }
    pub fn sol_transfer_with_abstraction(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;

        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

        let instruction =
            &system_instruction::transfer(&from_pubkey.key(), &to_pubkey.key(), amount);

        invoke_signed(
            instruction,
            &[from_pubkey, to_pubkey, program_id],
            signer_seeds,
        )?;

        Ok(())
    }

    pub fn sol_transfer_by_anchor(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;

        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        )
        .with_signer(signer_seeds);

        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(
        mut,
        seeds = [b"pda", recipient.key().as_ref()],
        bump,
    )]
    pda_account: SystemAccount<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>,

    system_program: Program<'info, System>,
}
