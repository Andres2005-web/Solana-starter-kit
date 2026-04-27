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
         // número automático PDA    
         )]
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


/*
# Explicación

Este proyecto consiste en una base de datos de videojuegos desarrollada con Anchor y Solana.

El programa permite realizar operaciones CRUD (Create, Read, Update y Delete) sobre una colección de videojuegos.

CRUD significa:

- Create → Crear datos
- Read → Leer datos
- Update → Actualizar datos
- Delete → Eliminar datos

El proyecto funciona mediante una cuenta PDA (Program Derived Address), que permite guardar información en blockchain de manera segura y única para cada usuario.



FUNCIONAMIENTO DEL PROGRAMA

1. Crear Database

La función crear_database() crea una base de datos para almacenar videojuegos.

Qué hace:

. uarda el owner (dueño de la database).
. Guarda el nombre de la base de datos.
. Inicializa un vector vacío de videojuegos.

Ejemplo:

crear_database("Mis Juegos")

Resultado:

Se crea una cuenta PDA que almacenará todos los videojuegos.


2. Agregar Videojuego

La función agregar_videojuego() permite agregar videojuegos al vector.

Datos que recibe:

- Nombre
- Género
- Estudio
- Dificultad
- Calificación

Ejemplo:

agregar_videojuego(
    "Halo",
    "Shooter",
    "Bungie",
    8,
    10
)

Resultado:

El videojuego se guarda dentro del vector de videojuegos.


3. Ver Videojuegos

La función ver_videojuegos() muestra todos los videojuegos almacenados.

Usa msg!() para imprimir datos dentro de los logs de Solana.

Ejemplo:

ver_videojuegos()

Resultado:

Se imprime la lista completa de videojuegos.


4. Eliminar Videojuego

La función eliminar_videojuego() busca un videojuego por nombre.

Si existe:

- Lo elimina del vector.
- Muestra mensaje de éxito.

Ejemplo:

eliminar_videojuego("Halo")

Resultado:

El videojuego desaparece de la database.


5. Cambiar Estado

La función alternar_estado() cambia el estado de disponibilidad.

Ejemplo:

true → false
false → true

Esto permite marcar si un videojuego está disponible.


ESTRUCTURAS DEL PROGRAMA

1. VideojuegoDB

Representa la cuenta principal.

Contiene:

. Owner
. Nombre de la database
. Lista de videojuegos

2. Videojuego

Representa cada videojuego individual.

Contiene:

- Nombre
- Género
- Estudio
- Dificultad
- Calificación
- Disponible


SEGURIDAD

El programa usa require!() para validar permisos.

Esto asegura que solamente el dueño pueda modificar la database.

Ejemplo:

require!(
    context.accounts.videojuego_db.owner == context.accounts.usuario.key(),
    ErrorCode::NoEresOwner
);



USO DE PDA

La PDA se crea con:

seeds = [b"database", usuario.key().as_ref()]

Esto genera una dirección única para cada usuario.

Beneficios:

. Seguridad
. No requiere private key
.  Cada usuario tiene su propia database



CONCLUSIÓN

Este proyecto implementa un CRUD completo usando Anchor y Solana.

Permite administrar videojuegos como una biblioteca digital dentro de blockchain.

El programa demuestra:

- Uso de PDA
- Uso de cuentas Anchor
- Validación de permisos
- Uso de vectores
- Persistencia de datos en Solana

Esto convierte el proyecto en una base de datos descentralizada de videojuegos.
*/
