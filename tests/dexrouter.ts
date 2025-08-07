import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import { Dexrouter } from "../target/types/dexrouter";
import {
	createMint,
	getOrCreateAssociatedTokenAccount,
	mintTo,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("dexrouter", () => {
	const provider = anchor.AnchorProvider.env();
	// Configure the client to use the local cluster.
	anchor.setProvider(provider);

	const program = anchor.workspace.dexrouter as Program<Dexrouter>;

	let mintA: anchor.web3.PublicKey;
	let mintB: anchor.web3.PublicKey;
	let userAtaA, userAtaB;
	let vaultA, vaultB;
	let poolSigner, poolPDA;

	it("Initializes pools and performs weighted swap", async () => {
		const payer = provider.wallet;
		const connection = provider.connection;

		mintA = await createMint(connection, payer.payer, payer.publicKey, null, 6);
		mintB = await createMint(connection, payer.payer, payer.publicKey, null, 6);

		userAtaA = await getOrCreateAssociatedTokenAccount(
			connection,
			payer.payer,
			mintA,
			payer.publicKey
		);
		userAtaB = await getOrCreateAssociatedTokenAccount(
			connection,
			payer.payer,
			mintB,
			payer.publicKey
		);

		await mintTo(
			connection,
			payer.payer,
			mintA,
			userAtaA.address,
			payer.payer,
			1_000_000
		);
		await mintTo(
			connection,
			payer.payer,
			mintB,
			userAtaB.address,
			payer.payer,
			1_000_000
		);

		const [pool] = anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from("pool"), mintA.toBuffer(), mintB.toBuffer()],
			program.programId
		);

		poolPDA = pool;

		const [signer] = anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from("pool_signer")],
			program.programId
		);
		poolSigner = signer;

		vaultA = anchor.web3.Keypair.generate();
		vaultB = anchor.web3.Keypair.generate();

		await program.methods
			.initializePool(new anchor.BN(10), new anchor.BN(10))
      .accounts({
        // @ts-ignore
				pool,
				mintA,
				mintB,
				vaultA: vaultA.publicKey,
				vaultB: vaultB.publicKey,
				payer: payer.publicKey,
				poolSigner: poolSigner,
				tokenProgram: TOKEN_PROGRAM_ID,
				rent: anchor.web3.SYSVAR_RENT_PUBKEY,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.signers([vaultA, vaultB])
			.rpc();

		await program.methods
			.swapWeighted(new anchor.BN(1000))
			.accounts({
				// @ts-ignore
				pool,
				vaultA: vaultA.publicKey,
				vaultB: vaultB.publicKey,
				user: payer.publicKey,
				userTokenA: userAtaA.address,
				userTokenB: userAtaB.address,
				poolSigner: poolSigner,
				tokenProgram: TOKEN_PROGRAM_ID,
			})
			.rpc();
	});
});
