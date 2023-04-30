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
    Add { item: String },
    Delete { idx: String },
    List,
}

fn main() -> Result<(), Box<dyn::std::error::Error>> {
    verify()?;

    let mut items = deserialize()?;
    let ids: Vec<String> = items.iter().map(|item| item.id.clone()).collect();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { item } => items.push(Item::new(item.into(), ids)),
        Commands::Delete { idx } => items.retain(|item| &item.id != idx),
        Commands::List => {}
    }

    println!("{cli:?}");

    serialize(items);

    Ok(())
}
