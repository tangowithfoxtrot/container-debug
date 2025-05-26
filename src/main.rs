use std::env;
use std::io::Write;

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::debug;

#[derive(Parser)]
#[command(version)]
/// Get an interactive shell into any container image
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Evaluate the specified commands instead, passing additional positional arguments through $argv.
    #[arg(long, short)]
    command_arg: Option<String>,

    /// Daemon docker socket to connect to. E.g.: 'ssh://root@example.org', 'unix:///some/path/docker.sock'
    #[arg(long)]
    host: Option<String>,

    /// Running containers only: Use same user as the running container. Default is root.
    #[arg(long)]
    preserve_user: bool,

    /// Running containers only: Give privileges to the shell (all capabilities).
    #[arg(long)]
    privileged: bool,

    /// Select a shell. Supported: "bash", "fish", "zsh", "auto". (default auto)
    #[arg(long)]
    shell: Option<String>,
}

struct CommandConfiguration {
    command_arg: String,
    host: String,
    preserve_user: bool,
    privileged: bool,
    shell: String,
}

impl CommandConfiguration {
    fn new(cli: &Cli) -> Self {
        let shell = cli.shell.clone().unwrap_or("/bin/sh".to_owned()); // TODO: get correct $SHELL automatically
        let command_arg = cli.command_arg.clone().unwrap_or(shell.clone()); // TODO: get correct $SHELL automatically
        let host = cli.host.clone().unwrap_or_else(|| {
            // TODO: don't hard-code the socket; determine it automatically
            "unix:///var/run/docker.sock".to_owned()
        });
        Self {
            command_arg,
            host,
            preserve_user: cli.preserve_user,
            privileged: cli.privileged,
            shell,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Required by the docker CLI to recognize this as a plugin
    #[command(hide = true)]
    DockerCliPluginMetadata,

    Rebug,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    debug!("Initialized logger");
    debug!("RUST_LOG={}", env::var("RUST_LOG").unwrap_or_default());

    let cli = Cli::parse();

    let command_config = CommandConfiguration::new(&cli);

    if let Some(command) = cli.command {
        match command {
            Commands::DockerCliPluginMetadata => {
                println!(
                            "{{\n  \"SchemaVersion\": \"0.1.0\",\n  \"Vendor\": \"tangowithfoxtrot\",\n  \"Version\": \"{}\",\n  \"ShortDescription\": \"Get an interactive shell into any container image\",\n  \"Url\": \"/dev/null\"\n}}",
                            env!("CARGO_PKG_VERSION")
                        )
            }
            Commands::Rebug => rebug(command_config),
        }
    };
    Ok(())
}

fn rebug(command_config: CommandConfiguration) {
    todo!()
}
