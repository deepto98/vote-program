import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VoteProgram } from "../target/types/vote_program";

describe("vote-program", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VoteProgram as Program<VoteProgram>;

  const voteUrl = "https://deepto.xyz"

  const voteAccount = anchor.web3.PublicKey.findProgramAddressSync([
    Buffer.from(voteUrl),],
    program.programId)[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(voteUrl)
      .accountsPartial({        //pass accounts required for initialize instruction
        payer: provider.wallet.publicKey,
        voteState: voteAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const voteState = await program.account.voteState.fetch(voteAccount);
    console.log("Vote count for url is", voteState.score.toString());
  });
});
