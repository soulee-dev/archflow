use clap::{Parser, Subcommand};
use std::fs;
use std::process;

#[derive(Parser)]
#[command(name = "archflow", about = "Archflow - Diagram as Code")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render a diagram IR JSON file to SVG
    Render {
        /// Input JSON IR file
        input: String,
        /// Output SVG file
        #[arg(short, long, default_value = "output.svg")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render { input, output } => {
            let json = match fs::read_to_string(&input) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error reading {}: {}", input, e);
                    process::exit(1);
                }
            };

            match archflow_core::render_svg(&json) {
                Ok(svg) => {
                    if let Err(e) = fs::write(&output, &svg) {
                        eprintln!("Error writing {}: {}", output, e);
                        process::exit(1);
                    }
                    println!("Rendered {} -> {}", input, output);
                }
                Err(e) => {
                    eprintln!("Render error: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
