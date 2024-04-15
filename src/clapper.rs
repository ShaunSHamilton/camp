use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// CLI that adds challenges to the freeCodeCamp database.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// MongoDB connection string
    #[arg(
        long,
        default_value = "mongodb://127.0.0.1:27017/freecodecamp?directConnection=true"
    )]
    pub uri: String,

    /// Username of user in the database
    #[arg(long, default_value = "developmentuser")]
    pub username: String,

    /// Path to `curriculum.json` file
    #[arg(short, long, default_value = "shared/config/curriculum.json")]
    pub curriculum_path: PathBuf,

    #[command(subcommand)]
    pub sub_commands: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Add all certification challenges to the user record
    ClaimCerts,
    /// Add selected challenges to user record
    AddChallenges,
    /// Add ALL challenges to completedChallenges array
    #[clap(name = "finishFCC")]
    FinishFreeCodeCamp,

    /// Add <n> random user records to the database
    AddUsers {
        /// Number of users to add
        #[clap(short, long)]
        count: u32,
    },
}
