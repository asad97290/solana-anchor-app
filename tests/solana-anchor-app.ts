import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaAnchorApp } from "../target/types/solana_anchor_app";
import { assert } from "chai";

describe("solana-anchor-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaAnchorApp as Program<SolanaAnchorApp>;
  const seed = "master" 
  it("Is initialized!", async () => {

    // Initialize the program.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction hash", tx);

    // Determine the PDA for the new_account
    const [newAccountPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(seed)],
      program.programId
    );

    // Fetch the newly created account
    const account = await program.account.newAccount.fetch(newAccountPDA);

    // Log the data from the account
    assert.ok(account.data == 1);
    assert.ok(account.text == "Hello");


    await program.methods.initialize2(1997).rpc();
    const account2 = await program.account.newAccount.fetch(newAccountPDA);
    assert.ok(account2.data == 1997);
  });
});
