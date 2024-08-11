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
    #[error("invalid asset kind")]
    InvalidAssetKind,
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
    REDBError(#[from] redb::Error),
    #[error(transparent)]
    REDBStorageError(#[from] redb::StorageError),
    #[error(transparent)]
    REDBDatabaseError(#[from] redb::DatabaseError),
    #[error(transparent)]
    REDBTransactionError(#[from] redb::TransactionError),
    #[error(transparent)]
    REDBTableError(#[from] redb::TableError),
    #[error(transparent)]
    REDBCompactionError(#[from] redb::CompactionError),
    #[error(transparent)]
    REDBCommitError(#[from] redb::CommitError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("error occurred inside parallel processing block")]
    ParallelError,
}

#[cfg(feature = "rocket")]
use rocket::{
    http::Status,
    response::{self, Responder},
    Request, Response,
};
#[cfg(feature = "rocket")]
use std::io::Cursor;
#[cfg(feature = "rocket")]
impl<'r> Responder<'r, 'static> for VCRError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let res = format!("{}", self);
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(res.len(), Cursor::new(res))
            .ok()
    }
}
