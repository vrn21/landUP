use anchor_lang::prelude::*;

declare_id!("2gViFt82VWBLbSDoF8xfT78Hb39AoJhGHVwvkJUh9HJA");

#[program]
pub mod land_up {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
