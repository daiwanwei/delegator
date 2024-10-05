import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import { Delegator } from "../target/types/delegator";
import { createMint, mintTo } from "./utils/token";
import { assert } from "chai";

describe("delegator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Delegator as Program<Delegator>;

  const MAX_SUPPLY = BigInt(10_000_000_000_000);
  const DECIMALS = 9;
  const MAX_CAP_STAKE_AMOUNT = new anchor.BN(10_000_000_000);

  let BASE_APR = new anchor.BN(1);
  let tiers = [
    {
      tier: 0,
      lockUpEpoch: 3,
      multiplier: 1,
    },
    {
      tier: 1,
      lockUpEpoch: 6,
      multiplier: 4,
    },
    {
      tier: 2,
      lockUpEpoch: 12,
      multiplier: 7,
    },
    {
      tier: 3,
      lockUpEpoch: 24,
      multiplier: 8,
    },
  ];

  const episode = new anchor.BN(1);

  const payer = anchor.getProvider().publicKey;
  const guardian = anchor.web3.Keypair.generate();
  const policy = anchor.web3.Keypair.generate();

  const tokenMint = anchor.web3.Keypair.generate();
  const rewardMint = anchor.web3.Keypair.generate();
  const tokenVault = anchor.web3.Keypair.generate();
  const rewardVault = anchor.web3.Keypair.generate();

  before(async () => {
    // Add your setup here.
    await createMint(anchor.getProvider(), tokenMint, 9, payer);

    await checkMintAccount(
      anchor.getProvider(),
      tokenMint.publicKey,
      payer,
      DECIMALS,
      TOKEN_2022_PROGRAM_ID
    );
    await createMint(anchor.getProvider(), rewardMint, 9, payer);

    await checkMintAccount(
      anchor.getProvider(),
      rewardMint.publicKey,
      payer,
      DECIMALS,
      TOKEN_2022_PROGRAM_ID
    );

    await mintTo(anchor.getProvider(), tokenMint.publicKey, payer, MAX_SUPPLY);

    await mintTo(anchor.getProvider(), rewardMint.publicKey, payer, MAX_SUPPLY);
  });

  it("Create Guardian successfully", async () => {
    // Add your test here.
    const tx = await program.methods
      .createGuardian()
      .accounts({
        payer,
        guardian: guardian.publicKey,
      })
      .signers([guardian])
      .rpc();
    console.log("Your transaction signature", tx);

    // Get the guardian account.
    const account = await program.account.guardian.fetch(guardian.publicKey);
    assert(
      payer.equals(account.adminAuthority),
      "Admin authority is not correct"
    );
  });

  it("Create Policy successfully", async () => {
    // Add your test here.

    const tx = await program.methods
      .createPolicy(BASE_APR, tiers)
      .accounts({
        payer,
        policy: policy.publicKey,
      })
      .signers([policy])
      .rpc();
    console.log("Your transaction signature", tx);

    checkPolicyAccount(program, policy.publicKey, BASE_APR, tiers);
  });

  it("Create Stake pool successfully", async () => {
    // Add your test here.
    const episode = new anchor.BN(1);
    const stakeCap = new anchor.BN(10000000000);
    const stakePool = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("stake_pool")),
        guardian.publicKey.toBuffer(),
        episode.toArrayLike(Buffer, "le", 1),
      ],
      program.programId
    )[0];

    const tx = await program.methods
      .createStakePool(episode, stakeCap)
      .accounts({
        payer,
        guardian: guardian.publicKey,
        stakePool: stakePool,
        policy: policy.publicKey,
        tokenMint: tokenMint.publicKey,
        tokenVault: tokenVault.publicKey,
        rewardTokenMint: rewardMint.publicKey,
        rewardVault: rewardVault.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        tokenProgramReward: TOKEN_2022_PROGRAM_ID,
      })
      .signers([tokenVault, rewardVault])
      .rpc();
    console.log("Your transaction signature", tx);

    // Get the guardian account.
    const account = await program.account.stakePool.fetch(stakePool);
    assert(
      guardian.publicKey.equals(account.guardian),
      "guardian is not correct"
    );

    await checkAccount(
      anchor.getProvider(),
      tokenVault.publicKey,
      tokenMint.publicKey,
      stakePool
    );

    await checkAccount(
      anchor.getProvider(),
      rewardVault.publicKey,
      rewardMint.publicKey,
      stakePool
    );

    await checkStakePoolAccount(
      program,
      stakePool,
      guardian.publicKey,
      tokenMint.publicKey,
      rewardMint.publicKey,
      tokenVault.publicKey,
      rewardVault.publicKey,
      episode,
      stakeCap
    );
  });

  it("deposit reward token successfully", async () => {
    // Add your test here.
    const stakePool = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("stake_pool")),
        guardian.publicKey.toBuffer(),
        episode.toArrayLike(Buffer, "le", 1),
      ],
      program.programId
    )[0];

    const amount = new anchor.BN(1000000000);
    const payerRewardTokenAta = getAssociatedTokenAddressSync(
      rewardMint.publicKey,
      payer,
      null,
      TOKEN_2022_PROGRAM_ID
    );

    const tx = await program.methods
      .depositReward(amount)
      .accounts({
        payer,
        stakePool: stakePool,
        payerRewardTokenAta: payerRewardTokenAta,
        rewardTokenMint: rewardMint.publicKey,
        rewardVault: rewardVault.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  });

  it("create user stake successfully", async () => {
    // Add your test here.
    const stakePool = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("stake_pool")),
        guardian.publicKey.toBuffer(),
        episode.toArrayLike(Buffer, "le", 1),
      ],
      program.programId
    )[0];

    const userStake = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("user_stake")),
        stakePool.toBuffer(),
        payer.toBuffer(),
      ],
      program.programId
    )[0];

    const tx = await program.methods
      .createUserStake(new anchor.BN(0))
      .accounts({
        payer,
        stakePool: stakePool,
        userStake: userStake,
        policy: policy.publicKey,
        user: payer,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();

    await checkUserStakeAccount(program, userStake, stakePool, payer);
  });

  it("stake successfully", async () => {
    // Add your test here.
    const stakePool = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("stake_pool")),
        guardian.publicKey.toBuffer(),
        episode.toArrayLike(Buffer, "le", 1),
      ],
      program.programId
    )[0];

    const userStake = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("user_stake")),
        stakePool.toBuffer(),
        payer.toBuffer(),
      ],
      program.programId
    )[0];

    const ata = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      payer,
      null,
      TOKEN_2022_PROGRAM_ID
    );

    const amount = new anchor.BN(1000000000);
    const tx = await program.methods
      .stake(amount)
      .accounts({
        payer,
        stakePool: stakePool,
        userStake: userStake,
        payerTokenAta: ata,
        tokenMint: tokenMint.publicKey,
        tokenVault: tokenVault.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();

    await checkUserStakeAccount(program, userStake, stakePool, payer, amount);
  });

  it("unstake successfully", async () => {
    // Add your test here.
    const stakePool = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("stake_pool")),
        guardian.publicKey.toBuffer(),
        episode.toArrayLike(Buffer, "le", 1),
      ],
      program.programId
    )[0];

    const userStake = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("user_stake")),
        stakePool.toBuffer(),
        payer.toBuffer(),
      ],
      program.programId
    )[0];

    const ata = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      payer,
      null,
      TOKEN_2022_PROGRAM_ID
    );

    const amount = new anchor.BN(1000000000);
    const tx = await program.methods
      .unstake(amount)
      .accounts({
        payer,
        stakePool: stakePool,
        userStake: userStake,
        payerTokenAta: ata,
        tokenMint: tokenMint.publicKey,
        tokenVault: tokenVault.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();

    await checkUserStakeAccount(program, userStake, stakePool, payer);
  });
});

