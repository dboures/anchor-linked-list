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

  let program: any;
  let baseProvider: anchor.AnchorProvider;
  let TEST_PAYER: Keypair;

  let nodeStack: Keypair;
  // let bump: number;
  // let secondNodeKp: Keypair;

  before(async () => {

    baseProvider = anchor.AnchorProvider.local();
    TEST_PAYER = Keypair.fromSecretKey(
      (baseProvider.wallet as NodeWallet).payer.secretKey
    );
  
    program = anchor.workspace.LinkedList as Program<LinkedList>;
  
    // [stack, bump] = await PublicKey.findProgramAddress(
    //   [baseProvider.wallet.publicKey.toBytes()],
    //   program.programId
    // );

    nodeStack = Keypair.generate()

  })

  it("It can initialize zero copy account", async () => {

    const size = 967502 + 8; // Account size in bytes.

    const tx = await program.rpc.initialize(
      {
        accounts: {
          stack: nodeStack.publicKey,
          signer: baseProvider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        instructions: [await program.account.nodeStack.createInstruction(nodeStack, size)],
        signers: [TEST_PAYER, nodeStack],
      }
    );

    const stackAccount = await program.account.nodeStack.fetch(nodeStack.publicKey);
    assert.ok(stackAccount.nodes.length == 750);
    assert.ok(stackAccount.idx == 0);
  });

  it("It can add a node", async () => {
    const tx = await program.rpc.addNode(
      {
        accounts: {
          stack: nodeStack.publicKey,
          signer: baseProvider.wallet.publicKey,
        },
        signers: [TEST_PAYER],
      }
    );

    const stackAccount = await program.account.nodeStack.fetch(nodeStack.publicKey)
    const newNode = stackAccount.nodes[stackAccount.idx - 1];

    assert.ok(stackAccount.nodes.length == 750);
    assert.ok(stackAccount.idx == 1);
    assert.ok(newNode.value == 21);
    assert.ok(newNode.foo.equals(PublicKey.default));
  });

  it("It will allow us to iterate over accounts", async () => {
    const tx = await program.rpc.iterate(
      {
        accounts: {
          stack: nodeStack.publicKey,
          signer: baseProvider.wallet.publicKey,
        },
        signers: [TEST_PAYER],
      }
    );
  });
    
    it("It will break (unpredictably) if we iterate over too many accounts", async () => { // something to think about
      try {
        const tx = await program.rpc.breakStack(
          {
            accounts: {
              stack: nodeStack.publicKey,
              signer: baseProvider.wallet.publicKey,
            },
            signers: [TEST_PAYER],
          }
        );
      } catch (error) {
        console.log(error)
      }
    });

});
