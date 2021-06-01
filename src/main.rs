use ethcontract::transport::DynTransport;
use clap::ArgMatches;

mod contracts;

async fn create_transport(schema: &str) -> Result<DynTransport, web3::Error> {
    use web3::transports::{Http, Ipc};
    match schema.split_once("://") {
        Some(("http", _)) => Http::new(schema).map(DynTransport::new),
        Some(("https", _)) => Http::new(schema).map(DynTransport::new),
        Some(("ipc", path)) => Ipc::new(path).await.map(DynTransport::new),
        _ => Err(web3::Error::Transport("invalid transport schema".into())),
    }
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let app = clap::App::new("Scam-ICO")
        .about("Use CLI to spend your precious ETH and get some 💩!")
        .arg(
            clap::Arg::with_name("transport")
                .long("transport")
                .short("t")
                .required(true)
                .takes_value(true)
                .value_name("transport")
                .help(
                    "transport that's used to connect to an ethereum node; \
                    we support 'https://' and 'ipc://'",
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("deploy")
                .about("deploy a new ICO contract")
        );

    let matches = app.get_matches();

    let transport = create_transport(matches.value_of("transport").unwrap()).await?;
    let web3 = web3::Web3::new(transport);

    // match matches.subcommand() {
    //     ("deploy",  Some(sub_m)) => {},
    //     (_, _) => {}
    // }
    //
    // println!("Calling accounts.");
    // let accounts = web3.eth().accounts().await?;
    // println!("Accounts: {:?}", accounts);

    // use contracts::ICO;
    // let c = contracts::ICO::deployed();

    contracts::ICO::builder(&web3, "".parse().unwrap(), "".parse().unwrap());

    // println!("Calling accounts.");
    // let mut accounts = web3.eth().accounts().await?;
    // println!("Accounts: {:?}", accounts);

    //ipc:///Users/taminomara/Documents/eth/data/geth.ipc
    // println!("Calling balance.");
    // let balance = web3.eth().balance("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap(), None).await?;
    // println!("Balance: {}", balance);

    Ok(())
}
