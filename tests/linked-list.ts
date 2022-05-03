import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LinkedList } from "../target/types/linked_list";
import {
  PublicKey,
  Keypair,
  SystemProgram
} from "@solana/web3.js";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
const assert = require("assert");

describe("linked-list", () => {
  // Configure the client to use the local cluster.
  const baseProvider = anchor.AnchorProvider.local();
  const TEST_PAYER = Keypair.fromSecretKey(
    (baseProvider.wallet as NodeWallet).payer.secretKey
  );

  const program = anchor.workspace.LinkedList as Program<LinkedList>;

  it("Is initialized!", async () => {
    const [nodeAddress, bump] = await PublicKey.findProgramAddress(
      [baseProvider.wallet.publicKey.toBytes()],
      program.programId
    );

    const tx = await program.rpc.initialize(
      {
        accounts: {
          node: nodeAddress,
          signer: baseProvider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [TEST_PAYER],
      }
    );

    const node = await program.account.node.fetch(nodeAddress)
    assert.ok(node.value == 1);
    assert.ok(node.nextNode.equals(PublicKey.default));
  });

  it("It can add node!", async () => {
    const [oldAddress, bump] = await PublicKey.findProgramAddress(
      [baseProvider.wallet.publicKey.toBytes()],
      program.programId
    );

    const newNodeKp = Keypair.generate();

    const tx = await program.rpc.addNode(
      {
        accounts: {
          oldNode: oldAddress,
          newNode: newNodeKp.publicKey,
          signer: baseProvider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [TEST_PAYER, newNodeKp], // HAVE TO think about who owns these, how to keep signatures valid, obviously it should be PDAs but hmm
      }
    );

    const prevNode = await program.account.node.fetch(oldAddress)
    console.log(prevNode)
    assert.ok(prevNode.value == 1);
    assert.ok(prevNode.nextNode.equals(newNodeKp.publicKey));

    const newNode = await program.account.node.fetch(newNodeKp.publicKey)
    console.log(newNode)
  });


  // it("It can add a lot of nodes", async () => {
  // });

  // it("It can read a lot of nodes", async () => {
  // });

});
