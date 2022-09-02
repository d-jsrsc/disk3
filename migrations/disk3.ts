import * as anchor from "@project-serum/anchor";
import {
  clusterApiUrl,
  PublicKey,
  Connection,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

import { readFile } from "fs/promises";

import { Disk3 } from "../target/types/disk3";
export { Disk3 } from "../target/types/disk3";

const idl = require("../target/idl/disk3.json");

const env = "devnet";
const connection = new Connection(clusterApiUrl(env));

export async function initProgram(payerPath: string) {
  const payerFileBuf = await readFile(payerPath, {});
  const payerBuffer = JSON.parse(payerFileBuf.toString());

  const payer = anchor.web3.Keypair.fromSecretKey(Buffer.from(payerBuffer));
  const walletWrapper = new anchor.Wallet(payer);
  const provider = new anchor.AnchorProvider(connection, walletWrapper, {
    preflightCommitment: "recent",
  });

  const disk3 = new anchor.Program(
    idl as any,
    new PublicKey(idl.metadata.address),
    provider
  ) as anchor.Program<Disk3>;
  if (env === "devnet") {
    // await connection.requestAirdrop(payer.publicKey, 2 * LAMPORTS_PER_SOL);
  }
  let payerInfo = await connection.getAccountInfo(payer.publicKey);
  console.log(
    "payerInfo",
    payer.publicKey.toString(),
    payerInfo.lamports / LAMPORTS_PER_SOL
  );

  return {
    payer,
    disk3,
  };
}
