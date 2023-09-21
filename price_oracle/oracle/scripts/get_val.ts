import { ethers } from 'hardhat';
import { Oracle__factory } from '../typechain-types';

async function main() {
  const signer = (await ethers.getSigners())[0];
  const provider = signer.provider!;
  const oracle = Oracle__factory.connect(
    '0x5FbDB2315678afecb367f032d93F642f64180aa3',
    provider
  );
  const price = await oracle.get('0x0bad3c621c5fd622e299f8856a807b86ff114ccd');
  console.log(price.toString());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
