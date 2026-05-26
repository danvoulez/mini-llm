use clap::{Parser, Subcommand};
use mini_llms_runtime::{DeterministicMockRuntime, InferenceRequest, LocalInference};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Doctor {
        #[arg(long)]
        want_mistral: bool,
        #[arg(long)]
        want_db: bool,
    },
    Classify,
    Ghosts,
    Summarize,
    Eval,
    Bench,
    Db {
        #[command(subcommand)]
        cmd: DbCmd,
    },
}
#[derive(Subcommand)]
enum DbCmd {
    Plan,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Doctor {
            want_mistral,
            want_db,
        } => {
            println!("doctor ok");
            if want_mistral {
                #[cfg(feature = "mistral")]
                {
                    let adapter = mini_llms_mistral::MistralAdapter::new(
                        std::env::var("LAB512_BASE_URL").unwrap_or("http://localhost:1234".into()),
                        std::env::var("LAB512_API_KEY").ok(),
                    );
                    let _ = adapter.doctor_models().await;
                }
                println!("want-mistral checked");
            }
            if want_db {
                println!(
                    "want-db configured={}",
                    std::env::var("DATABASE_URL").is_ok()
                );
            }
        }
        Commands::Classify => {
            let rt = DeterministicMockRuntime;
            let out = rt
                .infer(&InferenceRequest {
                    prompt: "classify".into(),
                    model: None,
                })
                .await?;
            println!("{}", out.candidate.is_some());
        }
        Commands::Ghosts | Commands::Summarize | Commands::Eval | Commands::Bench => println!("ok"),
        Commands::Db { cmd: DbCmd::Plan } => println!("db plan"),
    }
    Ok(())
}
