import { describeLitentry } from './utils';
import { step } from 'mocha-steps';
import { requestVC, setUserShieldingKey } from './indirect_calls';
import { Assertion } from './type-definitions';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';

const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2', 'A2'],
    A3: ['A3', 'A3'],
    A4: [10, 'A4'],
    A5: ['A5', 'A5'],
    A6: 'A6',
    A7: [10, 10],
    A8: [10],
    A9: 'A9',
    A10: [10, 10],
    A11: [10, 10],
};
describeLitentry('VC test', async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    step('set user shielding key', async function () {
        const who = await setUserShieldingKey(context, context.defaultSigner, aesKey, true);
        assert.equal(who, u8aToHex(context.defaultSigner.addressRaw), 'check caller error');
    });
    step('Request VC', async () => {
        const vc = await requestVC(context, context.defaultSigner, aesKey, true, context.shard, {
            A1: assertion.A1,
        });
        assert(vc !== u8aToHex());
    });
});
