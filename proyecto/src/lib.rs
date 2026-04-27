use anchor_lang::prelude::*; // usamos Anchor para Solana

declare_id!("F4gRcYvLs2MBCsJ1yGaokFicYBNj59W1KiuDLXuwLu3L");

#[program]
pub mod modulo {
    use super::*;

    // ==========================
    // CREAR DATABASE
    // ==========================
    pub fn crear_database(
        context: Context<CrearVideoJuegoDB>,
        nombre_db: String,
    ) -> Result<()> {

        let owner = context.accounts.usuario.key(); // dueño de la DB
        let juegos: Vec<Videojuego> = Vec::new(); // vector vacío

        context.accounts.videojuego_db.set_inner(VideojuegoDB {
            owner,
            nombre_db,
            juegos,
        });

        msg!("Database creada correctamente");

        Ok(())
    }

    // ==========================
    // CREATE -> AGREGAR VIDEOJUEGO
    // ==========================
    pub fn agregar_videojuego(
        context: Context<NuevoVideojuego>,
        nombre: String,
        genero: String,
        estudio: String,
        dificultad: u8,
        calificacion: u8,
    ) -> Result<()> {

        // validamos owner
        require!(
            context.accounts.videojuego_db.owner == context.accounts.usuario.key(),
            ErrorCode::NoEresOwner
        );

        let videojuego = Videojuego {
            nombre,
            genero,
            estudio,
            dificultad,
            calificacion,
            disponible: true,
        };

        // agregamos al vector
        context.accounts.videojuego_db.juegos.push(videojuego);

        msg!("Videojuego agregado correctamente");

        Ok(())
    }

    // ==========================
    // READ -> VER VIDEOJUEGOS
    // ==========================
    pub fn ver_videojuegos(
        context: Context<NuevoVideojuego>,
    ) -> Result<()> {

        require!(
            context.accounts.videojuego_db.owner == context.accounts.usuario.key(),
            ErrorCode::NoEresOwner
        );

        msg!(
            "Lista de videojuegos: {:#?}",
            context.accounts.videojuego_db.juegos
        );

        Ok(())
    }

    // ==========================
    // DELETE -> ELIMINAR VIDEOJUEGO
    // ==========================
    pub fn eliminar_videojuego(
        context: Context<NuevoVideojuego>,
        nombre: String,
    ) -> Result<()> {

        require!(
            context.accounts.videojuego_db.owner == context.accounts.usuario.key(),
            ErrorCode::NoEresOwner
        );

        let juegos = &mut context.accounts.videojuego_db.juegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                juegos.remove(i);

                msg!("Videojuego {} eliminado", nombre);

                return Ok(());
            }
        }

        Err(ErrorCode::JuegoNoExiste.into())
    }

    // ==========================
    // UPDATE -> MODIFICAR DISPONIBILIDAD
    // ==========================
    pub fn alternar_estado(
        context: Context<NuevoVideojuego>,
        nombre: String,
    ) -> Result<()> {

        require!(
            context.accounts.videojuego_db.owner == context.accounts.usuario.key(),
            ErrorCode::NoEresOwner
        );

        let juegos = &mut context.accounts.videojuego_db.juegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {

                let estado_actual = juegos[i].disponible;
                let nuevo_estado = !estado_actual;

                juegos[i].disponible = nuevo_estado;

                msg!(
                    "El videojuego {} ahora esta {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(ErrorCode::JuegoNoExiste.into())
    }
}

// ==========================
// ERRORES
// ==========================
#[error_code]
pub enum ErrorCode {
    #[msg("No eres dueño de esta database")]
    NoEresOwner,

    #[msg("Nombre demasiado largo")]
    NombreMuyLargo,

    #[msg("El videojuego no existe")]
    JuegoNoExiste,
}

// ==========================
// CUENTA DATABASE
// ==========================
#[account]
#[derive(InitSpace)]
pub struct VideojuegoDB {

    pub owner: Pubkey, // dueño

    #[max_len(30)]
    pub nombre_db: String, // nombre DB

    #[max_len(50)]
    pub juegos: Vec<Videojuego>, // vector de videojuegos
}

// ==========================
// STRUCT VIDEOJUEGO
// ==========================
#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Videojuego {

    #[max_len(30)]
    pub nombre: String,

    #[max_len(30)]
    pub genero: String,

    #[max_len(30)]
    pub estudio: String,

    pub dificultad: u8,

    pub calificacion: u8,

    pub disponible: bool,
}

// ==========================
// CREAR DATABASE
// ==========================
#[derive(Accounts)]
pub struct CrearVideoJuegoDB<'info> {

    #[account(mut)]
    pub usuario: Signer<'info>,

    #[account(
        init,
        // crea cuenta nueva
        payer = usuario,
        // usuario paga renta
        space = VideojuegoDB::INIT_SPACE + 8,
        // espacio reservado
        seeds = [b"database", usuario.key().as_ref()],
        // crea PDA
        bump
         // número automático PDA    )]
    pub videojuego_db: Account<'info, VideojuegoDB>,
    // cuenta database

    pub system_program: Program<'info, System>,
    // programa sistema Solana
}

// ==========================
// CONTEXTO VIDEOJUEGO
// ==========================
#[derive(Accounts)]
pub struct NuevoVideojuego<'info> {

    pub usuario: Signer<'info>,

    #[account(mut)]
    pub videojuego_db: Account<'info, VideojuegoDB>,
}
