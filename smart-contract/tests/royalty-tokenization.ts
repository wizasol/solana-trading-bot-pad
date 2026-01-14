import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RoyaltyTokenization } from "../target/types/royalty_tokenization";
import { 
  PublicKey, 
  Keypair, 
  SystemProgram,
  LAMPORTS_PER_SOL 
} from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { expect } from "chai";

describe("royalty-tokenization", () => {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RoyaltyTokenization as Program<RoyaltyTokenization>;
  
  const artist = Keypair.generate();
  const buyer = Keypair.generate();

  let projectPda: PublicKey;
  let mintPda: PublicKey;
  let treasuryPda: PublicKey;

  before(async () => {
    // Airdrop SOL to artist and buyer
    const airdrop1 = await provider.connection.requestAirdrop(
      artist.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    const airdrop2 = await provider.connection.requestAirdrop(
      buyer.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop1);
    await provider.connection.confirmTransaction(airdrop2);

    // Derive PDAs
    [projectPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("project"), artist.publicKey.toBuffer()],
      program.programId
    );

    [mintPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("mint"), projectPda.toBuffer()],
      program.programId
    );

    [treasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), projectPda.toBuffer()],
      program.programId
    );
  });

  it("Initializes a project", async () => {
    const tx = await program.methods
      .initializeProject(
        "Test Artist",
        "TART",
        new anchor.BN(1000000),
        50 // 50% royalty percentage
      )
      .accounts({
        artist: artist.publicKey,
        project: projectPda,
        mint: mintPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([artist])
      .rpc();

    console.log("Initialize transaction:", tx);

    const project = await program.account.project.fetch(projectPda);
    expect(project.name).to.equal("Test Artist");
    expect(project.symbol).to.equal("TART");
    expect(project.royaltyPercentage).to.equal(50);
  });

  it("Buys tokens", async () => {
    const buyerTokenAccount = await getAssociatedTokenAddress(
      mintPda,
      buyer.publicKey
    );

    // Check if token account exists, create if not
    const tokenAccountInfo = await provider.connection.getAccountInfo(buyerTokenAccount);
    if (!tokenAccountInfo) {
      const createATA = createAssociatedTokenAccountInstruction(
        buyer.publicKey,
        buyerTokenAccount,
        buyer.publicKey,
        mintPda
      );
      // This would need to be included in the transaction
    }

    const buyAmount = new anchor.BN(0.1 * LAMPORTS_PER_SOL);

    const tx = await program.methods
      .buyTokens(buyAmount)
      .accounts({
        buyer: buyer.publicKey,
        project: projectPda,
        mint: mintPda,
        buyerTokenAccount: buyerTokenAccount,
        treasury: treasuryPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();

    console.log("Buy tokens transaction:", tx);

    // Verify tokens were minted
    const tokenAccount = await provider.connection.getTokenAccountBalance(buyerTokenAccount);
    expect(tokenAccount.value.amount).to.equal(buyAmount.toString());
  });

  it("Distributes royalties", async () => {
    const distributionAmount = new anchor.BN(1 * LAMPORTS_PER_SOL);
    const oracle = artist.publicKey; // Using artist as oracle for test

    const tx = await program.methods
      .distributeRoyalties(distributionAmount)
      .accounts({
        project: projectPda,
        payer: artist.publicKey,
        oracle: oracle,
        treasury: treasuryPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([artist])
      .rpc();

    console.log("Distribute royalties transaction:", tx);

    const project = await program.account.project.fetch(projectPda);
    expect(project.totalDistributed.toNumber()).to.be.greaterThan(0);
  });

  it("Claims royalties", async () => {
    const buyerTokenAccount = await getAssociatedTokenAddress(
      mintPda,
      buyer.publicKey
    );

    const tx = await program.methods
      .claimRoyalties()
      .accounts({
        project: projectPda,
        mint: mintPda,
        holderTokenAccount: buyerTokenAccount,
        holder: buyer.publicKey,
        treasury: treasuryPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();

    console.log("Claim royalties transaction:", tx);
  });
});
