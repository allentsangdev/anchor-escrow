import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Project } from "../target/types/project";

describe("SOL Escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Project as Program<Project>;

  it("Init SOL Escrow", async () => {
    // Add your test here.
    
  });
});
