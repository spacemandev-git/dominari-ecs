import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Dominarisystems } from "../target/types/dominarisystems";
import * as byteify from "byteify";

describe("ecs", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Dominarisystems as Program<Dominarisystems>;

  it("Is initialized!", async () => {
    // Add your test here.
    let instance = 1;
    let systemSigner = findProgramAddressSync([Buffer.from("System_Signer")], program.programId)[0];
    let universe = new anchor.web3.PublicKey("GN5Ww5qa8ej4evFCJxMhV6AFEPKhD1Drdu8qYYptVgDJ");
    let world = new anchor.web3.PublicKey("H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3");
    let worldConfig = findProgramAddressSync([Buffer.from("world_signer")], world)[0];
    let worldInstance = findProgramAddressSync([
      Buffer.from("World"),
      world.toBuffer(),
      byteify.serializeUint64(instance)      
    ], universe)[0];
    let systemRegistration = findProgramAddressSync([
      Buffer.from("System_Registration"),
      worldInstance.toBuffer(),
      program.programId.toBuffer()
    ], world)[0];

    let entity_id = new anchor.BN(5);
    let mapEntity = findProgramAddressSync([
      Buffer.from("Entity"),
      byteify.serializeUint64(entity_id.toNumber()),
      worldInstance.toBuffer()
    ], universe)[0];

    let instanceIndex = findProgramAddressSync([
      Buffer.from("Instance_Index"),
      worldInstance.toBuffer()
    ], program.programId)[0];


    const tx = await program.methods
      .systemInitalizeMap(
        entity_id,
        8,
        8
      )
      .accounts({
        payer: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        systemSigner,
        worldConfig,
        worldProgram: world,
        universe,
        systemRegistration,
        worldInstance,
        mapEntity,
        instanceIndex
      })
      .rpc({skipPreflight: true});
    console.log("Tx: ", tx);
  });
});
