import { ethers } from 'hardhat';

async function main() {
  const signer = (await ethers.getSigners())[0];
  const tx = await signer.sendTransaction({
    to: '0x565911e8ff2930ed961bc3313eb7431cc434195e',
    value: ethers.utils.parseEther('10'),
  });
  console.log(tx);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
