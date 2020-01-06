use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ErrorKind {
    #[error("mongodb error")]
    Mongodb(#[from] mongodb::error::Error),
}

//impl From<mongodb::error::Error> for ErrorKind {
//    fn from(err: mongodb::error::Error) -> ErrorKind {
//        ErrorKind::Mongodb(err)
//    }
//}

impl ResponseError for ErrorKind {}
