import { anyValue } from '@nomicfoundation/hardhat-chai-matchers/withArgs';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import { Oracle__factory } from '../typechain-types';
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
  });
});
