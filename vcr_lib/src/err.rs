use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VCRError {
    #[error("entity not found")]
    EntityNotFound,
    #[error("entity type not found")]
    EntityTypeNotFound,
    #[error("patch data invalid")]
    InvalidPatchData,
    #[error("couldn't resolve json path")]
    PathResolutionError,
    #[error("invalid page token")]
    InvalidPageToken,
    #[error("invalid op code in patch bytecode")]
    InvalidOpCode,
    #[error("data not indexed during tapes build")]
    IndexMissing,
    #[error(transparent)]
    MsgPackEncError(#[from] rmp_serde::encode::Error),
    #[error(transparent)]
    MsgPackDecError(#[from] rmp_serde::decode::Error),
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    SerdeJSONError(#[from] serde_json::Error),
    #[error(transparent)]
    UTF8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("error occurred inside parallel processing block")]
    ParallelError,
    #[error("missing data required from streamdata construction")]
    MissingRequiredStreamData,
}

use rocket::{
    http::Status,
    response::{self, Responder},
    Request, Response,
};
use std::io::Cursor;
impl<'r> Responder<'r, 'static> for VCRError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let res = format!("{}", self);
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(res.len(), Cursor::new(res))
            .ok()
    }
}