async function checkMintAccount(
  provider: anchor.Provider,
  mint: anchor.web3.PublicKey,
  mintAuthority: anchor.web3.PublicKey,
  decimals: number,
  tokenProgramId: anchor.web3.PublicKey = TOKEN_2022_PROGRAM_ID
) {
  const mintAccount = await getMint(
    provider.connection,
    mint,
    null,
    tokenProgramId
  );
  assert(mintAccount.decimals === decimals, "Decimals is not correct");
  assert(
    mintAccount.mintAuthority.equals(mintAuthority),
    "Mint authority is not correct"
  );
}

async function checkAccount(
  provider: anchor.Provider,
  account: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
  tokenProgramId: anchor.web3.PublicKey = TOKEN_2022_PROGRAM_ID
) {
  const accountInfo = await getAccount(
    provider.connection,
    account,
    null,
    tokenProgramId
  );
  assert(accountInfo.mint.equals(mint), "Mint is not correct");
  assert(accountInfo.owner.equals(owner), "Owner is not correct");
}

async function checkStakePoolAccount(
  program: Program<Delegator>,
  stakePool: anchor.web3.PublicKey,
  guardian: anchor.web3.PublicKey,
  tokenMint: anchor.web3.PublicKey,
  rewardTokenMint: anchor.web3.PublicKey,
  tokenVault: anchor.web3.PublicKey,
  rewardVault: anchor.web3.PublicKey,
  episode: anchor.BN,
  stakeCap: anchor.BN
) {
  const account = await program.account.stakePool.fetch(stakePool);
  assert(guardian.equals(account.guardian), "Guardian is not correct");
  assert(tokenMint.equals(account.tokenMint), "Token mint is not correct");
  assert(
    rewardTokenMint.equals(account.rewardTokenMint),
    "Reward token mint is not correct"
  );
  assert(tokenVault.equals(account.tokenVault), "Token vault is not correct");
  assert(
    rewardVault.equals(account.rewardVault),
    "Reward vault is not correct"
  );
  assert(episode.eq(new anchor.BN(account.episode)), "Episode is not correct");
  assert(stakeCap.eq(account.capStakeAmount), "Stake cap is not correct");
}

async function checkUserStakeAccount(
  program: Program<Delegator>,
  userStake: anchor.web3.PublicKey,
  stakePool: anchor.web3.PublicKey,
  user: anchor.web3.PublicKey,
  stakedAmount: anchor.BN = new anchor.BN(0)
) {
  const account = await program.account.userStake.fetch(userStake);
  console.log(`user stake account ${account.owedReward}`);
  assert(stakePool.equals(account.stakePool), "Stake pool is not correct");
  assert(user.equals(account.owner), "User is not correct");
  assert(stakedAmount.eq(account.stakedAmount), "Staked amount is not correct");
}

async function checkPolicyAccount(
  program: Program<Delegator>,
  policy: anchor.web3.PublicKey,
  baseApr: anchor.BN,
  tiers: any[]
) {
  const account = await program.account.policy.fetch(policy);
  assert(baseApr.eq(account.baseAprX64), "Base APR is not correct");
  assert(tiers.length === account.tiers.length, "Tiers length is not correct");
  for (let i = 0; i < tiers.length; i++) {
    assert(tiers[i].tier === account.tiers[i].tier, "Tier is not correct");
    assert(
      tiers[i].lockUpEpoch === account.tiers[i].lockUpEpoch,
      "Lock up epoch is not correct"
    );
    assert(
      tiers[i].multiplier === account.tiers[i].multiplier,
      "Multiplier is not correct"
    );
  }
}
