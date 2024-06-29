mod database;




use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Get the balance of an address
    Balance(BalanceCmd),
    /// Mine a new block
    Mine(MineCmd),
    /// Run the miner
    Miner(MinerCmd),
    /// Node Operations
    Node(NodeCmd),
    /// Wallet Operations
    Wallet(WalletCmd),
}

#[derive(Parser, Debug)]
struct BalanceCmd {
    address: String,
}

#[derive(Parser, Debug)]
struct MineCmd {

    address: String,
}

#[derive(Parser, Debug)]
struct MinerCmd {

    address: String,

    threads: Option<usize>,
}

#[derive(Parser, Debug)]
struct NodeCmd {
    #[clap(subcommand)]
    subcmd: NodeSubCommand,
}

#[derive(Parser, Debug)]
enum NodeSubCommand {

    Start,

    Add,
}

#[derive(Parser, Debug)]
struct WalletCmd {
    name: String,
    #[clap(subcommand)]
    subcmd: WalletSubCommand,
}

#[derive(Parser, Debug)]
enum WalletSubCommand {
    /// Generate a new wallet
    New,
    /// Show the balance of the current wallet
    Balance,
    /// Show the address of the current wallet
    Address,
    /// Send coins to an address
    Send(SendCmd),
}

#[derive(Parser, Debug)]
struct SendCmd {
    to_address: String,

    amount: f64,
}

fn main() {
    let opts: Cli = Cli::parse();

    match opts.command {
        Commands::Balance(cmd) => {
            println!("Getting balance for address: {}", cmd.address);
            // Implement logic for 'balance' command here
        }
        Commands::Mine(cmd) => {
            println!("Mining with address: {}", cmd.address);
            // Implement logic for 'mine' command here
        }
        Commands::Miner(cmd) => {
            if let Some(threads) = cmd.threads {
                println!("Starting mining with address: {}, threads: {}", cmd.address, threads);
            } else {
                println!("Starting mining with address: {}", cmd.address);
            }
            // Implement logic for 'miner' command here
        }
        Commands::Node(subcmd) => match subcmd.subcmd {
            NodeSubCommand::Start => {
                println!("Starting node");
                // Implement logic for 'node start' command here
            }
            NodeSubCommand::Add => {
                println!("Adding address to node");
                // Implement logic for 'node add' command here
            }
        },
        Commands::Wallet(cmd) => match cmd.subcmd {
            WalletSubCommand::New => {
                println!("name: {}, subcmd: {:?}", cmd.name, cmd.subcmd);
                println!("Generating new wallet with name: {}", cmd.name);
                // Implement logic for 'wallet new' command here
            }
            WalletSubCommand::Balance => {
                println!("Getting balance for current wallet");
                // Implement logic for 'wallet balance' command here
            }
            WalletSubCommand::Address => {
                println!("Getting address for current wallet");
                // Implement logic for 'wallet address' command here
            }
            WalletSubCommand::Send(subcmd) => {
                println!("Sending {} from wallet {} to address {}", subcmd.amount, cmd.name, subcmd.to_address);
                // Implement logic for 'wallet send' command here
            }
        }
    }
}