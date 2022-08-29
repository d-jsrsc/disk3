import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import base58 from "bs58";
import { Disk3 } from "../target/types/disk3";

describe("disk3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Disk3 as Program<Disk3>;
  const connection = anchor.getProvider().connection;
  const sectionOwner = anchor.web3.Keypair.generate();
  const appreciator = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const initAmount = 10 * anchor.web3.LAMPORTS_PER_SOL;
  const commitment: anchor.web3.Commitment = "processed";

  before(async () => {
    await connection.confirmTransaction(
      await connection.requestAirdrop(payer.publicKey, initAmount),
      commitment
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(appreciator.publicKey, initAmount),
      commitment
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(sectionOwner.publicKey, initAmount),
      commitment
    );
  });

  // const filter = [
  //   {
  //     memcmp: {
  //       offset: 8 + 1,
  //       bytes: owner.toBase58(),
  //     },
  //   },
  //   {
  //     memcmp: {
  //       offset: 8 + 1,
  //       bytes: owner.toBase58(),
  //     },
  //   }
  // ];

  // const blogs = await this.program.account.blog.all(filter);

  it("Is initialized!", async () => {
    // Add your test here.
    const arweaveKey = "QeSUFwff9xDbl4SCXlOmEn0TuS4vPg11r2_ETPPu_nk";
    const parent = "file";
    const [filePda, sectionBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("disk3-file"),
          Buffer.from("fileMd5"),
          Buffer.from(parent),
          payer.publicKey.toBuffer(),
        ],
        program.programId
      );
    const tx = await program.methods
      .initFile("fileMd5", parent, arweaveKey)
      .accounts({
        file: filePda,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([payer])
      .rpc();
    console.log("Your transaction signature", tx);
    // const fileAccountInfo = await connection.getAccountInfo(filePda);
    // console.log(fileAccountInfo.data.toJSON());
    const pubKeyBuf = payer.publicKey.toBuffer();

    const parentBuf = Buffer.from(parent);

    const strBuf = Buffer.alloc(4);
    strBuf.writeUInt32LE(parentBuf.length);

    const bs58Encode = base58.encode(
      Buffer.concat(
        [pubKeyBuf, strBuf, parentBuf],
        pubKeyBuf.length + 4 + parentBuf.length
      )
    );
    console.log(bs58Encode, pubKeyBuf.length, 8);
    const filter = [
      {
        memcmp: {
          offset: 8 + 1,
          bytes: bs58Encode,
        },
      },
    ];

    const files = await program.account.file.all(filter);
    console.log(files);
  });
});
