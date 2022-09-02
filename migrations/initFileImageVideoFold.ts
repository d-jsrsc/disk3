import * as anchor from "@project-serum/anchor";
import { clusterApiUrl, Connection } from "@solana/web3.js";

import path from "path";
import os from "os";

import { Disk3, initProgram } from "./disk3";

const env = "devnet";
const connection = new Connection(clusterApiUrl(env));
const ROOT_FOLD_SEED = "disk3-root-fold";

// main
(async () => {
  const payerPath = path.resolve(
    os.homedir(),
    ".config/solana/gchtwgXQHUh6bpipMHEvpVJEyexsk7netBBkcgiZGP8.json"
  );
  const { payer, disk3 } = await initProgram(payerPath);
  await initFolds(disk3, payer);
})();

async function initFolds(
  program: anchor.Program<Disk3>,
  payer: anchor.web3.Keypair
) {
  await initFold("file", program, payer);
  await initFold("image", program, payer);
  await initFold("video", program, payer);
}

async function initFold(
  name: string,
  program: anchor.Program<Disk3>,
  payer: anchor.web3.Keypair
) {
  const [foldPda] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(ROOT_FOLD_SEED), Buffer.from(name)],
    program.programId
  );

  await program.methods
    .initRootFold(name)
    .accounts({
      fold: foldPda,
      payer: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([payer])
    .rpc();
  console.log(name, foldPda.toBase58());
}

/**
❯ ts-node migrations/initFileImageVideoFold.ts  
payerInfo gchtwgXQHUh6bpipMHEvpVJEyexsk7netBBkcgiZGP8 7.705339415
file 3CD5PpdZQUGmDw33MVFh7FriVdhXhKFRFGjPc41CRLRP
image GGktv2GXdXm2ixmZGmwaYagXSXjzU1JQ2y9w1P1D5Be2
video BcM6ZCGeRjoc9youESuGnYbpj9PqmKa49eDYimdNtnE3
 */
