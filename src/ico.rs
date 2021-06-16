use ethcontract::prelude::*;

use crate::cli::{Currency, Eth};
use chrono::{NaiveDateTime, TimeZone, Utc};
use ethcontract::batch::CallBatch;
use futures::StreamExt as _;

#[derive(structopt::StructOpt)]
#[structopt(about = "Participate in SCM ICO")]
pub enum IcoCommand {
    #[structopt(about = "Get status of the ICO")]
    Info,

    #[structopt(about = "Get number of SCM tokens available to the given user")]
    Balance {
        #[structopt(help = "Account we're fetching balance for (uses your account by default)")]
        address: Option<Address>,
        #[structopt(long, about = "Display balance in ETH")]
        eth: bool,
    },

    #[structopt(about = "Buy SCM")]
    Fund {
        #[structopt(
            short,
            long,
            about = "If ICO doesn't have enough tokens, buy all available ones"
        )]
        force: bool,
        #[structopt(long, about = "Wrap eth if you don't have enough of it")]
        wrap_weth: bool,
        #[structopt(long, about = "Ensure that ICO is authorized to spend WETH")]
        approve_weth: bool,
        #[structopt(help = "Number of ETH tokens to contribute to the ICO")]
        funds: Eth,
    },

    #[structopt(about = "Claim purchased SCM")]
    Claim {
        #[structopt(long, about = "If ICO is not finished, wait for it")]
        wait: bool,
    },

    #[structopt(about = "Wait for ICO to finish")]
    Wait,
}

impl IcoCommand {
    pub async fn invoke(&self, account: Account, web3: &Web3<Http>) {
        let contract_address = crate::contracts::get_ico_address(web3).await;
        let contract = crate::contracts::ICO::at(web3, contract_address);

        match self {
            IcoCommand::Info => {
                let current_block = web3.eth().block_number().await.unwrap();
                let current_block = BlockId::Number(BlockNumber::Number(current_block));

                let mut batch = CallBatch::new(web3.transport());

                let state = contract
                    .state()
                    .block(current_block)
                    .batch_call(&mut batch);
                let left_eth = contract
                    .left_eth()
                    .block(current_block)
                    .batch_call(&mut batch);
                let left_scm = contract
                    .left_scm()
                    .block(current_block)
                    .batch_call(&mut batch);
                let scm = contract
                    .scm()
                    .block(current_block)
                    .batch_call(&mut batch);
                let weth = contract
                    .weth()
                    .block(current_block)
                    .batch_call(&mut batch);

                batch.execute_all(100).await;

                let state = state.await.expect("state call failed");
                match state {
                    0x0 => println!("state: ongoing"),
                    0x1 => println!("state: closed"),
                    0x2 => println!("state: finished"),
                    unknown => println!("state: unknown ({})", unknown),
                };

                println!(
                    "left eth: {}",
                    left_eth.await.expect("left_eth call failed")
                );
                println!(
                    "left scm: {}",
                    left_scm.await.expect("left_scm call failed")
                );
                println!("scm: {}", scm.await.expect("scm call failed"));
                println!("weth: {}", weth.await.expect("weth call failed"));

                if state != 0 {
                    let close_time = {
                        let timestamp = contract
                            .close_time()
                            .block(current_block)
                            .call()
                            .await
                            .expect("close_time call failed")
                            .as_u64();
                        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp as i64, 0))
                    };

                    println!("close time: {}", close_time);

                    let finish_time = {
                        let timestamp = contract
                            .finish_time()
                            .block(current_block)
                            .call()
                            .await
                            .expect("close_time call failed")
                            .as_u64();
                        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp as i64, 0))
                    };

                    println!("finish time: {}", finish_time);
                }
            }

            IcoCommand::Balance { address, eth } => {
                let address = address.unwrap_or(account.address());

                let (method, currency) = if *eth {
                    (contract.balance_eth(address), "wei")
                } else {
                    (contract.balance_scm(address), "asc")
                };

                let balance = method
                    .call()
                    .await
                    .expect("balance fetch failed");

                println!("{}{}", balance, currency);
            }

            IcoCommand::Fund {
                force,
                wrap_weth,
                approve_weth,
                funds,
            } => {
                let weth_address = contract.weth().call().await.unwrap();
                let weth = crate::contracts::WETH9::at(web3, weth_address);

                if *wrap_weth {
                    let balance = weth
                        .balance_of(account.address())
                        .call()
                        .await
                        .expect("balance_of call failed");

                    if balance < funds.as_inner() {
                        println!("wrapping weth...");
                        weth.deposit()
                            .from(account.clone())
                            .value(funds.as_inner())
                            .send()
                            .await
                            .expect("deposit failed");
                    }
                }

                if *approve_weth || *wrap_weth {
                    println!("approving weth...");
                    weth.approve(contract_address, funds.as_inner())
                        .from(account.clone())
                        .send()
                        .await
                        .expect("approve failed");
                }

                if *force {
                    contract
                        .fund_any(funds.as_inner())
                        .from(account)
                        .send()
                        .await
                        .expect("fundAny call failed");
                } else {
                    contract
                        .fund(funds.as_inner())
                        .from(account)
                        .send()
                        .await
                        .expect("fund call failed");
                };

                println!("done");
            }

            IcoCommand::Claim { wait } => {
                if *wait {
                    wait_finish(web3, &contract).await;
                }

                contract
                    .claim()
                    .from(account)
                    .send()
                    .await
                    .expect("fund call failed");
            }

            IcoCommand::Wait => {
                wait_finish(web3, &contract).await;
            }
        }
    }
}

async fn wait_finish(web3: &Web3<Http>, contract: &crate::contracts::ICO) {
    let current_block = web3.eth().block_number().await.unwrap();
    let state = contract
        .state()
        .block(BlockId::Number(BlockNumber::Number(current_block)))
        .call()
        .await
        .unwrap();

    if state == 0 {
        println!("waiting for ICO to close");
        contract
            .events()
            .ico_closed()
            .from_block(BlockNumber::Number(current_block))
            .stream()
            .boxed()
            .next()
            .await;
    }

    let finish_time = {
        let timestamp = contract
            .finish_time()
            .call()
            .await
            .expect("finish_time call failed")
            .as_u64();
        let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        Utc.from_utc_datetime(&naive)
    };
    let now = Utc::now();

    if now < finish_time {
        println!("waiting for ICO to finish");
        tokio::time::sleep((finish_time - now).to_std().unwrap()).await;
    }
}
