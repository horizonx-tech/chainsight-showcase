import { ethers } from 'hardhat';

async function main() {
  const oracle = await ethers.deployContract('Oracle');

  console.log('Oracle deployed to:', oracle.address);

  // write the address to a file
  const fs = require('fs');  
  fs.writeFileSync('addresses.json', JSON.stringify({
    oracle: oracle.address
    }, 
  undefined, 2));

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});