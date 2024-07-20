import { test, expect } from "vitest";
import { FSWorld, d } from "xsuite";

test("Test", async () => {
  using world = await FSWorld.start();
  const userShard0 = await world.createWallet({
    address: { shard: 0 },
    balance: 10n ** 18n,
    kvs: {
      esdts: [{ id: "ABC-123456", amount: 1 }],
    }
  });
  const userShard1 = await world.createWallet({
    address: { shard: 1 },
    balance: 100n ** 18n,
  });
  const { contract: contractShard0 } = await userShard0.deployContract({
    code: "file:output/contract.wasm",
    codeMetadata: [],
    gasLimit: 20_000_000,
    codeArgs: [],
  });
  const { contract: contractShard1 } = await userShard1.deployContract({
    code: "file:output/contract.wasm",
    codeMetadata: [],
    gasLimit: 100_000_000,
    codeArgs: [],
  });

  await userShard0.callContract({
    callee: contractShard1,
    funcName: "remote",
    funcArgs: [contractShard0],
    esdts: [{ id: "ABC-123456", amount: 1 }],
    gasLimit: 50_000_000,
  });
  // working with this line:
  // await world.generateBlocks(6);

  expect(d.kvs().from(await userShard0.getAccountKvs())).toEqual({
    esdts: [{ id: "ABC-123456", amount: 1n }],
  });
});
