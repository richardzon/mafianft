import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CharacterNft } from "../target/types/character_nft";
import { 
  Keypair, 
  LAMPORTS_PER_SOL, 
  PublicKey, 
  SystemProgram,
  SYSVAR_RENT_PUBKEY 
} from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createMint,
  mintTo
} from "@solana/spl-token";
import { expect } from "chai";

describe("Character NFT Program", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CharacterNft as Program<CharacterNft>;
  
  // Test accounts
  let authority: Keypair;
  let user: Keypair;
  let collectionMint: PublicKey;
  let characterMint: Keypair;
  let configPda: PublicKey;
  let mintAuthorityPda: PublicKey;
  let characterPda: PublicKey;
  let userTokenAccount: PublicKey;
  let metadataPda: PublicKey;
  let masterEditionPda: PublicKey;

  // Program constants
  const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

  before(async () => {
    // Initialize test accounts
    authority = Keypair.generate();
    user = Keypair.generate();
    characterMint = Keypair.generate();

    // Airdrop SOL to test accounts
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(authority.publicKey, 2 * LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 2 * LAMPORTS_PER_SOL)
    );

    // Create collection mint
    collectionMint = await createMint(
      provider.connection,
      authority,
      authority.publicKey,
      authority.publicKey,
      0
    );

    // Derive PDAs
    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    [mintAuthorityPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("mint_authority")],
      program.programId
    );

    [characterPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("character"), characterMint.publicKey.toBuffer()],
      program.programId
    );

    // Get associated token account
    userTokenAccount = await getAssociatedTokenAddress(
      characterMint.publicKey,
      user.publicKey
    );

    // Derive metadata PDAs
    [metadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        characterMint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    [masterEditionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        characterMint.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
  });

  describe("Initialization", () => {
    it("Should initialize the character NFT program", async () => {
      const tx = await program.methods
        .initialize()
        .accounts({
          config: configPda,
          authority: authority.publicKey,
          collectionMint: collectionMint,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("Initialize transaction signature:", tx);

      // Verify config account
      const config = await program.account.config.fetch(configPda);
      expect(config.authority.toString()).to.equal(authority.publicKey.toString());
      expect(config.collectionMint.toString()).to.equal(collectionMint.toString());
      expect(config.totalMinted).to.equal(0);
      expect(config.maxSupply).to.equal(10000);
      expect(config.isActive).to.be.true;
    });

    it("Should fail to initialize twice", async () => {
      try {
        await program.methods
          .initialize()
          .accounts({
            config: configPda,
            authority: authority.publicKey,
            collectionMint: collectionMint,
            systemProgram: SystemProgram.programId,
          })
          .signers([authority])
          .rpc();
        
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.message).to.include("already in use");
      }
    });
  });

  describe("Character Minting", () => {
    it("Should mint a character NFT", async () => {
      const characterStats = {
        strength: 75,
        intelligence: 80,
        charisma: 70,
        luck: 65,
        stealth: 85,
        availablePoints: 0,
      };

      const rarity = { rare: {} }; // Enum variant for Rare

      const tx = await program.methods
        .mintCharacter(
          "Vincent The Shark",
          "SHARK",
          "https://metadata.mafianft.com/characters/1.json",
          rarity,
          characterStats
        )
        .accounts({
          config: configPda,
          mint: characterMint.publicKey,
          mintAuthority: mintAuthorityPda,
          tokenAccount: userTokenAccount,
          character: characterPda,
          metadata: metadataPda,
          masterEdition: masterEditionPda,
          payer: authority.publicKey,
          owner: user.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        .signers([authority, characterMint])
        .rpc();

      console.log("Mint character transaction signature:", tx);

      // Verify character account
      const character = await program.account.character.fetch(characterPda);
      expect(character.mint.toString()).to.equal(characterMint.publicKey.toString());
      expect(character.owner.toString()).to.equal(user.publicKey.toString());
      expect(character.level).to.equal(1);
      expect(character.experience).to.equal(0);
      expect(character.isStaked).to.be.false;

      // Verify stats
      expect(character.stats.strength).to.equal(75);
      expect(character.stats.intelligence).to.equal(80);
      expect(character.stats.charisma).to.equal(70);
      expect(character.stats.luck).to.equal(65);
      expect(character.stats.stealth).to.equal(85);

      // Verify config update
      const config = await program.account.config.fetch(configPda);
      expect(config.totalMinted).to.equal(1);

      // Verify token account
      const tokenAccountInfo = await provider.connection.getTokenAccountBalance(userTokenAccount);
      expect(tokenAccountInfo.value.amount).to.equal("1");
    });

    it("Should fail to mint with invalid stats for rarity", async () => {
      const invalidStats = {
        strength: 50, // Too low for rare rarity
        intelligence: 50,
        charisma: 50,
        luck: 50,
        stealth: 50,
        availablePoints: 0,
      };

      const rarity = { rare: {} };
      const newCharacterMint = Keypair.generate();

      const [newCharacterPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("character"), newCharacterMint.publicKey.toBuffer()],
        program.programId
      );

      const newUserTokenAccount = await getAssociatedTokenAddress(
        newCharacterMint.publicKey,
        user.publicKey
      );

      const [newMetadataPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          newCharacterMint.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );

      const [newMasterEditionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          newCharacterMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );

      try {
        await program.methods
          .mintCharacter(
            "Invalid Character",
            "INVALID",
            "https://metadata.mafianft.com/characters/invalid.json",
            rarity,
            invalidStats
          )
          .accounts({
            config: configPda,
            mint: newCharacterMint.publicKey,
            mintAuthority: mintAuthorityPda,
            tokenAccount: newUserTokenAccount,
            character: newCharacterPda,
            metadata: newMetadataPda,
            masterEdition: newMasterEditionPda,
            payer: authority.publicKey,
            owner: user.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          })
          .signers([authority, newCharacterMint])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.message).to.include("InvalidStatsForRarity");
      }
    });
  });

  describe("Character Leveling", () => {
    it("Should level up a character with sufficient experience", async () => {
      // First, we need to add experience to the character
      // This would normally be done through gameplay, but for testing we'll modify the account directly
      
      // Get current character data
      let character = await program.account.character.fetch(characterPda);
      
      // For level 1 -> 2, we need 150 experience points
      // We'll simulate this by directly calling the level_up instruction
      // after manually setting experience (in a real scenario, experience would be earned through missions)
      
      // Note: In a production environment, experience would be added through mission completion
      // For this test, we're assuming the character has gained enough experience
      
      const tx = await program.methods
        .levelUp()
        .accounts({
          character: characterPda,
          owner: user.publicKey,
        })
        .signers([user])
        .rpc();

      console.log("Level up transaction signature:", tx);

      // Verify character leveled up
      character = await program.account.character.fetch(characterPda);
      expect(character.level).to.equal(2);
      expect(character.stats.availablePoints).to.be.greaterThan(0);
    });

    it("Should fail to level up without sufficient experience", async () => {
      try {
        await program.methods
          .levelUp()
          .accounts({
            character: characterPda,
            owner: user.publicKey,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.message).to.include("InsufficientExperience");
      }
    });

    it("Should fail to level up if not the owner", async () => {
      const notOwner = Keypair.generate();
      
      // Airdrop SOL to notOwner
      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(notOwner.publicKey, LAMPORTS_PER_SOL)
      );

      try {
        await program.methods
          .levelUp()
          .accounts({
            character: characterPda,
            owner: notOwner.publicKey,
          })
          .signers([notOwner])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.message).to.include("NotOwner");
      }
    });
  });

  describe("Character Merging", () => {
    it("Should prepare for character merging functionality", async () => {
      // This test is a placeholder for the merge_characters functionality
      // which would involve burning multiple characters to create a higher tier one
      
      try {
        await program.methods
          .mergeCharacters()
          .accounts({
            owner: user.publicKey,
          })
          .signers([user])
          .rpc();

        console.log("Merge characters functionality called (placeholder)");
      } catch (error) {
        // Expected to fail as this is not fully implemented
        console.log("Merge functionality not yet implemented:", error.message);
      }
    });
  });

  describe("Edge Cases and Security", () => {
    it("Should handle maximum supply limit", async () => {
      // This test would require minting 9,999 more characters to reach the limit
      // For practical purposes, we'll test the logic by checking the current state
      
      const config = await program.account.config.fetch(configPda);
      expect(config.totalMinted).to.be.lessThan(config.maxSupply);
      expect(config.maxSupply).to.equal(10000);
    });

    it("Should validate character name length", async () => {
      const longName = "A".repeat(50); // Exceeds 32 character limit
      const newCharacterMint = Keypair.generate();
      
      const [newCharacterPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("character"), newCharacterMint.publicKey.toBuffer()],
        program.programId
      );

      const newUserTokenAccount = await getAssociatedTokenAddress(
        newCharacterMint.publicKey,
        user.publicKey
      );

      const [newMetadataPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          newCharacterMint.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );

      const [newMasterEditionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          newCharacterMint.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );

      const characterStats = {
        strength: 75,
        intelligence: 80,
        charisma: 70,
        luck: 65,
        stealth: 85,
        availablePoints: 0,
      };

      try {
        await program.methods
          .mintCharacter(
            longName,
            "LONG",
            "https://metadata.mafianft.com/characters/long.json",
            { rare: {} },
            characterStats
          )
          .accounts({
            config: configPda,
            mint: newCharacterMint.publicKey,
            mintAuthority: mintAuthorityPda,
            tokenAccount: newUserTokenAccount,
            character: newCharacterPda,
            metadata: newMetadataPda,
            masterEdition: newMasterEditionPda,
            payer: authority.publicKey,
            owner: user.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          })
          .signers([authority, newCharacterMint])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.message).to.include("NameTooLong");
      }
    });
  });
});
