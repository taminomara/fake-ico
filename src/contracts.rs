use ethcontract::prelude::*;

ethcontract::contract!(pub "deployments/ICO.json");
ethcontract::contract!(pub "deployments/SCM.json");
ethcontract::contract!(pub "deployments/WETH9.json");
ethcontract::contract!(pub "deployments/IERC20.json", contract=IERC20);

pub async fn get_weth_address(web3: &Web3<Http>) -> Address {
    if let Ok(address) = std::env::var("WETH_ADDRESS") {
        return address.parse().expect("invalid WETH_ADDRESS");
    }

    let net_id = web3
        .net()
        .version()
        .await
        .expect("unable to fetch network id");

    WETH9::artifact()
        .networks
        .get(&net_id)
        .expect(concat!(
            "there is no known instance of WETH on this network; ",
            "you should specify WETH address manually with WETH_ADDRESS environment variable"
        ))
        .address
}

pub async fn get_scm_address(web3: &Web3<Http>) -> Address {
    if let Ok(address) = std::env::var("SCM_ADDRESS") {
        return address.parse().expect("invalid SCM_ADDRESS");
    }

    let net_id = web3
        .net()
        .version()
        .await
        .expect("unable to fetch network id");

    SCM::artifact()
        .networks
        .get(&net_id)
        .expect(concat!(
            "there is no known instance of SCM on this network; ",
            "you should specify SCM address manually with SCM_ADDRESS environment variable"
        ))
        .address
}

pub async fn get_ico_address(web3: &Web3<Http>) -> Address {
    if let Ok(address) = std::env::var("ICO_ADDRESS") {
        return address.parse().expect("invalid ICO_ADDRESS");
    }

    let net_id = web3
        .net()
        .version()
        .await
        .expect("unable to fetch network id");

    ICO::artifact()
        .networks
        .get(&net_id)
        .expect(concat!(
            "there is no known instance of ICO on this network; ",
            "you should specify ICO address manually with ICO_ADDRESS environment variable"
        ))
        .address
}
