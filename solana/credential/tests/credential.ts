import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Credential } from "../target/types/credential";
import { expect } from "chai";

describe("credential", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Credential as Program<Credential>;
  const registryKeypair = anchor.web3.Keypair.generate();
  
  // Sample credential data
  const credentialHash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
  const credentialType = "education";
  const issuer = "University of Solana";
  const issueDate = new anchor.BN(Date.now() / 1000);
  const metadataUri = "ipfs://QmWWQSuPMS6aXCbZKpEjPHPUZN2NjB3YrhJTHsV4X3vb2t";
  
  // PDA for the credential
  let credentialPda;
  let credentialBump;

  before(async () => {
    // 注册表计数初始为0，我们使用这个作为种子
    const initialCountBytes = new Uint8Array(8);
    new anchor.BN(0).toBuffer().copy(initialCountBytes);
    
    // Find the credential PDA using the registry count as seed
    [credentialPda, credentialBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        registryKeypair.publicKey.toBuffer(),
        Buffer.from(initialCountBytes),
      ],
      program.programId
    );
  });

  it("Initializes the registry", async () => {
    // Initialize the registry
    await program.methods
      .initialize()
      .accounts({
        registry: registryKeypair.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([registryKeypair])
      .rpc();

    // Fetch the registry account
    const registry = await program.account.registry.fetch(registryKeypair.publicKey);
    
    // Verify account data
    expect(registry.authority.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    expect(registry.credentialCount.toNumber()).to.equal(0);
  });

  it("Registers a new credential", async () => {
    // Register credential
    await program.methods
      .registerCredential(
        credentialHash,
        credentialType,
        issuer,
        issueDate,
        metadataUri
      )
      .accounts({
        registry: registryKeypair.publicKey,
        credential: credentialPda,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the registry and credential accounts
    const registry = await program.account.registry.fetch(registryKeypair.publicKey);
    const credential = await program.account.credential.fetch(credentialPda);
    
    // Verify registry state
    expect(registry.credentialCount.toNumber()).to.equal(1);
    
    // Verify credential data
    expect(credential.hash).to.equal(credentialHash);
    expect(credential.credentialType).to.equal(credentialType);
    expect(credential.issuer).to.equal(issuer);
    expect(credential.issueDate.toNumber()).to.equal(issueDate.toNumber());
    expect(credential.metadataUri).to.equal(metadataUri);
    expect(credential.owner.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    expect(credential.revoked).to.equal(false);
    
    // Verify that seeds were stored correctly
    const seedsAsNumber = new anchor.BN(credential.seeds, null, "le").toNumber();
    expect(seedsAsNumber).to.equal(0); // 第一个凭证应该使用计数 0 作为种子
  });

  it("Verifies a credential", async () => {
    // Verify credential
    await program.methods
      .verifyCredential()
      .accounts({
        registry: registryKeypair.publicKey,
        credential: credentialPda,
      })
      .rpc();
    
    // No assertion needed - if it completes without error, the credential is valid
  });

  it("Updates credential metadata", async () => {
    const newMetadataUri = "ipfs://QmNewHash123456789abcdefghijklmnopqrstuvwxyz123456";
    
    // Update credential metadata
    await program.methods
      .updateMetadata(newMetadataUri)
      .accounts({
        registry: registryKeypair.publicKey,
        credential: credentialPda,
        authority: provider.wallet.publicKey,
      })
      .rpc();
    
    // Fetch updated credential
    const credential = await program.account.credential.fetch(credentialPda);
    
    // Verify updated metadata
    expect(credential.metadataUri).to.equal(newMetadataUri);
  });

  it("Revokes a credential", async () => {
    // Revoke credential
    await program.methods
      .revokeCredential()
      .accounts({
        registry: registryKeypair.publicKey,
        credential: credentialPda,
        authority: provider.wallet.publicKey,
      })
      .rpc();
    
    // Fetch updated credential
    const credential = await program.account.credential.fetch(credentialPda);
    
    // Verify revocation state
    expect(credential.revoked).to.equal(true);
  });

  it("Fails to verify a revoked credential", async () => {
    try {
      // Try to verify revoked credential
      await program.methods
        .verifyCredential()
        .accounts({
          registry: registryKeypair.publicKey,
          credential: credentialPda,
        })
        .rpc();
      
      // Should not reach here
      expect.fail("Expected verification to fail for revoked credential");
    } catch (error) {
      // Verify error message
      expect(error.message).to.include("CredentialRevoked");
    }
  });
});
