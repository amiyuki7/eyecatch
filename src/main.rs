use clap::{Parser, Subcommand};
use eyecatch::*;

#[derive(Parser, Debug)]
#[command(name = "eyecatch")]
#[command(author = "amiyuki7 <amiyuki788@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "An eye-catching CLI TODO", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new item to the TODO list
    Add { item: String },
    /// Delete an item from the TODO list
    Delete { idx: u32 },
    /// Displays the TODO list
    List,
}

fn main() -> Result<(), Box<dyn::std::error::Error>> {
    verify()?;

    let mut items = deserialize()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { item } => items.push(Item::new(item.into())),
        Commands::Delete { idx } => {}
        Commands::List => {}
    }

    println!("{cli:?}");

    serialize(items);

    Ok(())
}
