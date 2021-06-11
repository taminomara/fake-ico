use structopt::StructOpt;

use ethcontract::prelude::*;

mod cli;
mod contracts;
mod erc20;

#[derive(StructOpt)]
#[structopt(about = "Use CLI to spend your precious ETH and get some 💩")]
struct Opts {
    #[structopt(
        short,
        long,
        default_value = "http://localhost:8545",
        help = "endpoint for ethereum node"
    )]
    transport: String,

    #[structopt(subcommand)]
    subcommand: SubCommand,
}

#[derive(StructOpt)]
enum SubCommand {
    Scm(erc20::ScmCommand),
    Weth(erc20::WethCommand),
    Ico,
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();

    let account = get_account();
    let transport = Http::new(&opts.transport).expect("http connection failed");
    let web3 = Web3::new(transport);

    match opts.subcommand {
        SubCommand::Scm(scm) => scm.invoke(account, &web3).await,
        SubCommand::Weth(weth) => weth.invoke(account, &web3).await,
        SubCommand::Ico => todo!()
    };
}

fn get_account() -> Account {
    if let Ok(pk) = std::env::var("ETH_PK") {
        let pk = PrivateKey::from_hex_str(pk).expect("invalid private key");
        Account::Offline(pk, None)
    } else {
        let address = std::env::var("ETH_ACCOUNT")
            .expect("environment variable ETH_PK or ETH_ACCOUNT must be present")
            .parse()
            .expect("environment variable ETH_ACCOUNT must contain valid address");

        let password = std::env::var("ETH_PASSWORD")
            .expect("environment variable ETH_PASSWORD must be present");

        Account::Locked(address, Password::new(password), None)
    }
}
