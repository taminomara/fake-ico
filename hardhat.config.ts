import "@nomiclabs/hardhat-waffle";
import "hardhat-deploy";

import {HttpNetworkUserConfig} from "hardhat/types";

const {MNEMONIC, PK} = process.env;

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
    },

    namedAccounts: {
        deployer: 0,
    },
};
