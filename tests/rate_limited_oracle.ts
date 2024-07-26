import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { RateLimitedOracle } from "../target/types/rate_limited_oracle";
import { user } from "./utils";

describe("rate_limited_oracle", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .RateLimitedOracle as Program<RateLimitedOracle>;
  let oracle;
  it("Is initialized!", async () => {
    // Add your test here.
    const token_airdrop = await provider.connection.requestAirdrop(
      user.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: token_airdrop,
    });

    const [pda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("oracle")],
      program.programId
    );
    oracle = pda;
    const period = 60;
    await program.methods
      .initialize(new anchor.BN(period))
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    const state = await program.account.oracle.fetch(pda);

    expect(state.price.toNumber()).to.equal(0);
    expect(state.period.toNumber()).to.equal(period);
  });

  it("Updates the price!", async () => {
    const [oraclePda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("oracle")],
      program.programId
    );
    await program.methods
      .updatePrice(new anchor.BN(100))
      .accounts({
        oracle: oraclePda,
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    const state = await program.account.oracle.fetch(oraclePda);
    expect(state.price.toNumber()).to.equal(100);
  });
  it("Updates the period!", async () => {
    const [oraclePda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("oracle")],
      program.programId
    );
    await program.methods
      .updatePeriod(new anchor.BN(120))
      .accounts({
        oracle: oraclePda,
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    const state = await program.account.oracle.fetch(oraclePda);
    expect(state.period.toNumber()).to.equal(120);
  });
});
