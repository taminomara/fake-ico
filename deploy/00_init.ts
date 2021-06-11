module.exports = async ({getNamedAccounts, deployments, getChainId, getUnnamedAccounts}) => {
    const {deployer} = await getNamedAccounts();

    const weth = await deployments.get("WETH9");

    const ico = await deployments.deploy('ICO', {
        from: deployer,
        gasLimit: 4000000,
        args: [weth.address, deployer],
    });

    const scmAddress = await deployments.read('ICO', {}, 'scm');
    const scm = await deployments.getArtifact('SCM');
    deployments.save('SCM', {
        abi: scm.abi,
        address: scmAddress,
    });
};
