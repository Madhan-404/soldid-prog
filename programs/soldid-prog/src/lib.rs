use anchor_lang::prelude::*;

declare_id!("CH1FbHbFuDP6CjTzszbYW3Jn7rE8mQzPBGCJKwFSomfV");

#[program]
pub mod soldid_prog {
    use super::*;

    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        cid: String,
        timestamp: String,
    ) -> Result<()> {
        let metadata_account = &mut ctx.accounts.metadata_account;
        metadata_account.owner = *ctx.accounts.owner.key;
        metadata_account.cid = cid;
        metadata_account.timestamp = timestamp;
        Ok(())
        
    }

    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        new_cid: String,
        new_timestamp: String,
    ) -> Result<()> {
        let metadata_account = &mut ctx.accounts.metadata_account;

        metadata_account.cid = new_cid;
        metadata_account.timestamp = new_timestamp;
        
        Ok(())
    }

    pub fn delete_metadata(ctx: Context<DeleteMetadata>) -> Result<()> {
        let _metadata_account = &mut ctx.accounts.metadata_account;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(cid:String)]
pub struct CreateMetadata<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + MetadataAccount::INIT_SPACE,
        seeds = [cid.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cid:String)]
pub struct UpdateMetadata<'info> {
    #[account(
        mut,
        seeds = [cid.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + MetadataAccount::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cid:String)]
pub struct DeleteMetadata<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [cid.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    #[account(mut)]
    pub owner: Signer<'info>, 

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MetadataAccount {
    pub owner: Pubkey,     
    #[max_len(50)]
    pub cid: String, 
    #[max_len(50)]      
    pub timestamp: String, 
}
