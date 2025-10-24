import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterAnchor } from "../target/types/counter_anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("counter-anchor", () => {
  // Use the local test validator
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterAnchor as Program<CounterAnchor>;

  it("initializes and increments", async () => {
    // Create a new keypair for the counter account
    const counter = Keypair.generate();

    // Create (initialize) the counter account
    await program.methods.initializeCounter().accounts({
      payer: provider.wallet.publicKey,
      counter: counter.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([counter]).rpc();

    console.log("Counter account created:", counter.publicKey.toBase58());

    // Increment the counter
    await program.methods.increment().accounts({
      counter: counter.publicKey,
    }).rpc();

    await program.methods.increment().accounts({
      counter: counter.publicKey,
    }).rpc();

    // Fetch and read the on-chain data
    const account = await program.account.counter.fetch(counter.publicKey);
    console.log("Count value is now:", account.count.toString());

    // Simple assertion
    assert.equal(account.count.toNumber(), 2);
  });
});
