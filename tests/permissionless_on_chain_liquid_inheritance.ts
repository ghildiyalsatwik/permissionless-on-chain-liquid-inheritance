import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PermissionlessOnChainLiquidInheritance } from "../target/types/permissionless_on_chain_liquid_inheritance";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

const confirmTx = async (signature: string): Promise<string> => {

  const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();

  await anchor.getProvider().connection.confirmTransaction(
    
    {

      signature,
      ...latestBlockhash
    },

    "confirmed"
  );

  return signature;

};

describe("permissionless-on-chain-liquid-inheritance", () => {

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.PermissionlessOnChainLiquidInheritance as Program<PermissionlessOnChainLiquidInheritance>;

  console.log(`Program ID: ${program.programId.toBase58()}`);

  const admin = provider.wallet;

  console.log(`Admin address: ${admin.publicKey.toBase58()}`);

  const protocolMint = PublicKey.findProgramAddressSync([Buffer.from("mint")], program.programId)[0];

  const config = PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0];

  const vault = PublicKey.findProgramAddressSync([Buffer.from("vault")], program.programId)[0];

  const BPF_LOADER_UPGRADEABLE_PROGRAM_ID = new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");

  const programData = PublicKey.findProgramAddressSync([program.programId.toBuffer()], BPF_LOADER_UPGRADEABLE_PROGRAM_ID)[0];

  it("Program Data Account exists with upgrade authority", async () => {

    const programDataInfo = await provider.connection.getAccountInfo(programData);

    if(!programDataInfo) {

      throw new Error("ProgramData account does not exist");
    
    }

    const data = programDataInfo.data;

    const optionTag = data[12];

    console.log(`Option Tag: ${optionTag}`);

    if(optionTag === 1) {

      const upgradeAuthority = new PublicKey(data.slice(13, 45));

      console.log(`Upgrade Authority: ${upgradeAuthority.toBase58()}`);

      if(upgradeAuthority.toBase58() === admin.publicKey.toBase58()) {

        console.log("Program Data upgrade authority matches the admin from Anchor Provider");
      
      } else {

        throw new Error("Program Data upgrade authority does not match the wallet from Anchor provider");
      }
    
    } else {

      throw new Error("No upgrade authority set.");
    }

  });

  it("Airdrop", async () => {

      await Promise.all([admin].map(async (k) => {

        const sig = await anchor.getProvider().connection.requestAirdrop(k.publicKey, 3000 * anchor.web3.LAMPORTS_PER_SOL);

        await anchor.getProvider().connection.confirmTransaction(sig, "confirmed");
      
      }));

  });

  it("Initialize config", async () => {

    const tx = await program.methods.initializeConfig(new anchor.BN(100)).accountsStrict({

      admin: admin.publicKey,
      protocolMint,
      config,
      vault,
      thisProgram: program.programId,
      programData,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID
    }).rpc().then(confirmTx);

  });

  it("Update config fees before lock", async () => {

    try {
      await program.methods.updateConfigFees(new anchor.BN(200)).accountsStrict({
      admin: admin.publicKey,
      thisProgram: program.programId,
      programData,
      config,
      systemProgram: SystemProgram.programId
    }).rpc().then(confirmTx);
  } catch(err) {

    return;
  }

  throw new Error("Protocol is unlocoked, should not be able to update config fees");

  });

  it("Update config burned before lock", async () => {

    try {
      
      await program.methods.updateConfigBurned(new anchor.BN(1_000_000_000)).accountsStrict({

      admin: admin.publicKey,
      thisProgram: program.programId,
      programData,
      config,
      systemProgram: SystemProgram.programId
    }).rpc().then(confirmTx);
  } catch(err) {

    return;
  }

  throw new Error("Protocol is unlocoked, should not be able to update config burned");

  });

  it("Lock protocol", async () => {

    const tx = await program.methods.flipProtocol().accountsStrict({
      admin: admin.publicKey,
      thisProgram: program.programId,
      programData,
      config,
      systemProgram: SystemProgram.programId
    }).rpc().then(confirmTx);

  });

  it("Update config fees", async () => {

    const tx = await program.methods.updateConfigFees(new anchor.BN(200)).accountsStrict({
      admin: admin.publicKey,
      thisProgram: program.programId,
      programData,
      config,
      systemProgram: SystemProgram.programId
    }).rpc().then(confirmTx);

  });

  it("Update config burned", async () => {

    const tx1 = await program.methods.updateConfigBurned(new anchor.BN(1_000_000_000)).accountsStrict({

      admin: admin.publicKey,
      thisProgram: program.programId,
      programData,
      config,
      systemProgram: SystemProgram.programId
    }).rpc().then(confirmTx);

    const tx2 = await program.methods.updateConfigBurned(new anchor.BN(0)).accountsStrict({

      admin: admin.publicKey,
      thisProgram: program.programId,
      programData,
      config,
      systemProgram: SystemProgram.programId
    }).rpc().then(confirmTx);

  });

  it("Unlock protocol", async () => {

    const tx = await program.methods.flipProtocol().accountsStrict({
      admin: admin.publicKey,
      thisProgram: program.programId,
      programData,
      config,
      systemProgram: SystemProgram.programId
    }).rpc().then(confirmTx);

  });

});