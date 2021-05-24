import {expect} from "chai";
import {ethers} from "hardhat";
import {describe} from "mocha";

describe("ICO contract", async () => {
    let Eth;
    let eth;
    let Ico;
    let ico;
    let scm;
    let owner;
    let addr1;
    let addr2;
    let addrs;

    beforeEach(async () => {
        Eth = await ethers.getContractFactory("WETH9");
        eth = await Eth.deploy();

        Ico = await ethers.getContractFactory("ICO");
        ico = await Ico.deploy(eth.address);

        scm = ico.scm();

        [owner, addr1, addr2, ...addrs] = await ethers.getSigners();

        await eth.connect(addr1).deposit({value: 10});
        await eth.connect(addr2).deposit({value: 10});
    });
});
