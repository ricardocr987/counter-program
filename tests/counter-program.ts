// Mocha test

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CounterProgram } from "../target/types/counter_program";
import BN from "bn.js"
const { SystemProgram } = anchor.web3;
const assert = require("assert");

describe("counter-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  
  // Anchor lee nuestro workspace local, busca el idl y lo utiliza para crear un client
  // Esta variable es una representación de nuestro smart contract en un test mocha
  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;
  
  // Creamos una Keypair (public+private), va a ser la dirección de la Account que almacena los datos
  const counter_account = anchor.web3.Keypair.generate();

  // Primer test, inicialización/creación de la account
  it("Creates a counter!", async () => {
    const tx = await program.rpc.initialize( // con esta línea invocamos la función initialize del Smart Contract
      // program = client | rpc = con Anchor llamamos namespaces dentro del client, te permite firmar una transacción
      // y la envía al cluster, en estos namespaces tenemos las funciones del smart contract
      new BN(0), // primer argumento de la función, un BN es una clase numérica para la aritmética de números grandes
                // básicamente si la función espera un u64 (como este caso), tienes enviarlo a la función de esta forma
                // si la función pidiese un número que ocupara menos bytes (< 64) simplemente sería pasarle un int normal
      {
      accounts: { // Aquí declaramos el Context de la función ("segunda parte" de nuestro Smart Contract), que tmb es un argumento de la función
        counter: counter_account.publicKey,
        authority: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId, 
      },
      signers: [counter_account], // En el Solana runtime firmas las transacciones con todas las Account que creas
    });
    const account = await program.account.counter.fetch(counter_account.publicKey); // Buscamos la account en la "blockchain"
    console.log(tx) // Mostramos por pantalla la transaction en la que creamos la Account
    console.log(account) // Mostramos por consola la Account con la información que almacena
    assert.ok(account.count.toString() == "0"); // Comprobamos que el contador de la Account = 0
  });

  // Segundo test, comprueba que la función increment funciona, no nada nuevo que comentar aquí
  it("Increments the counter!", async () => {
    const tx = await program.rpc.increment({
      accounts: {
        counter: counter_account.publicKey,
        authority: program.provider.wallet.publicKey,
      },
    });
    const account = await program.account.counter.fetch(counter_account.publicKey);
    console.log(tx)
    console.log(account)
    assert.ok(account.count.toString() == "1");
  });
});
