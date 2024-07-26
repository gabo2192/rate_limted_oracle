import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { RateLimitedOracle } from "../target/types/rate_limited_oracle";
import { user } from "./utils";

describe("rate_limited_oracle", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .RateLimitedOracle as Program<RateLimitedOracle>;

  it("Is initialized!", async () => {
    // Add your test here.
    const [pda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("oracle")],
      program.programId
    );
    console.log(user.publicKey);
    // await program.methods
    //   .initialize()
    //   .accounts({
    //     user: user.publicKey,
    //   })
    //   .signers([user])
    //   .rpc();
    const state = await program.account.oracle.fetch(pda);

    expect(state.price.toNumber()).to.equal(0);
  });
});
