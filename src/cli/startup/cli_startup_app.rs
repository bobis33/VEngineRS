use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vengine_rs")]
#[command(about = "VEngine RS — 3D Vulkan Engine written in Rust", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run engine
    Run {
        /// width of the window
        #[arg(long, default_value_t = 1920)]
        width: u32,

        /// height of the window
        #[arg(long, default_value_t = 1080)]
        height: u32,
    },

    Info,
}

impl Cli {
    pub fn parse_cli() -> Self {
        Self::parse()
    }
}
