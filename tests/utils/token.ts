import * as anchor from "@coral-xyz/anchor";
import { Provider, web3 } from "@coral-xyz/anchor";

import {
  createInitializeMint2Instruction,
  createMintToInstruction,
  getMintLen,
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  getAccount,
  TokenAccountNotFoundError,
  TokenInvalidAccountOwnerError,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

export async function createMint(
  provider: Provider,
  mint: web3.Keypair,
  decimals: number,
  mintAuthority: web3.PublicKey,
  tokenProgramId: web3.PublicKey = TOKEN_2022_PROGRAM_ID
) {
  let len = getMintLen([]);
  let rent = await provider.connection.getMinimumBalanceForRentExemption(len);

  let createAccountIx = web3.SystemProgram.createAccount({
    fromPubkey: provider.publicKey,
    newAccountPubkey: mint.publicKey,
    space: len,
    lamports: rent,
    programId: TOKEN_2022_PROGRAM_ID,
  });

  let mintIx = createInitializeMint2Instruction(
    mint.publicKey,
    decimals,
    mintAuthority,
    null,
    tokenProgramId
  );

  let tx = new anchor.web3.Transaction();
  tx.add(createAccountIx);
  tx.add(mintIx);
  await provider.sendAndConfirm(tx, [mint]);
}

export async function mintTo(
  provider: Provider,
  mint: web3.PublicKey,
  receiver: web3.PublicKey,
  amount: bigint,
  tokenProgramId: web3.PublicKey = TOKEN_2022_PROGRAM_ID
) {
  let ata = getAssociatedTokenAddressSync(mint, receiver, null, tokenProgramId);
  let tx = new anchor.web3.Transaction();
  try {
    let ataAccount = await getAccount(
      provider.connection,
      ata,
      null,
      tokenProgramId
    );
  } catch (e) {
    if (
      e instanceof TokenAccountNotFoundError ||
      e instanceof TokenInvalidAccountOwnerError
    ) {
      tx.add(
        createAssociatedTokenAccountInstruction(
          provider.publicKey,
          ata,
          receiver,
          mint,
          tokenProgramId,
          ASSOCIATED_TOKEN_PROGRAM_ID
        )
      );
    }
  }

  let mintToIx = createMintToInstruction(
    mint,
    ata,
    provider.publicKey,
    amount,
    [],
    tokenProgramId
  );
  tx.add(mintToIx);
  await provider.sendAndConfirm(tx);
}
