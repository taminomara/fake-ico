import "@nomiclabs/hardhat-waffle";
import "hardhat-deploy";

import {HttpNetworkUserConfig} from "hardhat/types";

const {MNEMONIC, PK, INFURA_KEY} = process.env;

const DEFAULT_MNEMONIC =
    "candy maple cake sugar pudding cream honey rich smooth crumble sweet treat";

const sharedNetworkConfig: HttpNetworkUserConfig = {};
if (PK) {
    sharedNetworkConfig.accounts = [PK];
} else {
    sharedNetworkConfig.accounts = {
        mnemonic: MNEMONIC || DEFAULT_MNEMONIC,
    };
}

module.exports = {
    solidity: "0.5.0",

    networks: {
        localhost: {
            ...sharedNetworkConfig,
            live: false,
            saveDeployments: true,
        },
        mainnet: {
            ...sharedNetworkConfig,
            url: `https://mainnet.infura.io/v3/${INFURA_KEY}`,
        },
        rinkeby: {
            ...sharedNetworkConfig,
            url: `https://rinkeby.infura.io/v3/${INFURA_KEY}`,
        },
        goerli: {
            ...sharedNetworkConfig,
            url: `https://goerli.infura.io/v3/${INFURA_KEY}`,
        },
        ropsten: {
            ...sharedNetworkConfig,
            url: `https://ropsten.infura.io/v3/${INFURA_KEY}`,
        },
        kovan: {
            ...sharedNetworkConfig,
            url: `https://kovan.infura.io/v3/${INFURA_KEY}`,
        },
    },

    namedAccounts: {
        deployer: 0,
    },
};
