import {expect} from "chai";
import {ethers} from "hardhat";

describe("SCM contract", function() {
    it("should assign the total supply of tokens to the owner", async function() {
        const [owner] = await ethers.getSigners();

        const Token = await ethers.getContractFactory("SCM");

        const hardhatToken = await Token.deploy(100);

        expect(await hardhatToken.totalSupply()).to.equal(100);
        expect(await hardhatToken.balanceOf(owner.address)).to.equal(100);
    });
});
