import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CounterProgram } from "../target/types/counter_program";

describe("counter-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
