import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MagicblockEr } from "../target/types/magicblock_er";
import { GetCommitmentSignature } from "@magicblock-labs/ephemeral-rollups-sdk";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("magicblock-er", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const providerEphemeralRollup = new anchor.AnchorProvider(
		new anchor.web3.Connection(
			process.env.EPHEMERAL_PROVIDER_ENDPOINT ||
				"https://devnet.magicblock.app/",
			{
				wsEndpoint:
					process.env.EPHEMERAL_WS_ENDPOINT ||
					"wss://devnet.magicblock.app/",
			}
		),
		anchor.Wallet.local()
	);

	console.log("Base Layer Connection: ", provider.connection.rpcEndpoint);
	console.log(
		"Ephemeral Rollup Connection: ",
		providerEphemeralRollup.connection.rpcEndpoint
	);
	console.log(`Current SOL Public Key: ${anchor.Wallet.local().publicKey}`);

	before(async function () {
		const balance = await provider.connection.getBalance(
			anchor.Wallet.local().publicKey
		);
		console.log(
			`Current balance is : `,
			balance / LAMPORTS_PER_SOL,
			"SOL",
			"\n"
		);
	});

	const program = anchor.workspace.magicblockEr as Program<MagicblockEr>;

	const userAccount = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("user"), anchor.Wallet.local().publicKey.toBuffer()],
		program.programId
	)[0];

	it("Is initialized!", async () => {
		const tx = await program.methods
			.initialize()
			.accountsPartial({
				user: anchor.Wallet.local().publicKey,
				userAccount: userAccount,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();

		console.log("User account initialized: ", tx);
	});

	it("Update without VRF should fail", async () => {
		try{
			const definedArr = new Array(32).fill(1);
			await program.methods.update(definedArr as any).accountsPartial({
				user: anchor.Wallet.local().publicKey,
				userAccount: userAccount,
			}).rpc();
			throw new Error("Expected to faile with VRF")
		}catch(error) {
			console.log("Failed without VRF: ", error);
		}
	})

	it("Update State!", async () => {
		const tx = await program.methods
			.update(new anchor.BN(12))
			.accountsPartial({
				user: anchor.Wallet.local().publicKey,
				userAccount: userAccount,
			})
			.rpc();

		console.log("User account State Updated: ", tx);
	});

	it("Delegate to ephemeral Rollup!", async () => {
		let tx = await program.methods
			.delegate()
			.accountsPartial({
				user: anchor.Wallet.local().publicKey,
				userAccount: userAccount,
				validator: new PublicKey(
					"MAS1Dt9qreoRMQ14YQuhg8UTZMMzDdKhmkZMECCzk57"
				),
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc({ skipPreflight: true });

		console.log("\nUser Account Delegated to Ephemeral Rollup: ", tx);
	});

	it("Update State and Commit to Base Layer!", async () => {
		let tx = await program.methods
			.updateCommit(new anchor.BN(1212))
			.accountsPartial({
				user: providerEphemeralRollup.wallet.publicKey,
				userAccount: userAccount,
			})
			.transaction();

		tx.feePayer = providerEphemeralRollup.wallet.publicKey;

		tx.recentBlockhash = (
			await providerEphemeralRollup.connection.getLatestBlockhash()
		).blockhash;

		tx = await providerEphemeralRollup.wallet.signTransaction(tx);

		const txHash = await providerEphemeralRollup.sendAndConfirm(tx, [], {
			skipPreflight: false,
		});
		const txCommitSgn = await GetCommitmentSignature(
			txHash,
			providerEphemeralRollup.connection
		);

		console.log("\nUser Account State Updated: ", txHash);
		console.log("\nUser Account State Updated tx Commitment signature: ", txCommitSgn);
	});

  it("Commit and undelegate from Ephemeral Rollup", async () => {
    let info = await providerEphemeralRollup.connection.getAccountInfo(userAccount)

    console.log("User Account Info: ", info);

    console.log("User account", userAccount.toBase58());

    let tx = await program.methods.undelegate().accounts({
      user: providerEphemeralRollup.wallet.publicKey,
    }).transaction();

    tx.feePayer = providerEphemeralRollup.wallet.publicKey;

    tx.recentBlockhash = (await providerEphemeralRollup.connection.getLatestBlockhash()).blockhash;

    tx = await providerEphemeralRollup.wallet.signTransaction(tx);
    const txHash = await providerEphemeralRollup.sendAndConfirm(tx, [], {
      skipPreflight: false
    });

    const txCommitSign = await GetCommitmentSignature(
      txHash,
      providerEphemeralRollup.connection
    )

    console.log("\nUser Account Undelegated: ", txHash);
    console.log("\nUser Account Undelegated(CommitmentSignature): ", txCommitSign);
  })

  it("Update State!", async () => {
    let tx = await program.methods.update(new anchor.BN(1212)).accountsPartial({
      user: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
    })
    .rpc();

    console.log("\nUser Account State Updated: ", tx);
  });

  it("Close Account!", async () => {
    let balance = await provider.connection.getBalance(anchor.Wallet.local().publicKey)
    console.log("Balance Before : ", balance)
    
    const tx = await program.methods.close().accountsPartial({
      user: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("\nUser Account Closed: ", tx);
    let balanceAfter = await provider.connection.getBalance(anchor.Wallet.local().publicKey)
    console.log("Balance After: ", balanceAfter);
  });
});
