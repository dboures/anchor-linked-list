import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LinkedList } from "../target/types/linked_list";

describe("linked-list", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LinkedList as Program<LinkedList>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
