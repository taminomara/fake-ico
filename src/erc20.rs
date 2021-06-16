use ethcontract::prelude::*;

use crate::cli::{Currency, Eth, Scm};

#[derive(structopt::StructOpt)]
#[structopt(about = "Manage SCM tokens")]
pub enum ScmCommand {
    #[structopt(about = "Get balance of the given wallet")]
    Balance {
        #[structopt(help = "Account we're fetching balance for (uses your account by default)")]
        address: Option<Address>,
    },

    #[structopt(about = "Transfer funds between accounts")]
    Transfer {
        #[structopt(help = "Where are we transferring funds to")]
        recipient: Address,
        #[structopt(help = "Amount of funds we are transferring")]
        funds: Scm,
        #[structopt(
            long,
            help = "Where are we transferring funds from (uses your account by default)"
        )]
        owner: Option<Address>,
    },

    #[structopt(about = "Check allowance for the given owner-spender pair")]
    Allowance {
        #[structopt(help = "Who owns tokens")]
        owner: Address,
        #[structopt(help = "Who will be allowed to spend tokens")]
        spender: Address,
    },

    #[structopt(about = "Allow some other user to withdraw funds from your account")]
    Approve {
        #[structopt(help = "Who is allowed to withdraw funds")]
        spender: Address,
        #[structopt(
            help = "Amount of funds they are allowed to withdraw (overrides previous allowance)"
        )]
        value: Scm,
    },
}

impl ScmCommand {
    pub async fn invoke(&self, account: Account, web3: &Web3<Http>) {
        let account_address = account.address();

        let contract_address = crate::contracts::get_scm_address(web3).await;
        let contract = crate::contracts::SCM::at(web3, contract_address);

        match self {
            Self::Balance { address } => {
                Self::print_balance(address.unwrap_or(account_address), &contract).await;
            }

            Self::Transfer {
                recipient,
                funds,
                owner,
            } => {
                contract
                    .transfer_from(
                        owner.unwrap_or(account_address),
                        *recipient,
                        funds.as_inner(),
                    )
                    .from(account)
                    .send()
                    .await
                    .expect("transfer failed");

                println!("Done");
                Self::print_balance(account_address, &contract).await;
            }

            Self::Allowance { owner, spender } => {
                let allowance = contract
                    .allowance(*owner, *spender)
                    .call()
                    .await
                    .expect("allowance fetch failed");

                println!("Allowance: {}", Scm::new(allowance));
            }

            Self::Approve { spender, value } => {
                contract
                    .approve(*spender, value.as_inner())
                    .from(account)
                    .send()
                    .await
                    .expect("approve failed");

                println!("Done");
            }
        }
    }

    async fn print_balance(address: Address, contract: &crate::contracts::SCM) {
        let balance = contract
            .balance_of(address)
            .call()
            .await
            .expect("balance fetch failed");

        println!("SCM balance: {}", Scm::new(balance));
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Manage wrapped ethereum tokens")]
pub enum WethCommand {
    #[structopt(about = "Get balance of the given wallet")]
    Balance {
        #[structopt(help = "Account we're fetching balance for (uses your account by default)")]
        address: Option<Address>,
    },

    #[structopt(about = "Transfer funds between accounts")]
    Transfer {
        #[structopt(help = "Where are we transferring funds to")]
        recipient: Address,
        #[structopt(help = "Amount of funds we are transferring")]
        funds: Eth,
        #[structopt(
            long,
            help = "Where are we transferring funds from (uses your account by default)"
        )]
        owner: Option<Address>,
    },

    #[structopt(about = "Check allowance for the given owner-spender pair")]
    Allowance {
        #[structopt(help = "Who owns tokens")]
        owner: Address,
        #[structopt(help = "Who will be allowed to spend tokens")]
        spender: Address,
    },

    #[structopt(about = "Allow some other user to withdraw funds from your account")]
    Approve {
        #[structopt(help = "Who is allowed to withdraw funds")]
        spender: Address,
        #[structopt(
            help = "Amount of funds they are allowed to withdraw (overrides previous allowance)"
        )]
        value: Eth,
    },

    #[structopt(about = "Wrap ether")]
    Deposit {
        #[structopt(help = "Amount of ether to wrap")]
        amount: Eth,
    },

    #[structopt(about = "Unwrap ether")]
    Withdraw {
        #[structopt(help = "Amount of ether to unwrap")]
        amount: Eth,
    },
}

impl WethCommand {
    pub async fn invoke(&self, account: Account, web3: &Web3<Http>) {
        let account_address = account.address();

        let contract_address = crate::contracts::get_weth_address(web3).await;
        let contract = crate::contracts::WETH9::at(web3, contract_address);

        match self {
            Self::Balance { address } => {
                Self::print_balance(address.unwrap_or(account_address), &contract).await;
            }

            Self::Transfer {
                recipient,
                funds,
                owner,
            } => {
                contract
                    .transfer_from(
                        owner.unwrap_or(account_address),
                        *recipient,
                        funds.as_inner(),
                    )
                    .from(account)
                    .send()
                    .await
                    .expect("transfer failed");

                println!("Done");
                Self::print_balance(account_address, &contract).await;
            }

            Self::Allowance { owner, spender } => {
                let allowance = contract
                    .allowance(*owner, *spender)
                    .call()
                    .await
                    .expect("allowance fetch failed");

                println!("Allowance: {}", Eth::new(allowance));
            }

            Self::Approve { spender, value } => {
                contract
                    .approve(*spender, value.as_inner())
                    .from(account)
                    .send()
                    .await
                    .expect("approve failed");

                println!("Done");
            }

            Self::Deposit { amount } => {
                contract
                    .deposit()
                    .from(account)
                    .value(amount.as_inner())
                    .send()
                    .await
                    .expect("deposit failed");

                println!("Done");
                Self::print_balance(account_address, &contract).await;
            }

            Self::Withdraw { amount } => {
                contract
                    .withdraw(amount.as_inner())
                    .from(account)
                    .send()
                    .await
                    .expect("withdraw failed");

                println!("Done");
                Self::print_balance(account_address, &contract).await;
            }
        }
    }

    async fn print_balance(address: Address, contract: &crate::contracts::WETH9) {
        let balance = contract
            .balance_of(address)
            .call()
            .await
            .expect("balance fetch failed");

        println!("WETH balance: {}", Eth::new(balance));
    }
}
