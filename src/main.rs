mod block;
mod blockchain;
mod cli;
mod errors;
mod transaction;
pub mod wallet;
use crate::cli::Cli;

pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()>{
    // println!("Hello, world!");/
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
