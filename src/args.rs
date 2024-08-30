use clap::{Parser, Arg, Subcommand};



#[derive(Debug, Parser)]
#[clap(author, version, about = "A description of your application")]
pub struct SPMArgs {
    #[clap(subcommand)]
    pub command: SPMCommand,
}

#[derive(Debug, Subcommand)]
pub enum SPMCommand {
    /// Builds the script with the added dependencies.
    Build {
        /// The script file to build.
        file: String,
    },
    /// Adds the dependency to your config.spm file.
    Add {
        /// The name of the dependency to add.
        name: String,
    },
}