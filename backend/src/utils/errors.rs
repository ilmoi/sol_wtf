use actix_web::ResponseError;

// preserve sqlx error where possible, otherwise show anyhow error
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)] // this implements Display
    SqlxError(#[from] sqlx::error::Error),
    #[error(transparent)] // this implements Display
    UnexpectedError(#[from] anyhow::Error),
}

// it says Display not implemented, but actually it is because we're deriving Display from thiserror
impl ResponseError for ApiError {}

// ----------------------------------------------------------------------------- saved for my ref - manual display/debug impl

// impl std::fmt::Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "<<< This error is coming from Display impl")
//     }
// }

// impl std::fmt::Debug for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         error_chain_fmt(self, f)
//     }
// }

// fn error_chain_fmt(
//     e: &impl std::error::Error,
//     f: &mut std::fmt::Formatter<'_>,
// ) -> std::fmt::Result {
//     writeln!(f, "{}\n", e)?;
//     let mut current = e.source();
//     while let Some(cause) = current {
//         writeln!(f, "Caused by: \n\t{}", cause)?;
//         current = cause.source();
//     }
//     Ok(())
// }
