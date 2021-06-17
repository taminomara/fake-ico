use ethcontract::prelude::*;

use crate::cli::{Currency, Eth, Scm};
use chrono::{Local, NaiveDateTime, TimeZone, Utc};
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
        #[structopt(long, help = "Display balance in ETH")]
        eth: bool,
    },

    #[structopt(about = "Buy SCM")]
    Fund {
        #[structopt(long, help = "Wrap and approve eth if you don't have enough of it")]
        wrap_weth: bool,
        #[structopt(long, help = "Ensure that ICO is authorized to spend WETH")]
        approve_weth: bool,
        #[structopt(help = "Number of ETH tokens to contribute to the ICO")]
        funds: Eth,
    },

    #[structopt(about = "Claim purchased SCM")]
    Claim {
        #[structopt(long, help = "If ICO is not finished, wait for it")]
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

                let state = contract.state().block(current_block).batch_call(&mut batch);
                let left_eth = contract
                    .left_eth()
                    .block(current_block)
                    .batch_call(&mut batch);
                let left_scm = contract
                    .left_scm()
                    .block(current_block)
                    .batch_call(&mut batch);
                let scm = contract.scm().block(current_block).batch_call(&mut batch);
                let weth = contract.weth().block(current_block).batch_call(&mut batch);

                batch.execute_all(100).await;

                let state = state.await.expect("state call failed");
                match state {
                    0x0 => println!("State: Ongoing"),
                    0x1 => println!("State: Closed"),
                    0x2 => println!("State: Finished"),
                    unknown => println!("State: Unknown ({})", unknown),
                };

                println!(
                    "Left ETH: {}",
                    Eth::new(left_eth.await.expect("left_eth call failed"))
                );
                println!(
                    "Left SCM: {}",
                    Scm::new(left_scm.await.expect("left_scm call failed"))
                );
                println!("ICO: {:?}", contract_address);
                println!("SCM: {:?}", scm.await.expect("scm call failed"));
                println!("WETH: {:?}", weth.await.expect("weth call failed"));

                if state != 0 {
                    let close_time = {
                        let timestamp = contract
                            .close_time()
                            .block(current_block)
                            .call()
                            .await
                            .expect("close_time call failed")
                            .as_u64();
                        Local.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp as i64, 0))
                    };

                    println!("Close time: {}", close_time);

                    let finish_time = {
                        let timestamp = contract
                            .finish_time()
                            .block(current_block)
                            .call()
                            .await
                            .expect("close_time call failed")
                            .as_u64();
                        Local.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp as i64, 0))
                    };

                    println!("Finish time: {}", finish_time);
                }
            }

            IcoCommand::Balance { address, eth } => {
                let address = address.unwrap_or(account.address());

                if *eth {
                    let balance = contract
                        .balance_eth(address)
                        .call()
                        .await
                        .expect("balance fetch failed");
                    println!("ICO balance: {}", Eth::new(balance));
                } else {
                    let balance = contract
                        .balance_scm(address)
                        .call()
                        .await
                        .expect("balance fetch failed");
                    println!("ICO balance: {}", Scm::new(balance));
                };
            }

            IcoCommand::Fund {
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
                        println!("Wrapping WETH");
                        weth.deposit()
                            .from(account.clone())
                            .value(funds.as_inner())
                            .send()
                            .await
                            .expect("deposit failed");
                    } else {
                        println!("WETH balance is sufficient, no need to wrap more");
                    }
                }

                if *approve_weth || *wrap_weth {
                    let allowance = weth
                        .allowance(account.address(), contract_address)
                        .call()
                        .await
                        .expect("balance_of call failed");

                    if allowance < funds.as_inner() {
                        println!("Approving WETH");
                        weth.approve(contract_address, U256::exp10(18) * 10)
                            .from(account.clone())
                            .send()
                            .await
                            .expect("approve failed");
                    } else {
                        println!("WETH allowance is sufficient, no need to approve more");
                    }
                }

                contract
                    .fund(funds.as_inner())
                    .from(account.clone())
                    .send()
                    .await
                    .expect("fund call failed");

                println!("Done");

                let balance = contract
                    .balance_scm(account.address())
                    .call()
                    .await
                    .expect("balance fetch failed");

                println!("ICO balance: {}", Scm::new(balance));
            }

            IcoCommand::Claim { wait } => {
                if *wait {
                    wait_finish(web3, &contract).await;
                }

                contract
                    .claim()
                    .from(account.clone())
                    .send()
                    .await
                    .expect("fund call failed");

                println!("Done");

                let scm_address = contract.scm().call().await.unwrap();
                let scm = crate::contracts::SCM::at(web3, scm_address);

                let balance = scm
                    .balance_of(account.address())
                    .call()
                    .await
                    .expect("balance fetch failed");

                println!("SCM balance: {}", Scm::new(balance));
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
        println!("Waiting for ICO to close");
        contract
            .events()
            .ico_closed()
            .from_block(BlockNumber::Number(current_block))
            .stream()
            .boxed()
            .next()
            .await;
        println!("ICO closed");
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
        println!("ICO will finish on {}", finish_time.with_timezone(&Local));
        println!("Waiting for ICO to finish");
        tokio::time::sleep((finish_time - now).to_std().unwrap()).await;
        println!("ICO finished");
    }
}
