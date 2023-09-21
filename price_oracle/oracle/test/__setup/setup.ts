import { ethers } from 'hardhat';

export const setup = async () => {
  const oracleFactory = await ethers.getContractFactory('Oracle');
  const oracle = await oracleFactory.deploy();
  await oracle.deployed();

  return {
    oracle,
  };
};
