import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { Dexrouter } from "../target/types/dexrouter";

describe("dexrouter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.dexrouter as Program<Dexrouter>;

  // let mintA: anchor.web3.PublicKey;
  // let mintB: anchor.web3.PublicKey;
  //
  // let userAtaA, userAtaB;
  // let vaultA, vaultB;
  // let poolSigner, poolPDA;

  it('Initializes pools and performs routed swap', async () => {
    let user = anchor.getProvider().publicKey;
    let poolA = anchor.web3.Keypair.generate();
    let poolB = anchor.web3.Keypair.generate();

    const lamports = await program.provider.connection.getMinimumBalanceForRentExemption(16);

    await program.methods.initialize(new anchor.BN(10), new anchor.BN(10))
        .accounts({
          pool: poolA.publicKey,
          payer: user,
        })
        .signers([poolA])
        .rpc()

    await program.methods.initialize(new anchor.BN(1000), new anchor.BN(1000))
        .accounts({
          pool: poolB.publicKey,
          payer: user,
        })
        .signers([poolB])
        .rpc()

    await program.methods.routeSwap(new anchor.BN(1000))
        .accounts({
          poolA: poolA.publicKey,
          poolB: poolB.publicKey,
          user,
        })
        .rpc()
  });


  it("Is initialized!", async () => {
    // Add your test here.
    let user = anchor.getProvider().publicKey;
    let poolA = anchor.web3.Keypair.generate();

    const tx = await program.methods.initialize(new anchor.BN(1000), new anchor.BN(1000))
        .accounts({
            pool: poolA.publicKey,
            payer: user,
        })
        .signers([poolA])
        .rpc();
    console.log("Your transaction signature", tx);
  });
});
