use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rivulet", about = "local-first sync engine cli")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version,
    Inspect { path: String },
    Relay { #[arg(long, default_value = "8080")] port: u16 },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Version => {
            println!("rivulet {}", env!("CARGO_PKG_VERSION"));
            println!("still local-first. still a bit chaotic. ship it anyway.");
        }
        Commands::Inspect { path } => println!("inspecting {path} ... (stub)"),
        Commands::Relay { port } => println!("relay would listen on :{port} here"),
    }
}
