use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod linked_list {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.node.value = 1;
        ctx.accounts.node.next_node = Pubkey::default();
        Ok(())
    }

    pub fn add_node(ctx: Context<AddNode>) -> Result<()> {
        ctx.accounts.new_node.value = ctx.accounts.old_node.value + 1;
        ctx.accounts.new_node.next_node = Pubkey::default();

        ctx.accounts.old_node.next_node = ctx.accounts.new_node.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: I don't care
    #[account(signer, mut)]
    pub signer: AccountInfo<'info>,
    /// CHECK: I don't care
    #[account(init, 
        seeds = [signer.key.as_ref()], 
        bump, 
        payer = signer,
        space = 8 + 32 + 1)]
    pub node: Account<'info, Node>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct AddNode<'info> {
    /// CHECK: I don't care
    #[account(signer, mut)]
    pub signer: AccountInfo<'info>,
    /// CHECK: I don't care
    #[account(mut)]
    pub old_node: Account<'info, Node>,
    /// CHECK: I don't care
    #[account(init,
        payer = signer,
        space = 8 + 32 + 1)]
    pub new_node: Account<'info, Node>,
    pub system_program: Program<'info, System>,
}

/// Node Account for linked list
#[account]
#[derive(Copy)]
pub struct Node {
    /// Node value of linked list
    pub value: u8, // 1 byte

    // start liquidity
    pub next_node: Pubkey, // 32 bytes
}
