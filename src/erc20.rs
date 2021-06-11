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
        let contract_address = crate::contracts::get_scm_address(web3).await;
        let contract = crate::contracts::SCM::at(web3, contract_address);

        match self {
            Self::Balance { address } => {
                let balance = contract
                    .balance_of(address.unwrap_or(account.address()))
                    .from(account.address())
                    .call()
                    .await
                    .expect("balance fetch failed");

                println!("{}asc", balance);
            }

            Self::Transfer { recipient, funds, owner } => {
                contract
                    .transfer_from(owner.unwrap_or(account.address()), *recipient, funds.as_inner())
                    .from(account)
                    .call()
                    .await
                    .expect("transfer failed");
            }

            Self::Allowance { owner, spender } => {
                let allowance = contract
                    .allowance(*owner, *spender)
                    .from(account.address())
                    .call()
                    .await
                    .expect("allowance fetch failed");

                println!("{}asc", allowance);
            }

            Self::Approve { spender, value } => {
                contract
                    .approve(*spender, value.as_inner())
                    .from(account)
                    .call()
                    .await
                    .expect("approve failed");
            }
        }
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Manage wrapped ethereum tokens")]
pub enum EthCommand {
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

impl EthCommand {
    pub async fn invoke(&self, account: Account, web3: &Web3<Http>) {
        let contract_address = crate::contracts::get_scm_address(web3).await;
        let contract = crate::contracts::WETH9::at(web3, contract_address);

        match self {
            Self::Balance { address } => {
                let balance = contract
                    .balance_of(address.unwrap_or(account.address()))
                    .from(account.address())
                    .call()
                    .await
                    .expect("balance fetch failed");

                println!("{}wei", balance);
            }

            Self::Transfer { recipient, funds, owner } => {
                contract
                    .transfer_from(owner.unwrap_or(account.address()), *recipient, funds.as_inner())
                    .from(account)
                    .call()
                    .await
                    .expect("transfer failed");
            }

            Self::Allowance { owner, spender } => {
                let allowance = contract
                    .allowance(*owner, *spender)
                    .from(account.address())
                    .call()
                    .await
                    .expect("allowance fetch failed");

                println!("{}wei", allowance);
            }

            Self::Approve { spender, value } => {
                contract
                    .approve(*spender, value.as_inner())
                    .from(account)
                    .call()
                    .await
                    .expect("approve failed");
            }

            Self::Deposit { amount } => {
                contract
                    .deposit()
                    .from(account)
                    .value(amount.as_inner())
                    .call()
                    .await
                    .expect("deposit failed");
            }

            Self::Withdraw { amount } => {
                contract
                    .withdraw(amount.as_inner())
                    .from(account)
                    .call()
                    .await
                    .expect("withdraw failed");
            }
        }
    }
}
