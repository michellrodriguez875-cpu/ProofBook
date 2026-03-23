use anchor_lang::prelude::*;

declare_id!("5PH8j9pXu5bLmmaSj5JMB6mHz3GYixJ7YxhryFhvfXfo");

#[program]
pub mod proofbook {
    use super::*;

    // Crear perfil de usuario
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.user = *ctx.accounts.user.key;
        profile.reputation = 0;
        Ok(())
    }

    // Registrar libro
    pub fn add_book(
        ctx: Context<AddBook>,
        title: String,
        author: String,
        year: u16,
    ) -> Result<()> {
        let book = &mut ctx.accounts.book;
        let profile = &mut ctx.accounts.profile;

        book.owner = *ctx.accounts.user.key;
        book.title = title;
        book.author = author;
        book.year = year;

        // Recompensa reputación
        profile.reputation += 5;

        Ok(())
    }

    // Agregar reseña
    pub fn add_review(
        ctx: Context<AddReview>,
        rating: u8,
        comment: String,
    ) -> Result<()> {

        require!(rating <= 5, ErrorCode::InvalidRating);

        let review = &mut ctx.accounts.review;
        let profile = &mut ctx.accounts.profile;

        review.reviewer = *ctx.accounts.user.key;
        review.book = ctx.accounts.book.key();
        review.rating = rating;
        review.comment = comment;

        // Recompensa reputación
        profile.reputation += 1;

        Ok(())
    }

    // Eliminar libro
    pub fn delete_book(_ctx: Context<DeleteBook>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct BookAccount {
    pub owner: Pubkey,
    pub title: String,
    pub author: String,
    pub year: u16,
}

#[account]
pub struct ReviewAccount {
    pub reviewer: Pubkey,
    pub book: Pubkey,
    pub rating: u8,
    pub comment: String,
}

#[account]
pub struct UserProfile {
    pub user: Pubkey,
    pub reputation: u64,
}

// -------- CONTEXTOS --------

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddBook<'info> {

    #[account(init, payer = user, space = 8 + 32 + 4 + 50 + 4 + 50 + 2)]
    pub book: Account<'info, BookAccount>,

    #[account(mut)]
    pub profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddReview<'info> {

    #[account(init, payer = user, space = 8 + 32 + 32 + 1 + 4 + 100)]
    pub review: Account<'info, ReviewAccount>,

    #[account(mut)]
    pub book: Account<'info, BookAccount>,

    #[account(mut)]
    pub profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteBook<'info> {

    #[account(mut, close = owner)]
    pub book: Account<'info, BookAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

// -------- ERRORES --------

#[error_code]
pub enum ErrorCode {
    #[msg("Rating must be between 0 and 5")]
    InvalidRating,
}
