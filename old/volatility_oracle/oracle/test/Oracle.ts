import { expect } from 'chai';
import { ethers } from 'hardhat';
import { setup } from './__setup/setup';

describe('Oracle', function () {
  describe('updateState', function () { 
    it('Should put a value', async function () {
      const { oracle } = await setup();
      const price = 1000;
      const symbol = 'ETH';
      const [putter] = await ethers.getSigners();
      await oracle.connect(putter).updateState(price);
      const result = await oracle.get(await putter.getAddress());
      expect(result).to.equal(price);
    });
  })
});