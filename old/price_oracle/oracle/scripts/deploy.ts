import { ethers } from 'hardhat';

async function main() {
  const oracleUSDT = await ethers.deployContract('Oracle');

  console.log('Oracle USDT deployed to:', oracleUSDT.address);

  const oracleETH = await ethers.deployContract('Oracle');

  console.log('Oracle ETH deployed to:', oracleETH.address);

  const mockChainlinkWrapper = await ethers.deployContract(
    'MockChainlinkWrapper'
  );
  console.log(
    'MockChainlinkWrapper deployed to:',
    mockChainlinkWrapper.address
  );

  const chainlinkProxy = await ethers.getContractAt(
    'ChainlinkProxy',
    mockChainlinkWrapper.address
  );
  console.log('ChainlinkProxy deployed to:', chainlinkProxy.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
