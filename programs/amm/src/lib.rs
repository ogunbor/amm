use anchor_lang::prelude::*;

declare_id!("41mmmSzn8vrtStwdETeAQVqCKcRLqPHpQczXnvFnFZwK");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
