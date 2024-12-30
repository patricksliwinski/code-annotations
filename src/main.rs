use std::ffi::OsString;
use uuid::Uuid;

use clap::{Args, Parser, Subcommand, ValueEnum};

mod annotation;

#[derive(Debug, Parser)]
#[command(name = "code-annotations")]
#[command(about = "CLI for bookmarking and annotating code", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to annotations file
    #[arg(short, long, default_value = "~/.annotations")]
    file: Option<OsString>
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add an annotation
    Add {
        /// File path
        path: OsString,

        /// Line number to annotate
        line: u64,

        /// Annotation text
        text: String
    },

    /// Remove an annotation
    Remove {
        /// File path
        path: OsString,

        /// Line number
        line: u64,
    },

    /// Remove by uuid
    RemoveUUID {
        uuid: Uuid,
    },

    /// Get annotations
    Get {
        /// File path
        #[arg(short, long)]
        path: Option<OsString>,

        /// Line number
        #[arg(short, long)]
        line: Option<u64>,

        #[arg(short, long)]
        uuid: Option<Uuid> 
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Add { path, line, text } => {}
        Commands::Remove { path, line } => {}
        Commands::RemoveUUID { uuid } => {}
        Commands::Get { path, line, uuid } => {}
    }
}
