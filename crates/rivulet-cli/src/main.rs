use clap::{Parser, Subcommand};
use rivulet_core::{push_pull, ActorId, Document, OpPayload};
use serde_json::json;

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
    Relay {
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Two in-memory peers edit, then sync, then print op counts.
    Demo,
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
        Commands::Inspect { path } => println!("inspecting {path} ... (no on-disk format yet)"),
        Commands::Relay { port } => println!("relay would listen on :{port} here"),
        Commands::Demo => {
            let alice = ActorId::new();
            let bob = ActorId::new();
            let mut a = Document::new();
            let mut b = Document::new();
            b.id = a.id;
            a.local_op(
                alice,
                OpPayload::MapSet {
                    key: "title".into(),
                    value: json!("notes"),
                },
            );
            b.local_op(
                bob,
                OpPayload::MapSet {
                    key: "mood".into(),
                    value: json!("offline"),
                },
            );
            println!("before sync alice={} bob={}", a.ops.len(), b.ops.len());
            push_pull(&mut a, &mut b);
            println!("after sync alice={} bob={}", a.ops.len(), b.ops.len());
            assert_eq!(a.ops.len(), 2);
            assert_eq!(b.ops.len(), 2);
        }
    }
}
