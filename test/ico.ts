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

    it("provides basic information", async () => {
    });

    describe("state method", async () => {
        it("says state is ongoing when we don't have enough funds", async () => {

        });

        it("says state is closed when we have enough funds", async () => {

        });

        it("says state is finished when we have enough funds and enough time has passed", async () => {

        });
    });

    describe("fund method", async () => {
        it("updates user balance", async () => {

        });

        it("sends WETH to contract's account", async () => {

        });

        it("updates number of SCM tokens left", async () => {

        });

        it("updates user balance when called multiple times", async () => {

        });

        it("sends WETH to contract's account when called multiple times", async () => {

        });

        it("updates user balance when called multiple times", async () => {

        });

        it("updates number of SCM tokens left when called multiple times", async () => {

        });

        it("emits an event", async () => {

        });

        it("closes ICO when no tokens left to purchase", async () => {

        });

        it("sets proper closing time", async () => {

        });

        it("emits closing event", async () => {

        });

        it("reverts when buyer doesn't have enough WETH", async () => {

        });

        it("reverts when this contract is not allowed to spend buyer's' WETH", async () => {

        });

        it("reverts when purchasing more tokens than available", async () => {

        });

        it("reverts when ICO is closed", async () => {

        });

        it("reverts when ICO is finished", async () => {

        });
    });

    describe("fundAny method", async () => {
        it("spends all user's ether if possible", async () => {

        });

        it("gives user all available SCM tokens if they have enough ETH", async () => {

        });

        it("emits an appropriate event when all user's ether is used", async () => {

        });

        it("emits an appropriate event when only part of user's ether is used", async () => {

        });

        it("reverts when ICO is closed", async () => {

        });

        it("reverts when ICO is finished", async () => {

        });
    });

    describe("claim", async () => {
        it("transfers user's SCM to their account", async () => {

        });

        it("resets user balance", async () => {

        });

        it("emits an event", async () => {

        });

        it("reverts when user balance is zero", async () => {

        });

        it("reverts if trying to claim tokens twice", async () => {

        });

        it("reverts if ICO is not ongoing", async () => {

        });

        it("reverts if ICO is closed but not finished", async () => {

        });
    });
});