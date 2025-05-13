import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {AtomicswapSolana} from "../target/types/atomicswap_solana";
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, SYSVAR_CLOCK_PUBKEY, Transaction} from "@solana/web3.js";
import {BN} from "bn.js";
import {ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
    createAssociatedTokenAccountIdempotentInstruction,
    createInitializeMint2Instruction,
    createMintToInstruction,
    getAssociatedTokenAddressSync,
    getMinimumBalanceForRentExemptMint,
    MINT_SIZE
} from "@solana/spl-token";

describe("atomicswap_solana", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const provider = anchor.getProvider();

    const connection = provider.connection;

    const program = anchor.workspace.AtomicswapSolana as Program<AtomicswapSolana>;

    const confirm = async (signature: string): Promise<string> => {
        const block = await connection.getLatestBlockhash();
        await connection.confirmTransaction({
            signature,
            ...block,
        });
        return signature;
    };

    const log = async (signature: string): Promise<string> => {
        console.log(
            `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
        );
        return signature;
    };

    const fee = new BN(500);
    const swapId = Buffer.from("32-byte-string-here-1234567890123456").subarray(0, 32);
    const secretKey = [
        125, 149, 215, 216, 215, 68, 246, 157, 132, 148, 25, 101, 167, 114, 168, 136, 197,
        7, 93, 34, 129, 103, 21, 179, 167, 102, 180, 151, 13, 145, 182, 73
    ];
    const secretHash = [
        40, 103, 196, 181, 43, 147, 6, 208, 75, 94, 70, 165, 120, 50, 192, 67, 27, 192, 26,
        6, 51, 49, 0, 237, 26, 66, 172, 205, 85, 54, 64, 152
    ];
    const signer = Keypair.generate();
    const buyer = Keypair.generate();
    const mintToken = Keypair.generate();
    const pool = PublicKey.findProgramAddressSync([
            Buffer.from("pool"),
            mintToken.publicKey.toBuffer()
        ],
        program.programId)[0];
    const swap = PublicKey.findProgramAddressSync([
            Buffer.from("swap"),
            Buffer.from(pool.toBytes()),
            swapId
        ],
        program.programId)[0];
    const tokenProgram = TOKEN_PROGRAM_ID;

    const poolAta = getAssociatedTokenAddressSync(
        mintToken.publicKey,
        pool,
        true,
        tokenProgram
    );

    const signerAta = getAssociatedTokenAddressSync(
        mintToken.publicKey,
        signer.publicKey,
        false,
        tokenProgram
    );

    const buyerAta = getAssociatedTokenAddressSync(
        mintToken.publicKey,
        buyer.publicKey,
        false,
        tokenProgram
    );

    const accounts = {
        signer: signer.publicKey,
        mintToken: mintToken.publicKey,
        pool,
        swap,
        signerAta,
        buyerAta,
        poolAta,
        tokenProgram,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        time: SYSVAR_CLOCK_PUBKEY,
    }

    it("Airdrop and create mints", async () => {
        let lamports = await getMinimumBalanceForRentExemptMint(connection);
        let tx = new Transaction();
        tx.instructions = [
            SystemProgram.transfer({
                fromPubkey: provider.publicKey,
                toPubkey: signer.publicKey,
                lamports: 10 * LAMPORTS_PER_SOL,
            }),
            ...[mintToken].map((mint) =>
                SystemProgram.createAccount({
                    fromPubkey: provider.publicKey,
                    newAccountPubkey: mint.publicKey,
                    lamports,
                    space: MINT_SIZE,
                    programId: tokenProgram,
                })
            ),
            createInitializeMint2Instruction(mintToken.publicKey, 6, provider.publicKey!, null, tokenProgram),
            createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, signerAta, signer.publicKey, mintToken.publicKey, tokenProgram),
            createMintToInstruction(mintToken.publicKey, signerAta, provider.publicKey!, 1e9, undefined, tokenProgram),
            createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, buyerAta, buyer.publicKey, mintToken.publicKey, tokenProgram),
            createMintToInstruction(mintToken.publicKey, buyerAta, provider.publicKey!, 1e9, undefined, tokenProgram)
        ];

        await provider.sendAndConfirm(tx, [mintToken]).then(log);
    });

    it("Initialize", async () => {
        await program.methods.initialize(
            fee.toNumber()
        )
            .accountsStrict({
                ...accounts
            })
            .signers([
                signer
            ])
            .rpc()
            .then(confirm)
            .then(log);
    });

    it("Deposit", async () => {
        // const lockTimestamp = Math.floor((Date.now() - 100000) / 1000); // redund
        const lockTimestamp = Math.floor(Date.now() / 1000); // redeem
        await program.methods.deposit(
            swapId,
            new BN(lockTimestamp),
            secretHash,
            buyer.publicKey,
            new BN(2500),
        )
            .accountsStrict({
                ...accounts
            })
            .signers([
                signer
            ])
            .rpc()
            .then(confirm)
            .then(log);

    });

    it("Redeem", async () => {
        await program.methods.redeem(swapId, secretKey)
            .accountsStrict({
                ...accounts
            })
            .signers([
                signer
            ])
            .rpc()
            .then(confirm)
            .then(log);
    });

    it("Refund", async () => {
        // await program.methods.refund(swapId)
        //     .accountsStrict({
        //         ...accounts
        //     })
        //     .signers([
        //         signer
        //     ])
        //     .rpc()
        //     .then(confirm)
        //     .then(log);
    });
});
