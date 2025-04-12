//! Commandline arguments to allow for doing things like host a server etc.

use clap::{
    Parser, Subcommand
};
use tracing::Level;

#[derive(Parser, Clone, PartialEq, Eq, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: MainCommand,

    #[arg(short, long, default_value_t = Level::INFO)]
    /// Tells the executable to print out additional debug info.
    pub log_level: Level,
}

#[derive(Subcommand, Clone, Default, PartialEq, Eq, Debug)]
pub enum MainCommand {
    #[default]
    /// Starts the executable as a client.
    Client,
    /// Starts the executable as a server.
    Server {
        #[command(subcommand)]
        command: ServerCommands,

        #[arg(short, long, default_value_t = 26550)]
        /// The port to host the server on.
        port: u16,
        #[arg(long, default_value_t = String::from("0.0.0.0"))]
        /// The ip to host on, leave as 0.0.0.0 to run on any ip.
        ip: String,
        #[arg(long, default_value_t = 50)]
        /// The maximum number of clients who can connect.
        max_players: u32,

        #[arg(long, default_value_t = false)]
        /// Tells the server to show a control pannel, usefull for less technical hosters.
        gui: bool
    },
}

#[derive(Subcommand, Clone, PartialEq, Eq, Debug)]
pub enum ServerCommands {
    /// Creates a new server with the specified name
    Create {
        #[arg(long)]
        /// The name of your new server
        name: String,
        #[arg(short, long, default_value_t = String::from("My amazing rust station!"))]
        /// The description of the new server (will show in the server browser)
        description: String,
    },
    /// Starts a server
    Start {
        #[arg(long)]
        /// The server to run
        server: String,
    },
    Wipe {
        #[arg(long)]
        /// The server to wipe
        server: String,
    },
    ConfigureDB {
        #[arg(long)]
        /// The server to configure a db for.
        for_server: String,
        
        #[arg(long)]
        address: String,
        #[arg(long)]
        port: u16,
        #[arg(long)]
        user: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        database: String,
    }
}