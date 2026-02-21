import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PermissionlessOnChainLiquidInheritance } from "../target/types/permissionless_on_chain_liquid_inheritance";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction,
  createMint, getAssociatedTokenAddressSync, mintTo, TOKEN_PROGRAM_ID, getAccount } from "@solana/spl-token";
import { expect } from "chai";

describe("permissionless-on-chain-liquid-inheritance", () => {

});