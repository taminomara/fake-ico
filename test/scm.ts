import {expect} from "chai";
import {ethers} from "hardhat";
import {describe} from "mocha";

describe("SCM contract", async () => {
    const totalSupply = 100;

    let Scm;
    let scm;
    let owner;
    let addr1;
    let addr2;
    let addrs;

    beforeEach(async () => {
        Scm = await ethers.getContractFactory("SCM");
        [owner, addr1, addr2, ...addrs] = await ethers.getSigners();
        scm = await Scm.deploy(totalSupply);
    });

    it("provides basic information", async () => {
        expect(await scm.name()).to.equal("Scam");
        expect(await scm.symbol()).to.equal("SCM");
        expect(await scm.decimals()).to.equal(18);
    });

    it("provides the total supply of tokens", async () => {
        expect(await scm.totalSupply()).to.equal(totalSupply);
    });

    it("assigns the total supply of tokens to the owner", async () => {
        expect(await scm.balanceOf(owner.address)).to.equal(totalSupply);
        expect(await scm.balanceOf(addr1.address)).to.equal(0);
        expect(await scm.balanceOf(addr2.address)).to.equal(0);
    });

    describe("transfer from own address", async () => {
        describe("when recipient address is zero", async () => {
            it("reverts", async () => {
                await expect(scm.transfer(ethers.constants.AddressZero, 10))
                    .to.be.revertedWith("sending to zero address");
            });
        });

        describe("when recipient address is same as owner address", async () => {
            it("succeeds when balance is sufficient", async () => {
                expect(await scm.balanceOf(owner.address))
                    .to.equal(totalSupply);
                await expect(scm.transfer(owner.address, 10))
                    .to.not.be.reverted;
                expect(await scm.balanceOf(owner.address))
                    .to.equal(totalSupply);
            });

            it("emits an event on success", async () => {
                await expect(scm.transfer(owner.address, 10))
                    .to.emit(scm, "Transfer")
                    .withArgs(owner.address, owner.address, 10);
            });

            it("reverts when balance is not sufficient", async () => {
                await expect(scm.transfer(owner.address, 1000))
                    .to.be.revertedWith("not sufficient funds");
            });
        });

        describe("when recipient address is different from owner address", async () => {
            it("succeeds and transfers funds when balance is sufficient", async () => {
                expect(await scm.balanceOf(owner.address))
                    .to.equal(totalSupply);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(0);
                await expect(scm.transfer(addr1.address, 10))
                    .to.not.be.reverted;
                expect(await scm.balanceOf(owner.address))
                    .to.equal(totalSupply - 10);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(10);
            });

            it("emits an event on success", async () => {
                await expect(scm.transfer(addr1.address, 10))
                    .to.emit(scm, "Transfer")
                    .withArgs(owner.address, addr1.address, 10);
            });

            it("reverts when balance is not sufficient", async () => {
                await expect(scm.transfer(addr1.address, 1000))
                    .to.be.revertedWith("not sufficient funds");
                expect(await scm.balanceOf(owner.address))
                    .to.equal(totalSupply);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(0);
            });

            it("reverts when balance is not sufficient after sufficient calls", async () => {
                await expect(scm.transfer(addr1.address, 50))
                    .to.not.be.reverted;
                await expect(scm.transfer(addr1.address, 100))
                    .to.be.revertedWith("not sufficient funds");
                expect(await scm.balanceOf(owner.address))
                    .to.equal(totalSupply - 50);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(50);
                await expect(scm.transfer(addr1.address, 50))
                    .to.not.be.reverted;
                expect(await scm.balanceOf(owner.address))
                    .to.equal(totalSupply - 100);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(100);
            });
        });
    });

    describe("transfer from other's address", async () => {
        describe("when recipient address is zero", async () => {
            it("reverts", async () => {

            });
        });

        describe("when sender address is zero", async () => {
            it("reverts", async () => {

            });
        });

        describe("when recipient address is same as owner address", async () => {
            describe("when allowance is sufficient", async () => {
                it("succeeds when balance is sufficient", async () => {

                });

                it("emits an event on success", async () => {

                });

                it("reverts when balance is not sufficient", async () => {

                });
            });

            describe("when allowance is not sufficient", async () => {
                it("reverts when balance is sufficient", async () => {

                });

                it("reverts when balance is not sufficient", async () => {

                });
            });
        });

        describe("when recipient address is different from owner address", async () => {
            describe("when allowance is sufficient", async () => {
                it("succeeds when balance is sufficient", async () => {

                });

                it("emits an event on success", async () => {

                });

                it("reverts when balance is not sufficient", async () => {

                });
            });

            describe("when allowance is not sufficient", async () => {
                it("reverts when balance is sufficient", async () => {

                });

                it("reverts when balance is not sufficient", async () => {

                });
            });

            describe("when allowance became insufficient after some operations", async () => {
                it("reverts", async () => {

                });
            });

            describe("when allowance got increased after it became insufficient", async () => {
                it("succeeds", async () => {

                });
            });

            describe("when allowance was decreased and became insufficient", async () => {
                it("reverts", async () => {

                });
            });

            describe("when allowance was decreased but still remains sufficient", async () => {
                it("succeeds", async () => {

                });
            });
        });
    });

    describe("allowance", async () => {
        describe("when spender address is zero", async () => {
            it("reverts", async () => {

            });
        });

        describe("when spender address is same as owner address", async () => {
            it("does not modify allowance", async () => {

            });

            it("emits an event", async () => {

            });
        });

        describe("when spender address is different from owner address", async () => {
            it("modifies allowance", async () => {

            });

            it("emits an event", async () => {

            });

            describe("when owner has less funds than spender is allowed to withdraw", async () => {
                it("still modifies allowance", async () => {

                });

                it("emits an event", async () => {

                });
            });

            describe("when allowance is set twice", async () => {
                it("overwrites previous allowance", async () => {

                });

                it("emits an event", async () => {

                });
            });
        });
    });
});
