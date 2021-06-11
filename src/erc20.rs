use ethcontract::prelude::*;

use crate::cli::Eth;

#[derive(structopt::StructOpt)]
#[structopt(about = "Manage SCM tokens")]
pub enum ScmCommand {
    Balance(Balance),
    Transfer(Transfer),
    Allowance(Allowance),
    Approve(Approve),
}

impl ScmCommand {
    pub async fn invoke(&self, account: Account, web3: &Web3<Http>) {
        let address = crate::contracts::get_scm_address(web3).await;
        let contract = crate::contracts::IERC20::at(web3, address);

        match self {
            Self::Balance(command) => command.invoke(account, &contract).await,
            Self::Transfer(command) => command.invoke(account, &contract).await,
            Self::Allowance(command) => command.invoke(account, &contract).await,
            Self::Approve(command) => command.invoke(account, &contract).await,
        }
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Manage wrapped ethereum tokens")]
pub enum WethCommand {
    Balance(Balance),
    Transfer(Transfer),
    Allowance(Allowance),
    Approve(Approve),
    Deposit(Deposit),
    Withdraw(Withdraw),
}

impl WethCommand {
    pub async fn invoke(&self, account: Account, web3: &Web3<Http>) {
        let address = crate::contracts::get_weth_address(web3).await;
        let contract = crate::contracts::IERC20::at(web3, address);

        match self {
            Self::Balance(command) => command.invoke(account, &contract).await,
            Self::Transfer(command) => command.invoke(account, &contract).await,
            Self::Allowance(command) => command.invoke(account, &contract).await,
            Self::Approve(command) => command.invoke(account, &contract).await,
            Self::Deposit(command) => {
                command
                    .invoke(account, &crate::contracts::WETH9::at(web3, address))
                    .await
            }
            Self::Withdraw(command) => {
                command
                    .invoke(account, &crate::contracts::WETH9::at(web3, address))
                    .await
            }
        }
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Get balance of the given wallet")]
pub struct Balance {
    #[structopt(help = "Account we're fetching balance for (uses your account by default)")]
    pub address: Option<Address>,
}

impl Balance {
    pub async fn invoke(&self, account: Account, contract: &crate::contracts::IERC20) {
        let address = self.address.unwrap_or_else(|| account.address());

        let balance = contract
            .balance_of(address)
            .from(account.address())
            .call()
            .await
            .expect("balance fetch failed");

        println!("{}", balance);
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Transfer funds between accounts")]
pub struct Transfer {
    #[structopt(help = "Where are we transferring funds to")]
    pub recipient: Address,
    #[structopt(help = "Amount of funds we are transferring")]
    pub funds: Eth,
    #[structopt(
        long,
        help = "Where are we transferring funds from (uses your account by default)"
    )]
    pub owner: Option<Address>,
}

impl Transfer {
    pub async fn invoke(&self, account: Account, contract: &crate::contracts::IERC20) {
        let owner = self.owner.unwrap_or_else(|| account.address());

        contract
            .transfer_from(owner, self.recipient, self.funds.as_inner())
            .from(account)
            .call()
            .await
            .expect("transfer failed");
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Check allowance for the given owner-spender pair")]
pub struct Allowance {
    #[structopt(help = "Who owns tokens")]
    pub owner: Address,
    #[structopt(help = "Who will be allowed to spend tokens")]
    pub spender: Address,
}

impl Allowance {
    pub async fn invoke(&self, account: Account, contract: &crate::contracts::IERC20) {
        let allowance = contract
            .allowance(self.owner, self.spender)
            .from(account.address())
            .call()
            .await
            .expect("allowance fetch failed");

        println!("{}", allowance);
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Allow some other user to withdraw funds from your account")]
pub struct Approve {
    #[structopt(help = "Who is allowed to withdraw funds")]
    pub spender: Address,
    #[structopt(
        help = "Amount of funds they are allowed to withdraw (overrides previous allowance)"
    )]
    pub value: Eth,
}

impl Approve {
    pub async fn invoke(&self, account: Account, contract: &crate::contracts::IERC20) {
        contract
            .approve(self.spender, self.value.as_inner())
            .from(account)
            .call()
            .await
            .expect("approve failed");
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Wrap ether")]
pub struct Deposit {
    #[structopt(help = "Amount of ether to wrap")]
    pub amount: Eth,
}

impl Deposit {
    pub async fn invoke(&self, account: Account, contract: &crate::contracts::WETH9) {
        contract
            .deposit()
            .from(account)
            .value(self.amount.as_inner())
            .call()
            .await
            .expect("deposit failed");
    }
}

#[derive(structopt::StructOpt)]
#[structopt(about = "Unwrap ether")]
pub struct Withdraw {
    #[structopt(help = "Amount of ether to unwrap")]
    pub amount: Eth,
}

impl Withdraw {
    pub async fn invoke(&self, account: Account, contract: &crate::contracts::WETH9) {
        contract
            .withdraw(self.amount.as_inner())
            .from(account)
            .call()
            .await
            .expect("withdraw failed");
    }
}
