import {expect} from "chai";
import {ethers} from "hardhat";
import {describe} from "mocha";

describe("SCM contract", async () => {
    let Scm;
    let scm;
    let owner;
    let addr1;
    let addr2;
    let addrs;

    beforeEach(async () => {
        Scm = await ethers.getContractFactory("SCM");
        [owner, addr1, addr2, ...addrs] = await ethers.getSigners();
        scm = await Scm.deploy(100);
    });

    it("provides basic information", async () => {
        expect(await scm.name()).to.equal("Scam");
        expect(await scm.symbol()).to.equal("SCM");
        expect(await scm.decimals()).to.equal(18);
    });

    it("provides the total supply of tokens", async () => {
        expect(await scm.totalSupply()).to.equal(100);
    });

    it("assigns the total supply of tokens to the owner", async () => {
        expect(await scm.balanceOf(owner.address)).to.equal(100);
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
                    .to.equal(100);
                await expect(scm.transfer(owner.address, 10))
                    .to.not.be.reverted;
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100);
            });

            it("emits an event on success", async () => {
                await expect(scm.transfer(owner.address, 10))
                    .to.emit(scm, "Transfer")
                    .withArgs(owner.address, owner.address, 10);
            });

            it("reverts when balance is insufficient", async () => {
                await expect(scm.transfer(owner.address, 10000))
                    .to.be.revertedWith("insufficient funds");
            });
        });

        describe("when recipient address is different from owner address", async () => {
            it("succeeds and transfers funds when balance is sufficient", async () => {
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(0);
                await expect(scm.transfer(addr1.address, 10))
                    .to.not.be.reverted;
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100 - 10);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(10);
            });

            it("emits an event on success", async () => {
                await expect(scm.transfer(addr1.address, 10))
                    .to.emit(scm, "Transfer")
                    .withArgs(owner.address, addr1.address, 10);
            });

            it("reverts when balance is insufficient", async () => {
                await expect(scm.transfer(addr1.address, 10000))
                    .to.be.revertedWith("insufficient funds");
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(0);
            });

            it("reverts when balance is insufficient after sufficient calls", async () => {
                await expect(scm.transfer(addr1.address, 50))
                    .to.not.be.reverted;
                await expect(scm.transfer(addr1.address, 100))
                    .to.be.revertedWith("insufficient funds");
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100 - 50);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(50);
                await expect(scm.transfer(addr1.address, 50))
                    .to.not.be.reverted;
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100 - 100);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(100);
            });

            it("does not modify allowance", async () => {
                expect(await scm.allowance(owner.address, owner.address))
                    .to.equal(0);
                await expect(scm.transfer(addr1.address, 10))
                    .to.not.be.reverted;
                expect(await scm.allowance(owner.address, owner.address))
                    .to.equal(0);
            });
        });
    });

    describe("transfer from other's address", async () => {
        let scm1;

        beforeEach(async () => {
            scm1 = scm.connect(addr1);
            await scm.approve(addr1.address, 50);
        });

        describe("when recipient address is zero", async () => {
            it("reverts", async () => {
                await expect(scm.transferFrom(addr1.address, ethers.constants.AddressZero, 10))
                    .to.be.revertedWith("sending to zero address");
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(0);
            });
        });

        describe("when sender address is zero", async () => {
            it("reverts", async () => {
                await expect(scm.transferFrom(ethers.constants.AddressZero, addr1.address, 10))
                    .to.be.revertedWith("sending from zero address");
                expect(await scm.balanceOf(owner.address))
                    .to.equal(100);
                expect(await scm.balanceOf(addr1.address))
                    .to.equal(0);
            });
        });

        describe("when recipient address is same as owner address", async () => {
            describe("when allowance is sufficient", async () => {
                it("succeeds when balance is sufficient", async () => {
                    await expect(scm1.transferFrom(owner.address, addr1.address, 5))
                        .to.not.be.reverted;
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(95);
                    expect(await scm.balanceOf(addr1.address))
                        .to.equal(5);
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(45);
                });

                it("emits an event on success", async () => {
                    await expect(scm1.transferFrom(owner.address, addr1.address, 5))
                        .to.emit(scm, "Transfer")
                        .withArgs(owner.address, addr1.address, 5);
                });

                it("decreases allowance on success", async () => {
                    await expect(scm1.transferFrom(owner.address, addr1.address, 5))
                        .to.not.be.reverted;
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(45);
                });

                it("reverts when balance is insufficient", async () => {
                    await scm.transfer(addr2.address, 75);

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(50);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(25);
                    await expect(scm1.transferFrom(owner.address, addr1.address, 50))
                        .to.be.revertedWith("insufficient funds");
                });
            });

            describe("when allowance is insufficient", async () => {
                it("reverts when balance is sufficient", async () => {
                    await expect(scm1.transferFrom(owner.address, addr1.address, 75))
                        .to.be.revertedWith("allowance exhausted");
                });

                it("reverts when balance is insufficient", async () => {
                    await scm.transfer(addr2.address, 30);

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(50);
                    await expect(scm1.transferFrom(owner.address, addr1.address, 75))
                        .to.be.reverted;
                });
            });
        });

        describe("when recipient address is different from owner address", async () => {
            describe("when allowance is sufficient", async () => {
                it("succeeds when balance is sufficient", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 5))
                        .to.not.be.reverted;
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(95);
                    expect(await scm.balanceOf(addr1.address))
                        .to.equal(0);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(5);
                });

                it("emits an event on success", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 5))
                        .to.emit(scm, "Transfer")
                        .withArgs(owner.address, addr2.address, 5);
                });

                it("decreases allowance on success", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 5))
                        .to.not.be.reverted;
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(45);
                });

                it("reverts when balance is insufficient", async () => {
                    await scm.transfer(addr2.address, 75);

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(50);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(25);
                    await expect(scm1.transferFrom(owner.address, addr2.address, 50))
                        .to.be.revertedWith("insufficient funds");
                });
            });

            describe("when allowance is insufficient", async () => {
                it("reverts when balance is sufficient", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 75))
                        .to.be.revertedWith("allowance exhausted");
                });

                it("reverts when balance is insufficient", async () => {
                    await scm.transfer(addr2.address, 30);

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(50);
                    await expect(scm1.transferFrom(owner.address, addr2.address, 75))
                        .to.be.reverted;
                });
            });

            describe("when allowance became insufficient after some operations", async () => {
                it("reverts", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 5))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(45);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(95);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(5);

                    await expect(scm1.transferFrom(owner.address, addr2.address, 50))
                        .to.be.revertedWith("allowance exhausted");

                    await expect(scm1.transferFrom(owner.address, addr2.address, 45))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(0);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(50);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(50);

                    await expect(scm1.transferFrom(owner.address, addr2.address, 50))
                        .to.be.revertedWith("allowance exhausted");
                });
            });

            describe("when allowance got increased after it became insufficient", async () => {
                it("succeeds", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 50))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(0);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(50);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(50);

                    expect(scm.approve(addr1.address, 10))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(10);

                    await expect(scm1.transferFrom(owner.address, addr2.address, 10))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(0);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(40);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(60);
                });
            });

            describe("when allowance was decreased and became insufficient", async () => {
                it("reverts", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 5))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(45);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(95);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(5);

                    expect(scm.approve(addr1.address, 10))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(10);

                    await expect(scm1.transferFrom(owner.address, addr2.address, 50))
                        .to.be.revertedWith("allowance exhausted");
                });
            });

            describe("when allowance was decreased but still remains sufficient", async () => {
                it("succeeds", async () => {
                    await expect(scm1.transferFrom(owner.address, addr2.address, 5))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(45);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(95);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(5);

                    expect(scm.approve(addr1.address, 10))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(10);

                    await expect(scm1.transferFrom(owner.address, addr2.address, 5))
                        .to.not.be.reverted;

                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(5);
                    expect(await scm.balanceOf(owner.address))
                        .to.equal(90);
                    expect(await scm.balanceOf(addr2.address))
                        .to.equal(10);
                });
            });
        });
    });

    describe("allowance", async () => {
        describe("when spender address is zero", async () => {
            it("reverts", async () => {
                await expect(scm.approve(ethers.constants.AddressZero, 10))
                    .to.be.revertedWith("setting allowance for zero address");
                expect(await scm.allowance(owner.address, ethers.constants.AddressZero))
                    .to.equal(0);

            });
        });

        describe("when spender address is same as owner address", async () => {
            it("does not modify allowance", async () => {
                expect(await scm.allowance(owner.address, owner.address))
                    .to.equal(0);
                await expect(scm.approve(owner.address, 10))
                    .to.not.be.reverted;
                expect(await scm.allowance(owner.address, owner.address))
                    .to.equal(0);
            });

            it("emits an event", async () => {
                await expect(scm.approve(owner.address, 10))
                    .to.emit(scm, "Approval")
                    .withArgs(owner.address, owner.address, 10);
            });
        });

        describe("when spender address is different from owner address", async () => {
            it("modifies allowance", async () => {
                expect(await scm.allowance(owner.address, addr1.address))
                    .to.equal(0);
                await expect(scm.approve(addr1.address, 10))
                    .to.not.be.reverted;
                expect(await scm.allowance(owner.address, addr1.address))
                    .to.equal(10);
            });

            it("emits an event", async () => {
                await expect(scm.approve(addr1.address, 10))
                    .to.emit(scm, "Approval")
                    .withArgs(owner.address, addr1.address, 10);
            });

            describe("when owner has less funds than spender is allowed to withdraw", async () => {
                it("still modifies allowance", async () => {
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(0);
                    await expect(scm.approve(addr1.address, 10000))
                        .to.not.be.reverted;
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(10000);
                });

                it("emits an event", async () => {
                    await expect(scm.approve(addr1.address, 10000))
                        .to.emit(scm, "Approval")
                        .withArgs(owner.address, addr1.address, 10000);
                });
            });

            describe("when allowance is set twice", async () => {
                it("overwrites previous allowance", async () => {
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(0);
                    await expect(scm.approve(addr1.address, 10))
                        .to.not.be.reverted;
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(10);
                    await expect(scm.approve(addr1.address, 50))
                        .to.not.be.reverted;
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(50);
                    await expect(scm.approve(addr1.address, 10))
                        .to.not.be.reverted;
                    expect(await scm.allowance(owner.address, addr1.address))
                        .to.equal(10);
                });

                it("emits an event", async () => {
                    await expect(scm.approve(addr1.address, 10))
                        .to.emit(scm, "Approval")
                        .withArgs(owner.address, addr1.address, 10);
                    await expect(scm.approve(addr1.address, 50))
                        .to.emit(scm, "Approval")
                        .withArgs(owner.address, addr1.address, 50);
                    await expect(scm.approve(addr1.address, 10))
                        .to.emit(scm, "Approval")
                        .withArgs(owner.address, addr1.address, 10);
                });
            });
        });
    });
});
