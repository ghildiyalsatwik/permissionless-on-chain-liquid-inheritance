import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PermissionlessOnChainLiquidInheritance } from "../target/types/permissionless_on_chain_liquid_inheritance";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddressSync, ASSOCIATED_TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

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

  const maker = Keypair.generate();

  const keeper = Keypair.generate();

  const withdrawer = Keypair.generate();

  const hacker = Keypair.generate();

  const hackerInheritor = Keypair.generate();

  const hackerInActivityTime = new anchor.BN(1);

  const hackerWithdrawAmount = new anchor.BN(1_000_000_000);

  const inheritor1 = Keypair.generate();

  const inheritor2 = Keypair.generate();

  const finalInheritor = Keypair.generate();

  const inheritor1New = Keypair.generate();

  const seed1 = new anchor.BN(0);

  const seed2 = new anchor.BN(1);

  const finalSeed = new anchor.BN(0);

  const inactivityTime1 = new anchor.BN(60 * 60 * 24);

  const inactivityTime2 = new anchor.BN(1);

  const inactivityTime1New = new anchor.BN(60 * 60 * 24 * 2);

  const finalInActicityTime = new anchor.BN(10);

  const inheritanceAmount1 = new anchor.BN(1_000_000_000);

  const inheritanceAmount1ToAdd = new anchor.BN(1_000_000_000);

  const inhertianceAmount1ToRemove = new anchor.BN(1_000_000_000);

  const inhertianceAmount2ToRemove = new anchor.BN(500_000_000);

  const finalInheritanceAmount = new anchor.BN(5_000_000_000);

  const bountyAmount1 = new anchor.BN(100_000_000);

  const bountyAmount1New = new anchor.BN(200_000_000);

  const finalBountyAmount = new anchor.BN(1_000_000_000);

  const inheritancePDA1 = PublicKey.findProgramAddressSync([Buffer.from("inheritance"), maker.publicKey.toBuffer(), inheritor1.publicKey.toBuffer(), seed1.toArrayLike(Buffer, "le", 8)], program.programId)[0];

  const inheritanceVault1 = PublicKey.findProgramAddressSync([Buffer.from("inheritance_vault"), inheritancePDA1.toBuffer()], program.programId)[0];

  console.log(`Inheritance Vault Address of first inheritor: ${inheritanceVault1.toBase58()}`);

  console.log(`Inheritance PDA address of the first inheritor: ${inheritancePDA1.toBase58()}`);

  const inheritancePDA2 = PublicKey.findProgramAddressSync([Buffer.from("inheritance"), maker.publicKey.toBuffer(), inheritor1.publicKey.toBuffer(), seed2.toArrayLike(Buffer, "le", 8)], program.programId)[0];

  const inheritanceVault2 = PublicKey.findProgramAddressSync([Buffer.from("inheritance_vault"), inheritancePDA2.toBuffer()], program.programId)[0];

  const inheritancePDA3 = PublicKey.findProgramAddressSync([Buffer.from("inheritance"), maker.publicKey.toBuffer(), inheritor2.publicKey.toBuffer(), seed2.toArrayLike(Buffer, "le", 8)], program.programId)[0];

  const inheritanceVault3 = PublicKey.findProgramAddressSync([Buffer.from("inheritance_vault"), inheritancePDA3.toBuffer()], program.programId)[0];

  const finalInheritancePDA = PublicKey.findProgramAddressSync([Buffer.from("inheritance"), maker.publicKey.toBuffer(), finalInheritor.publicKey.toBuffer(), finalSeed.toArrayLike(Buffer, "le", 8)], program.programId)[0];

  const finalInheritanceVault = PublicKey.findProgramAddressSync([Buffer.from("inheritance_vault"), finalInheritancePDA.toBuffer()], program.programId)[0];

  const makerAta1 = getAssociatedTokenAddressSync(protocolMint, maker.publicKey, false, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

  const hackerAta = getAssociatedTokenAddressSync(protocolMint, hacker.publicKey, false, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

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

      await Promise.all([admin, maker, keeper, withdrawer, hacker].map(async (k) => {

        const sig = await anchor.getProvider().connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL);

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

  it("Initialize first inheritance", async () => {

    const tx = await program.methods.initializeInheritance(seed1, inheritor1.publicKey, inheritanceAmount1, bountyAmount1, inactivityTime1)
    .accountsStrict({
      maker: maker.publicKey,
      config,
      vault,
      protocolMint,
      makerAta: makerAta1,
      inheritance: inheritancePDA1,
      inheritanceVault: inheritanceVault1,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Check in, first inheritance", async () => {

    const tx = await program.methods.checkIn().accountsStrict({
      maker: maker.publicKey,
      inheritance: inheritancePDA1,
      config,
      systemProgram: SystemProgram.programId
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Change inheritor, first inheritance", async () => {

    const tx = await program.methods.changeInheritor(inheritor1New.publicKey).accountsStrict({
      maker: maker.publicKey,
      inheritance: inheritancePDA1,
      config,
      systemProgram: SystemProgram.programId
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Change inactivity time, first inheritance", async () => {

    const tx = await program.methods.changeInactivityTime(inactivityTime1New).accountsStrict({
      maker: maker.publicKey,
      inheritance: inheritancePDA1,
      config,
      systemProgram: SystemProgram.programId
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Increase inheritance bounty, first inheritance", async () => {

    const tx = await program.methods.increaseInheritanceBounty(bountyAmount1New).accountsStrict({
      maker: maker.publicKey,
      inheritanceVault: inheritanceVault1,
      inheritance: inheritancePDA1,
      config,
      systemProgram: SystemProgram.programId
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Decrease inheritance equal to current locked amount, first inheritance", async () => {

    try {
      
      await program.methods.reduceInheritance(inhertianceAmount1ToRemove).accountsStrict({
      maker: maker.publicKey,
      makerAta: makerAta1,
      protocolMint: protocolMint,
      config,
      vault,
      inheritance: inheritancePDA1,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to reduce inheritance by current locked amount, instruction for doing so is close inheritance.");

  });

  it("Increase inheritance, first inheritance", async () => {

    const tx = await program.methods.increaseInheritance(inheritanceAmount1ToAdd).accountsStrict({
      maker: maker.publicKey,
      makerAta: makerAta1,
      protocolMint: protocolMint,
      config,
      vault,
      inheritance: inheritancePDA1,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Decrease inheritance, first inheritance", async () => {

    const tx = await program.methods.reduceInheritance(inhertianceAmount1ToRemove).accountsStrict({
      maker: maker.publicKey,
      makerAta: makerAta1,
      protocolMint: protocolMint,
      config,
      vault,
      inheritance: inheritancePDA1,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Close first inheritance", async () => {

    const tx = await program.methods.closeInheritance().accountsStrict({
      maker: maker.publicKey,
      makerAta: makerAta1,
      protocolMint: protocolMint,
      config,
      vault,
      inheritance: inheritancePDA1,
      inheritanceVault: inheritanceVault1,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Initializing second inheritance with same maker and inheritor", async () => {

    const tx = await program.methods.initializeInheritance(seed2, inheritor1.publicKey, inheritanceAmount1, bountyAmount1, inactivityTime1)
    .accountsStrict({
      maker: maker.publicKey,
      config,
      vault,
      protocolMint,
      makerAta: makerAta1,
      inheritance: inheritancePDA2,
      inheritanceVault: inheritanceVault2,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);

  });

  it("Initializing first inheritance with second inheritor", async () => {

    const tx = await program.methods.initializeInheritance(seed2, inheritor2.publicKey, inheritanceAmount1, bountyAmount1, inactivityTime2)
    .accountsStrict({
      maker: maker.publicKey,
      config,
      vault,
      protocolMint,
      makerAta: makerAta1,
      inheritance: inheritancePDA3,
      inheritanceVault: inheritanceVault3,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);
    
  });

  it("Trying to check-in with an inheritance where the timer has run out", async () => {

    await new Promise((resolve) => setTimeout(resolve, 2000));

    try {
      
      await program.methods.checkIn().accountsStrict({
      maker: maker.publicKey,
      inheritance: inheritancePDA3,
      config,
      systemProgram: SystemProgram.programId
    }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to check-in for an inheritance where the timer has run out");

  });

  it("Trying to change inactivity time for an inheritance where the timer has run out", async () => {

    try {
      
      await program.methods.changeInactivityTime(inactivityTime1New).accountsStrict({
        maker: maker.publicKey,
        inheritance: inheritancePDA3,
        config,
        systemProgram: SystemProgram.programId
      }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to change inactivity time for an inheritance where the timer has run out");

  });

  it("Trying to change inheritor for an inheritance where the timer has run out", async () => {

    try {
      
      await program.methods.changeInheritor(inheritor1New.publicKey).accountsStrict({
        maker: maker.publicKey,
        inheritance: inheritancePDA3,
        config,
        systemProgram: SystemProgram.programId
      }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to change inheritor for an inheritance where the timer has run out");

  });

  it("Trying to increase inheritance bounty for an inheritance where the timer has run out", async () => {

    try {
      
      await program.methods.increaseInheritanceBounty(bountyAmount1New).accountsStrict({
        maker: maker.publicKey,
        inheritanceVault: inheritanceVault3,
        inheritance: inheritancePDA3,
        config,
        systemProgram: SystemProgram.programId
      }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to increase inheritance bounty for an inheritance where the timer has run out");

  });

  it("Trying to increase inheritance amount for an inheritance where the timer has run out", async () => {

    try {
      
      await program.methods.increaseInheritance(inheritanceAmount1ToAdd).accountsStrict({
        maker: maker.publicKey,
        makerAta: makerAta1,
        protocolMint: protocolMint,
        config,
        vault,
        inheritance: inheritancePDA3,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to increase inheritance amount for an inheritance where the timer has run out");

  });

  it("Trying to decrease inheritance amount for an inheritance where the timer has run out", async () => {

    try {
      
      await program.methods.reduceInheritance(inhertianceAmount2ToRemove).accountsStrict({
        maker: maker.publicKey,
        makerAta: makerAta1,
        protocolMint: protocolMint,
        config,
        vault,
        inheritance: inheritancePDA3,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to decrease inheritance amount for an inheritance where the timer has run out");

  });

  it("Trying to close an inheritance where the timer has run out", async () => {

    try {
      
      await program.methods.closeInheritance().accountsStrict({
        maker: maker.publicKey,
        makerAta: makerAta1,
        protocolMint: protocolMint,
        config,
        vault,
        inheritance: inheritancePDA3,
        inheritanceVault: inheritanceVault3,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      }).signers([maker]).rpc().then(confirmTx);

  } catch(err) {

    return;
  }

  throw new Error("Should not be able to close inheritance where the timer has run out");

  });

  it("Trigger valid inheritance", async () => {

    const tx = await program.methods.triggerInheritance().accountsStrict({

      keeper: keeper.publicKey,
      inheritor: inheritor2.publicKey,
      maker: maker.publicKey,
      makerAta: makerAta1,
      admin: admin.publicKey,
      protocolMint,
      vault,
      config,
      inheritance: inheritancePDA3,
      inheritanceVault: inheritanceVault3,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
      programData,
      thisProgram: program.programId

    }).signers([keeper]).rpc().then(confirmTx);

  });

  it("Reopening closed inheritance", async () => {

    //await new Promise((resolve) => setTimeout(resolve, 5000)); //added this delay to see if the account already exists error is due to trying to open account in the same slot as close.

    const tx = await program.methods.initializeInheritance(seed1, inheritor1.publicKey, inheritanceAmount1, bountyAmount1, inactivityTime1)
    .accountsStrict({
      maker: maker.publicKey,
      config,
      vault,
      protocolMint,
      makerAta: makerAta1,
      inheritance: inheritancePDA1,
      inheritanceVault: inheritanceVault1,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);
  });

  it("Trigger invalid inheritance", async () => {

    try {

      await program.methods.triggerInheritance().accountsStrict({

        keeper: keeper.publicKey,
        inheritor: inheritor1.publicKey,
        maker: maker.publicKey,
        makerAta: makerAta1,
        admin: admin.publicKey,
        protocolMint,
        vault,
        config,
        inheritance: inheritancePDA1,
        inheritanceVault: inheritanceVault1,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
        programData,
        thisProgram: program.programId
  
      }).signers([keeper]).rpc().then(confirmTx);


    } catch(err) {

      return;
    }

    throw new Error("Should not be able to trigger an inheritance where the timer has not run out");

  });

  it("Trying to open a new inheritance by maker for inheritor that was triggered previously", async () => {

    const tx = await program.methods.initializeInheritance(seed2, inheritor2.publicKey, inheritanceAmount1, bountyAmount1, inactivityTime1)
    .accountsStrict({
      maker: maker.publicKey,
      config,
      vault,
      protocolMint,
      makerAta: makerAta1,
      inheritance: inheritancePDA3,
      inheritanceVault: inheritanceVault3,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc().then(confirmTx);
  });

  it("Trying to check-in for an inheritance from a hacker", async () => {

    await new Promise((resolve) => setTimeout(resolve, 2000));

    try {

      await program.methods.checkIn().accountsStrict({
        maker: maker.publicKey,
        inheritance: inheritancePDA3,
        config,
        systemProgram: SystemProgram.programId
      }).signers([hacker]).rpc().then(confirmTx);

    } catch(err) {

      return;
    }

    throw new Error("Should not be able to check-in for someone else's inheritance");

  });

  it("Hacker trying to change inheritor", async () => {

    try {

      await program.methods.changeInheritor(hackerInheritor.publicKey).accountsStrict({
        maker: maker.publicKey,
        inheritance: inheritancePDA3,
        config,
        systemProgram: SystemProgram.programId
      }).signers([hacker]).rpc().then(confirmTx);

    } catch(err) {

      return;
    }

    throw new Error("Should not be able to change inheritor for someone else's inheritance");

  });

  it("Hacker trying to change inactivity time", async () => {

    try {

      await program.methods.changeInactivityTime(hackerInActivityTime).accountsStrict({
        maker: maker.publicKey,
        inheritance: inheritancePDA3,
        config,
        systemProgram: SystemProgram.programId
      }).signers([hacker]).rpc().then(confirmTx);

    } catch(err) {

      return;
    }

    throw new Error("Should not be able to change inactivity time for someone else's inheritance.");

  });

  it("Hacker trying to increase inheritance bounty", async () => {

    try {

      await program.methods.increaseInheritanceBounty(bountyAmount1New).accountsStrict({
        maker: maker.publicKey,
        inheritanceVault: inheritanceVault3,
        inheritance: inheritancePDA3,
        config,
        systemProgram: SystemProgram.programId
      }).signers([hacker]).rpc().then(confirmTx);

    } catch(err) {

      return;
    }

    throw new Error("Hacker should not be able to increase inheritance bounty");

  });

  it("Hacker trying to withdraw without any balance in wallet", async () => {

    try {

      await program.methods.withdrawSol(hackerWithdrawAmount).accountsStrict({

        withdrawer: hacker.publicKey,
        withdrawerAta: hackerAta,
        protocolMint,
        config,
        vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      }).signers([hacker]).rpc().then(confirmTx);


    } catch(err) {

      return;
    }

    throw new Error("Hacker should not be able to withdraw w/o any protocol token balance in their wallet");

  });

  it("Transfer 0.5 protocol mint tokens to withdrawer", async () => {

    const tx = await program.methods.initializeInheritance(finalSeed, finalInheritor.publicKey, finalInheritanceAmount, finalBountyAmount, finalInActicityTime)
    .accountsStrict({
      maker: maker.publicKey,
      config,
      vault,
      protocolMint,
      makerAta: makerAta1,
      inheritance: finalInheritancePDA,
      inheritanceVault: finalInheritanceVault,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID

    }).signers([maker]).rpc().then(confirmTx);

    const withdrawerAta = await getOrCreateAssociatedTokenAccount(anchor.getProvider().connection, maker, protocolMint, withdrawer.publicKey, false, undefined, undefined, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

    await transfer(anchor.getProvider().connection, maker, makerAta1, withdrawerAta.address, maker.publicKey, 2_000_000_000, [], undefined, TOKEN_2022_PROGRAM_ID);

  });

});