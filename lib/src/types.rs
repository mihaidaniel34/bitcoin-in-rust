mod blockchain;
mod block;
mod transaction;

use crate::error::BtcError;
pub type Result<T> = std::result::Result<T, BtcError>;


pub use block::{Block, BlockHeader};
pub use blockchain::Blockchain;
pub use transaction::{
    Transaction, TransactionInput, TransactionOutput,
};