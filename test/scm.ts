import {expect} from "chai";
import {ethers} from "hardhat";

describe("SCM contract", function() {
    const totalSupply = 100;

    let Scm;
    let scm;
    let owner;
    let addr1;
    let addr2;
    let addrs;

    beforeEach(async function () {
        Scm = await ethers.getContractFactory("SCM");
        [owner, addr1, addr2, ...addrs] = await ethers.getSigners();
        scm = await Scm.deploy(totalSupply);
    });

    describe("deploy", function () {
        it("should provide basic information", async function() {
            expect(await scm.name()).to.equal("Scam");
            expect(await scm.symbol()).to.equal("SCM");
            expect(await scm.decimals()).to.equal(18);
        });

        it("should provide the total supply of tokens", async function() {
            expect(await scm.totalSupply()).to.equal(totalSupply);
        });

        it("should assign the total supply of tokens to the owner", async function() {
            expect(await scm.balanceOf(owner.address)).to.equal(totalSupply);
            expect(await scm.balanceOf(addr1.address)).to.equal(0);
            expect(await scm.balanceOf(addr2.address)).to.equal(0);
        });
    });
});
