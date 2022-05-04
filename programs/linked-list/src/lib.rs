use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod linked_list {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn add_node(ctx: Context<AddNode>) -> Result<()> {
        let node_stack = &mut ctx.accounts.stack.load_mut()?; // Necessary to load zero_copy accounts

        if node_stack.idx >= 749 {
            panic!("Stack is full, better empty it out");
        }

        let index = node_stack.idx.clone();
        node_stack.nodes[index as usize] = Node {
            value: 21,
            foo: Pubkey::default(),
            bar: Pubkey::default(),
            baz: Pubkey::default(),
            ser: Pubkey::default(),
        };

        node_stack.idx += 1;

        Ok(())
    }

    pub fn iterate(ctx: Context<Iterate>) -> Result<()> {
        let node_stack = &mut ctx.accounts.stack.load()?; // Necessary to load zero_copy accounts

        let mut account_keys = Vec::new();

        for n in 0..200 { // 200 works
            account_keys.push(node_stack.nodes[n].foo)
        }

        Ok(())
    }

    pub fn break_stack(ctx: Context<Iterate>) -> Result<()> {
        let node_stack = &mut ctx.accounts.stack.load()?; // Necessary to load zero_copy accounts

        let mut account_keys = Vec::new();

        for n in 0..300 { // 300 is too much
            account_keys.push(node_stack.nodes[n].foo)
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: I don't care
    #[account(signer, mut)]
    pub signer: AccountInfo<'info>,
    /// CHECK: I don't care
    #[account(zero)]
    stack: AccountLoader<'info, NodeStack>,
}

#[derive(Accounts)]
pub struct AddNode<'info> {
    /// CHECK: I don't care
    #[account(signer)]
    pub signer: AccountInfo<'info>,

    /// CHECK: I don't care
    #[account(mut)]
    stack: AccountLoader<'info, NodeStack>,
}

#[derive(Accounts)]
pub struct Iterate<'info> {
    /// CHECK: I don't care
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    /// CHECK: I don't care
    stack: AccountLoader<'info, NodeStack>,
}

#[account(zero_copy)]
pub struct NodeStack {
    pub nodes: [Node; 750], // Max account size is 10 MB (1_000_000 bytes), here we have 750 * 129 = 967_500
    pub idx: u16,
}

/// Node Struct for array
#[zero_copy]
pub struct Node { // 129 bytes total
    pub value: u8, // 1 byte
    pub foo: Pubkey, // 32
    pub bar: Pubkey, // 32
    pub baz: Pubkey, // 32
    pub ser: Pubkey, // 32
}

// A separate type is used for the RPC interface for two main reasons.
//
// 1. AnchorSerialize and AnchorDeserialize must be derived. Anchor requires
//    *all* instructions to implement the AnchorSerialize and AnchorDeserialize
//    traits, so any types in method signatures must as well.
// 2. All types for zero copy deserialization are `#[repr(packed)]`. However,
//    the implementation of AnchorSerialize (i.e. borsh), uses references
//    to the fields it serializes. So if we were to just throw tehse derives
//    onto the other `Event` struct, we would have references to
//    `#[repr(packed)]` fields, which is unsafe. To avoid the unsafeness, we
//    just use a separate type.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RpcNode {
    pub value: u8,
    pub foo: Pubkey,
    pub bar: Pubkey, // 32
    pub baz: Pubkey, // 32
    pub ser: Pubkey, // 32
}

impl From<RpcNode> for Node {
    fn from(e: RpcNode) -> Node {
        Node {
            value: e.value,
            foo: e.foo,
            bar: e.bar,
            baz: e.baz,
            ser: e.ser,
        }
    }
}

