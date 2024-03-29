import {expect} from "chai";
import {ethers} from "hardhat";
import {describe} from "mocha";

describe("ICO contract", async () => {
    const ether = ethers.utils.parseEther;

    let Eth;
    let eth;
    let Ico;
    let ico;
    let Scm;
    let scm;
    let owner;
    let ethReceiver;
    let addr1;
    let addr2;
    let addrs;

    beforeEach(async () => {
        [owner, ethReceiver, addr1, addr2, ...addrs] = await ethers.getSigners();

        Eth = await ethers.getContractFactory("WETH9");
        eth = await Eth.deploy();

        Ico = await ethers.getContractFactory("ICO");
        ico = await Ico.deploy(eth.address, ethReceiver.address);

        Scm = await ethers.getContractFactory("SCM");
        scm = Scm.attach(await ico.scm());

        await eth.connect(owner).deposit({value: ether("100")});
        await eth.connect(owner).approve(ico.address, ether("100"));
        await eth.connect(addr1).deposit({value: ether("100")});
        await eth.connect(addr1).approve(ico.address, ether("100"));
        await eth.connect(addr2).deposit({value: ether("100")});
        await eth.connect(addr2).approve(ico.address, ether("100"));
    });

    it("provides basic information", async () => {
        expect(await ico.target()).to.equal(ether("10"));
        expect(await ico.rate()).to.equal(10);
        expect(await ico.leftEth()).to.equal(ether("10"));
        expect(await ico.leftScm()).to.equal(ether("10").mul(10));
        expect(await ico.holdDuration()).to.equal(2 * 60);
    });

    describe("state method", async () => {
        it("says state is ongoing when we don't have enough funds", async () => {
            expect(await ico.state())
                .to.equal(0);
        });

        it("says state is closed when we have enough funds", async () => {
            await expect(ico.fund(ether("10")))
                .to.not.be.reverted;

            expect(await ico.state())
                .to.equal(1);
        });

        it("says state is finished when we have enough funds and enough time has passed", async () => {
            await expect(ico.fund(ether("10")))
                .to.not.be.reverted;

            await ethers.provider.send("evm_increaseTime", [2 * 60]);
            await ethers.provider.send("evm_mine", []);

            expect(await ico.state())
                .to.equal(2);
        });
    });

    describe("fund method", async () => {
        it("updates user balance", async () => {
            await expect(ico.connect(addr1).fund(ether("5")))
                .to.not.be.reverted;

            expect(await ico.balanceEth(addr1.address))
                .to.equal(ether("5"));
            expect(await ico.balanceScm(addr1.address))
                .to.equal(ether("5").mul(10));
        });

        it("sends WETH to contract's account", async () => {
            await expect(ico.connect(addr1).fund(ether("5")))
                .to.not.be.reverted;

            expect(await eth.balanceOf(ethReceiver.address))
                .to.equal(ether("5"));
            expect(await eth.balanceOf(addr1.address))
                .to.equal(ether("95"));
        });

        it("updates number of SCM tokens left", async () => {
            await expect(ico.connect(addr1).fund(ether("5")))
                .to.not.be.reverted;

            expect(await ico.leftEth())
                .to.equal(ether("5"));
            expect(await ico.leftScm())
                .to.equal(ether("5").mul(10));
        });

        it("updates user balance when called multiple times", async () => {
            await expect(ico.connect(addr1).fund(ether("5")))
                .to.not.be.reverted;
            await expect(ico.connect(addr1).fund(ether("3")))
                .to.not.be.reverted;

            expect(await ico.balanceEth(addr1.address))
                .to.equal(ether("8"));
            expect(await ico.balanceScm(addr1.address))
                .to.equal(ether("8").mul(10));
        });

        it("sends WETH to contract's account when called multiple times", async () => {
            await expect(ico.connect(addr1).fund(ether("5")))
                .to.not.be.reverted;
            await expect(ico.connect(addr1).fund(ether("3")))
                .to.not.be.reverted;

            expect(await eth.balanceOf(ethReceiver.address))
                .to.equal(ether("8"));
            expect(await eth.balanceOf(addr1.address))
                .to.equal(ether("92"));
        });

        it("updates number of SCM tokens left when called multiple times", async () => {
            await expect(ico.connect(addr1).fund(ether("5")))
                .to.not.be.reverted;
            await expect(ico.connect(addr1).fund(ether("3")))
                .to.not.be.reverted;

            expect(await ico.leftEth())
                .to.equal(ether("2"));
            expect(await ico.leftScm())
                .to.equal(ether("2").mul(10));
        });

        it("emits an event", async () => {
            await expect(ico.connect(addr1).fund(ether("5")))
                .to.emit(ico, "Fund")
                .withArgs(addr1.address, ether("5"), ether("5").mul(10));
        });

        it("closes ICO when no tokens left to purchase", async () => {
            expect(await ico.state())
                .to.equal(0);

            await expect(ico.connect(addr1).fund(ether("5")))
                .to.not.be.reverted;
            await expect(ico.connect(addr2).fund(ether("5")))
                .to.not.be.reverted;

            expect(await ico.state())
                .to.equal(1);
        });

        it("sets proper closing time", async () => {
            const time = new Date().getTime() + 2 * 60;

            await ethers.provider.send("evm_setNextBlockTimestamp", [time]);

            await expect(ico.connect(addr1).fund(ether("10")))
                .to.not.be.reverted;

            expect(await ico.closeTime())
                .to.be.equal(time);
            expect(await ico.finishTime())
                .to.be.equal(time + 2 * 60);
        });

        it("emits closing event", async () => {
            const time = new Date().getTime() + 2 * 60;

            await ethers.provider.send("evm_setNextBlockTimestamp", [time]);

            await expect(ico.connect(addr1).fund(ether("10")))
                .to.emit(ico, "IcoClosed")
                .withArgs(time, time + 2 * 60);
        });

        it("reverts when buyer doesn't have enough WETH", async () => {
            let addr = addrs[0];

            await eth.connect(addr).deposit({value: ether("5")});
            await eth.connect(addr).approve(ico.address, ether("5"));

            await expect(ico.connect(addr).fund(ether("10")))
                .to.be.revertedWith("not enough WETH");
        });

        it("reverts when this contract is not allowed to spend buyer's' WETH", async () => {
            let addr = addrs[0];

            await eth.connect(addr).deposit({value: ether("10")});
            await eth.connect(addr).approve(ico.address, ether("5"));

            await expect(ico.connect(addr).fund(ether("10")))
                .to.be.revertedWith("not allowed to spend WETH");
        });

        it("reverts when purchasing more tokens than available", async () => {
            await expect(ico.connect(addr1).fund(ether("100")))
                .to.be.revertedWith("not enough tokens left");
        });

        it("reverts when ICO is closed", async () => {
            await ico.fund(ether("10"));

            await expect(ico.connect(addr1).fund(ether("10")))
                .to.be.revertedWith("ICO is closed");
        });

        it("reverts when ICO is finished", async () => {
            await ico.fund(ether("10"));

            await ethers.provider.send("evm_increaseTime", [2 * 60]);
            await ethers.provider.send("evm_mine", []);

            await expect(ico.connect(addr1).fund(ether("10")))
                .to.be.revertedWith("ICO is closed");
        });
    });

    describe("fundAny method", async () => {
        it("spends all user's ether if possible", async () => {
            await expect(ico.connect(addr1).fundAny(ether("5")))
                .to.not.be.reverted;

            expect(await eth.balanceOf(ethReceiver.address))
                .to.equal(ether("5"));
            expect(await eth.balanceOf(addr1.address))
                .to.equal(ether("95"));
            expect(await ico.balanceEth(addr1.address))
                .to.equal(ether("5"));
        });

        it("gives user all available SCM tokens if they have enough ETH", async () => {
            await expect(ico.connect(addr1).fundAny(ether("50")))
                .to.not.be.reverted;

            expect(await eth.balanceOf(ethReceiver.address))
                .to.equal(ether("10"));
            expect(await eth.balanceOf(addr1.address))
                .to.equal(ether("90"));
            expect(await ico.balanceEth(addr1.address))
                .to.equal(ether("10"));
        });

        it("gives user all available SCM tokens if they have enough ETH if ICO is partially funded", async () => {
            await expect(ico.connect(addr2).fund(ether("3")))
                .to.not.be.reverted;
            await expect(ico.connect(addr1).fundAny(ether("50")))
                .to.not.be.reverted;

            expect(await eth.balanceOf(ethReceiver.address))
                .to.equal(ether("10"));
            expect(await eth.balanceOf(addr1.address))
                .to.equal(ether("93"));
            expect(await ico.balanceEth(addr1.address))
                .to.equal(ether("7"));
        });

        it("emits an appropriate event when all user's ether is used", async () => {
            await expect(ico.connect(addr1).fundAny(ether("5")))
                .to.emit(ico, "Fund")
                .withArgs(addr1.address, ether("5"), ether("5").mul(10));
        });

        it("emits an appropriate event when only part of user's ether is used", async () => {
            await expect(ico.connect(addr1).fundAny(ether("50")))
                .to.emit(ico, "Fund")
                .withArgs(addr1.address, ether("10"), ether("10").mul(10));
        });

        it("reverts when ICO is closed", async () => {
            await expect(ico.connect(addr2).fund(ether("10")))
                .to.not.be.reverted;

            await expect(ico.connect(addr1).fundAny(ether("50")))
                .to.be.revertedWith("ICO is closed");
        });

        it("reverts when ICO is finished", async () => {
            await expect(ico.connect(addr2).fund(ether("10")))
                .to.not.be.reverted;

            await ethers.provider.send("evm_increaseTime", [2 * 60]);
            await ethers.provider.send("evm_mine", []);

            await expect(ico.connect(addr1).fundAny(ether("50")))
                .to.be.revertedWith("ICO is closed");
        });
    });

    describe("claim", async () => {
        describe("when ico is finished", async () => {
            beforeEach(async () => {
                await ico.connect(addr1).fund(ether("7"));
                await ico.connect(addr2).fund(ether("3"));

                await ethers.provider.send("evm_increaseTime", [2 * 60]);
                await ethers.provider.send("evm_mine", []);
            });

            it("transfers user's SCM to their account", async () => {
                expect(await scm.balanceOf(ico.address))
                    .to.be.equal(ether("10").mul(10));
                expect(await scm.balanceOf(addr1.address))
                    .to.be.equal(0);

                await expect(ico.connect(addr1).claim())
                    .to.not.be.reverted;

                expect(await scm.balanceOf(ico.address))
                    .to.be.equal(ether("3").mul(10));
                expect(await scm.balanceOf(addr1.address))
                    .to.be.equal(ether("7").mul(10));
            });

            it("resets user balance", async () => {
                expect(await ico.balanceEth(addr1.address))
                    .to.be.equal(ether("7"));
                expect(await ico.balanceEth(addr2.address))
                    .to.be.equal(ether("3"));

                await expect(ico.connect(addr1).claim())
                    .to.not.be.reverted;

                expect(await ico.balanceEth(addr1.address))
                    .to.be.equal(ether("0"));
                expect(await ico.balanceEth(addr2.address))
                    .to.be.equal(ether("3"));

                await expect(ico.connect(addr2).claim())
                    .to.not.be.reverted;

                expect(await ico.balanceEth(addr1.address))
                    .to.be.equal(ether("0"));
                expect(await ico.balanceEth(addr2.address))
                    .to.be.equal(ether("0"));
            });

            it("reverts when user balance is zero", async () => {
                await expect(ico.connect(owner).claim())
                    .to.be.revertedWith("no SCM tokens to claim");
            });

            it("reverts if trying to claim tokens twice", async () => {
                await expect(ico.connect(addr1).claim())
                    .to.not.be.reverted;
                await expect(ico.connect(addr1).claim())
                    .to.be.revertedWith("no SCM tokens to claim");

                await expect(ico.connect(addr2).claim())
                    .to.not.be.reverted;
                await expect(ico.connect(addr2).claim())
                    .to.be.revertedWith("no SCM tokens to claim");
            });
        });

        it("reverts if ICO is ongoing", async () => {
            await expect(ico.connect(addr1).claim())
                .to.be.revertedWith("ICO is not finished yet");

            await ico.connect(addr1).fund(ether("7"));

            await expect(ico.connect(addr1).claim())
                .to.be.revertedWith("ICO is not finished yet");
        });

        it("reverts if ICO is closed but not finished", async () => {
            await ico.connect(addr1).fund(ether("10"));

            await expect(ico.connect(addr1).claim())
                .to.be.revertedWith("ICO is not finished yet");

            await ethers.provider.send("evm_increaseTime", [2 * 60]);
            await ethers.provider.send("evm_mine", []);

            await expect(ico.connect(addr1).claim())
                .to.not.be.reverted;
        });
    });
});
