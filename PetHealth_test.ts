import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PetHealth } from "../target/types/pet_health";

describe("pet_health", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.PetHealth as Program<PetHealth>;

  it("¡Registra una mascota exitosamente!", async () => {
    const [mascotaPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("expediente"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .registrarMascota("Sol", "Husky")
      .accounts({
        mascotaAccount: mascotaPDA,
        owner: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });
});
