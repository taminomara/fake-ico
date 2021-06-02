module.exports = async ({getNamedAccounts, deployments, getChainId, getUnnamedAccounts}) => {
    const {deployer} = await getNamedAccounts();

    // const weth = await deployments.get("WETH9");

    await deployments.deploy('SCM', {
        from: deployer,
        gasLimit: 4000000,
        args: [1000],
    });
};
