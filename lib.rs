use anchor_lang::prelude::*;

declare_id!("3VMwg6tqwHcVxeFFcXF35ymgTMjkwxvwkuWcabjCDGnf");

#[program]
pub mod pet_health_vault {
    use super::*;

    //Aqui se crea el expediente de la mascota
    pub fn registrar_mascota(
        ctx: Context<RegistrarMascota>,
        nombre: String,
        raza: String,
    ) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota_account;
        mascota.owner = ctx.accounts.owner.key();
        mascota.nombre = nombre;
        mascota.raza = raza;
        mascota.historial = Vec::new(); //Inicia sin consultas
        msg!(
            "Expediente creado para: {} (Propietario: {})",
            mascota.nombre,
            mascota.owner
        );
        Ok(())
    }

    //Se agrega una nueva consulta médica
    pub fn agregar_consulta(
        ctx: Context<GestionarExpediente>,
        fecha: String,
        diagnostico: String,
        costo: u64,
    ) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota_account;

        //Solo el dueño que creó el expediente puede agregar datos
        require!(
            mascota.owner == ctx.accounts.owner.key(),
            ErrorMascota::NoAutorizado
        );

        let nueva_entrada = EntradaMedica {
            fecha,
            diagnostico,
            costo,
        };

        mascota.historial.push(nueva_entrada);
        msg!("Nueva consulta registrada para {}", mascota.nombre);
        Ok(())
    }

    //Borrar expediente (para liberar espacio/SOL si es necesario)
    pub fn cerrar_expediente(_ctx: Context<GestionarExpediente>) -> Result<()> {
        msg!("Expediente cerrado y datos liberados.");
        Ok(())
    }
}

#[account]
#[derive(InitSpace)] //Calcula el tamaño de la cuenta automáticamente
pub struct Mascota {
    pub owner: Pubkey,
    #[max_len(30)]
    pub nombre: String,
    #[max_len(30)]
    pub raza: String,
    #[max_len(10)] //Guardamos hasta 10 consultas para no saturar la cuenta
    pub historial: Vec<EntradaMedica>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct EntradaMedica {
    #[max_len(12)] // Ejemplo: "2024-10-25"
    pub fecha: String,
    #[max_len(50)]
    pub diagnostico: String,
    pub costo: u64,
}

#[derive(Accounts)]
pub struct RegistrarMascota<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Mascota::INIT_SPACE, // 8 bytes de discriminador + espacio de datos
        seeds = [b"expediente", owner.key().as_ref()], // PDA única por dueño
        bump
    )]
    pub mascota_account: Account<'info, Mascota>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarExpediente<'info> {
    #[account(
        mut, 
        seeds = [b"expediente", owner.key().as_ref()], 
        bump,
        close = owner //Opción para borrar la cuenta y recuperar el dinero (SOL)
    )]
    pub mascota_account: Account<'info, Mascota>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorMascota {
    #[msg("No tienes permiso para modificar este expediente médico.")]
    NoAutorizado,
}
