use anchor_lang::prelude::*;

declare_id!("F4gRcYvLs2MBCsJ1yGaokFicYBNj59W1KiuDLXuwLu3L");

#[program]
pub mod modulo {
    use super::*;

    // CREAR DATABASE
    pub fn crear_database(
        ctx: Context<CrearVideoJuegoDB>,
        nombre_db: String,
    ) -> Result<()> {
        let db = &mut ctx.accounts.videojuego_db;

        require!(nombre_db.len() <= 30, ErrorCode::NombreMuyLargo);

        db.owner = ctx.accounts.usuario.key();
        db.nombre_db = nombre_db;
        db.juegos = Vec::new();

        Ok(())
    }

    // ==========================
    // CREATE -> AGREGAR JUEGO
    // ==========================
    pub fn agregar_videojuego(
        ctx: Context<CrearJuego>,
        juego_nombre: String,
        genero: String,
        estudio: String,
        dificultad: u8,
        calificacion: u8,
    ) -> Result<()> {

        let videojuego_db = &mut ctx.accounts.videojuego_db;
        let juego = &mut ctx.accounts.juego;

        require!(juego_nombre.len() <= 30, ErrorCode::NombreMuyLargo);
        require!(genero.len() <= 30, ErrorCode::NombreMuyLargo);
        require!(estudio.len() <= 30, ErrorCode::NombreMuyLargo);

        juego.nombre = juego_nombre;
        juego.genero = genero;
        juego.estudio = estudio;
        juego.dificultad = dificultad;
        juego.calificacion = calificacion;

        videojuego_db.juegos.push(juego.key());

        Ok(())
    }

    // ==========================
    // UPDATE -> ACTUALIZAR JUEGO
    // ==========================
    pub fn actualizar_videojuego(
        ctx: Context<ActualizarJuego>,
        nuevo_nombre: String,
        nuevo_genero: String,
        nuevo_estudio: String,
        nueva_dificultad: u8,
        nueva_calificacion: u8,
    ) -> Result<()> {

        let juego = &mut ctx.accounts.juego;

        require!(nuevo_nombre.len() <= 30, ErrorCode::NombreMuyLargo);
        require!(nuevo_genero.len() <= 30, ErrorCode::NombreMuyLargo);
        require!(nuevo_estudio.len() <= 30, ErrorCode::NombreMuyLargo);

        juego.nombre = nuevo_nombre;
        juego.genero = nuevo_genero;
        juego.estudio = nuevo_estudio;
        juego.dificultad = nueva_dificultad;
        juego.calificacion = nueva_calificacion;

        Ok(())
    }

    // ==========================
    // DELETE -> ELIMINAR JUEGO
    // ==========================
    pub fn eliminar_videojuego(
        ctx: Context<EliminarJuego>,
    ) -> Result<()> {

        let videojuego_db = &mut ctx.accounts.videojuego_db;
        let juego_key = ctx.accounts.juego.key();

        // elimina la pubkey del vector
        videojuego_db.juegos.retain(|&x| x != juego_key);

        Ok(())
    }
}

// ==========================
// ERRORES
// ==========================
#[error_code]
pub enum ErrorCode {
    #[msg("El nombre es demasiado largo.")]
    NombreMuyLargo,
}

// ==========================
// DATABASE
// ==========================
#[account]
#[derive(InitSpace)]
pub struct VideojuegoDB {
    pub owner: Pubkey,

    #[max_len(30)]
    pub nombre_db: String,

    #[max_len(50)]
    pub juegos: Vec<Pubkey>,
}

// ==========================
// JUEGO
// ==========================
#[account]
#[derive(InitSpace)]
pub struct Juego {
    #[max_len(30)]
    pub nombre: String,

    #[max_len(30)]
    pub genero: String,

    #[max_len(30)]
    pub estudio: String,

    pub dificultad: u8,
    pub calificacion: u8,
}

// ==========================
// CREAR DATABASE
// ==========================
#[derive(Accounts)]
pub struct CrearVideoJuegoDB<'info> {
    #[account(
        init,
        payer = usuario,
        space = 8 + VideojuegoDB::INIT_SPACE,
        seeds = [b"database", usuario.key().as_ref()],
        bump
    )]
    pub videojuego_db: Account<'info, VideojuegoDB>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ==========================
// CREAR JUEGO
// ==========================
#[derive(Accounts)]
#[instruction(juego_nombre:String)]
pub struct CrearJuego<'info> {

    #[account(mut)]
    pub videojuego_db: Account<'info, VideojuegoDB>,

    #[account(
        init,
        payer = usuario,
        space = 8 + Juego::INIT_SPACE,
        seeds = [b"juego", usuario.key().as_ref(), juego_nombre.as_bytes()],
        bump
    )]
    pub juego: Account<'info, Juego>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ==========================
// ACTUALIZAR JUEGO
// ==========================
#[derive(Accounts)]
pub struct ActualizarJuego<'info> {

    #[account(mut)]
    pub videojuego_db: Account<'info, VideojuegoDB>,

    #[account(mut)]
    pub juego: Account<'info, Juego>,

    pub usuario: Signer<'info>,
}

// ==========================
// ELIMINAR JUEGO
// ==========================
#[derive(Accounts)]
pub struct EliminarJuego<'info> {

    #[account(mut)]
    pub videojuego_db: Account<'info, VideojuegoDB>,

    #[account(
        mut,
        close = usuario
    )]
    pub juego: Account<'info, Juego>,

    #[account(mut)]
    pub usuario: Signer<'info>,
}
