// Sólo es un import, no tiene nada que ver con anchor, sólo con Rust, el lenguaje
use anchor_lang::prelude::*;

// Program ID: Es necesario para vincular la Account con el Program, para hacerlo owner
// de la account, como he explicado en el artículo anteriormente (owner != authority)
// aparte de la productividad de trabajo, este tipo de cosas son de las que aportan valor 
// de Anchor, es decir, más seguridad, en una línea de código. Remarcar que cuando creamos un proyecto
// con anchor nos pondrá la misma PublicKey por defecto con usemos el comando build con su CLI, tendremos
// que cambiar esta Publickey por la que nos muestre por pantalla, además de la que hay
// en el archivo Anchor.toml del folder principal del proyecto
declare_id!("AkZZu7ipdyxhxt8NcZQt2WBRH6KtXqx5AWzb2c1LsTgs");

// CONSEJO: Para entender mejor el código mejor que leas de abajo a arriba.

//---------------------
// Lógica del programa: Las diferentes instrucciones del smart contract
//---------------------

#[program] // Etiqueta este módulo como un programa y que las funciones deben ser tratadas
        // como instrucciones, se podrían decir que son nuestras APIs
pub mod counter_program {
    use super::*;

    pub fn initialize( // Esta función recibe los siguientes argumentos:
        ctx: Context<Initialize>, // El Context Initialize declarado en el módulo de abajo
        start: u64 // Un número que reibimos como argumento introducido por el usuario por el que
                    // empezará el contador
    ) -> Result<()> { //Puro Rust, simplemente decimos que esta función devolverá un Ok() o un Error (si los declaramos)
        
        let counter = &mut ctx.accounts.counter; // Referencia mutable de la Account Counter que
                                                // que obtenemos a partir del Context que recibimos
                                                // como argumento

        counter.authority = ctx.accounts.authority.key(); // Establecemos la authority de la authority
        counter.count = start; // Establecemos la variable contador de la Account
        
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // En este módulo no hay que explicar mucho, simplemente incrementamos el contador de la Account
        let counter = &mut ctx.accounts.counter;
        
        counter.count += 1;
        
        Ok(())
    }
}

//---------------------------------------------
// Declaración del Context de cada instrucción: Cada instrucción del smart contract
//---------------------------------------------
//      debe tener su propio Context declarado, aquí definimos los diferentes constraints
//      o restricciones/verificaciones para hacer más seguro nuestro contrato. 

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account( // Este macro es lo que nos permite declarar constraints
        init, // Crea una nueva account, en este caso, counter
        payer = authority, // Determina que quien paga el rent es la authority/usuario
        space = 48, // Declaramos el espacio de la account (8 (SIEMPRE) + 32 (Pubkey) + 8 (u64))
        // Aquí utilizamos bytes, los 8 primeros son el discriminator
        // Una Pubkey = 32bytes
        // Por tanto 64bits(u64) = 8bytes 
    )]
    pub counter: Account<'info, Counter>, // Nuestra Account, que hemos declarado en el último módulo
    #[account(mut)] // mut: Declara que es mutable, el Signer siempre debe serlo cuando se crea una Account
    pub authority: Signer<'info>, // Declaramos quien es la autoridad de la Account y el que firma/aprueba la transacción
    pub system_program: Program<'info, System>, // Cuando creamos una Account es necesario que se declare el System program
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut, // Aparte de lo que he comentado antes, nos permite que el estado de la Account se actualice y persista ese cambio
        has_one = authority // has_one: Comprueba que el campo authority (en este caso) coincide que el authority que hemos 
                            //          almacenado en la Account es el mismo, es decir, que es la misma Pubkey la que quiere modificar
                            //          el estado de esta Account
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>
}

// Para más info sobre constraints y Accounts: https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html

//------------------------
// Declaración del Account: Simplemente definimos el almacenamiento de datos, aquel que
//------------------------
//      va a ser controlado por el program, aquí tenemos que definir los tipos de cada variable
//      IMPORTANTE: Si utilizas vectores/Strings (variables dinámicas) tienes que definir el 
//      tamaño "a mano", para eso te dejo este link para que te lo guardes en marcadores ;)
//      https://solanacookbook.com/references/anchor.html#calculating-account-space-size

#[account] // Esto es la macro para declarar una account, es la magia de Anchor
pub struct Counter {
    pub authority: Pubkey, // Aquí almacenamos la Pubkey del usuario que crea la Account del
                            // Counter y que tiene la autoridad para modificar los datos de la misma
    
    pub count: u64, // Esto es simplemente un número (de hasta 64bits), la variable contador
}