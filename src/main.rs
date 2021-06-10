use ethcontract::prelude::*;

mod cli;
mod contracts;

#[tokio::main]
async fn main() -> Result<(), ethcontract::errors::DeployError> {
    let app = clap::App::new("Scam-ICO")
        .about("Use CLI to spend your precious ETH and get some 💩!")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            clap::Arg::with_name("host")
                .long("host")
                .short("h")
                .takes_value(true)
                .value_name("url")
                .default_value("http://localhost:8545")
                .help("endpoint for ethereum node"),
        )
        .subcommand(
            clap::SubCommand::with_name("deploy")
                .about("deploy a new ICO contract")
                .arg(
                    clap::Arg::with_name("weth")
                        .long("weth")
                        .takes_value(true)
                        .value_name("address")
                        .help("address of the WETH contract"),
                )
                .arg(
                    clap::Arg::with_name("deploy-weth")
                        .long("deploy-weth")
                        .conflicts_with("weth")
                        .help("deploy new weth contract"),
                ),
        );

    let matches = app.get_matches();

    let account = std::env::var("ETH_ACCOUNT")
        .expect("environment variable ETH_ACCOUNT must be present")
        .parse()
        .expect("environment variable ETH_ACCOUNT must contain valid address");

    let password =
        std::env::var("ETH_PASSWORD").expect("environment variable ETH_PASSWORD must be present");

    let transport = Http::new(matches.value_of("host").unwrap()).expect("http connection failed");
    let web3 = Web3::new(transport);

    web3.personal()
        .unlock_account(account, &password, None)
        .await?;

    match matches.subcommand() {
        ("deploy", Some(sub_m)) => deploy(&web3, sub_m, account).await,
        (cmd, _) => panic!("unknown command {:?}", cmd),
    }
}

async fn deploy(
    web3: &Web3<Http>,
    args: &clap::ArgMatches<'static>,
    address: Address,
) -> Result<(), ethcontract::errors::DeployError> {
    let weth = {
        if args.is_present("deploy-weth") {
            println!("Deploying WETH...");
            contracts::WETH9::builder(&web3).deploy().await?.address()
        } else if let Some(address) = args.value_of("weth") {
            address
                .parse()
                .expect("invalid address for --weth argument")
        } else {
            match web3.eth().chain_id().await?.as_u64() {
                // Mainnet
                1 => "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
                    .parse()
                    .unwrap(),

                // Ropsten
                3 => "0xc778417e063141139fce010982780140aa0cd5ab"
                    .parse()
                    .unwrap(),

                // Rinkeby
                4 => "0xc778417e063141139fce010982780140aa0cd5ab"
                    .parse()
                    .unwrap(),

                // Kovan
                42 => "0xd0a1e359811322d97991e03f863a0c30c2cf029c"
                    .parse()
                    .unwrap(),

                id => panic!(
                    "unknown canonical WETH address for net id {}; \
                    please specify WETH contract manually with --weth flag",
                    id
                ),
            }
        }
    };

    println!("Using WETH implementation at {:?}", weth);
    println!("ICO will send ether to {:?}", address);
    println!("Deploying ICO...");

    let ico = contracts::ICO::builder(&web3, weth, address)
        .deploy()
        .await?;

    println!("ICO deployed at {:?}", ico.address());

    Ok(())
}
