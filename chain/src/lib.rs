extern crate bitcrypto as crypto;
extern crate heapsize;
extern crate primitives;
extern crate rustc_hex as hex;
extern crate serialization as ser;
#[macro_use]
extern crate serialization_derive;

pub mod constants;

mod block;
mod block_header;
mod join_split;
mod merkle_root;
mod sapling;
mod solution;
mod transaction;

mod indexed_block;
mod indexed_header;
mod indexed_transaction;
/// `IndexedBlock` extension
mod read_and_hash;

pub use primitives::{bigint, bytes, compact, hash};

pub use crate::transaction::{
    BTC_TX_VERSION, OVERWINTER_TX_VERSION, SAPLING_TX_VERSION, SPROUT_TX_VERSION,
};
pub use crate::transaction::{OVERWINTER_TX_VERSION_GROUP_ID, SAPLING_TX_VERSION_GROUP_ID};

pub use crate::block::Block;
pub use crate::block_header::BlockHeader;
pub use crate::join_split::{JoinSplit, JoinSplitDescription, JoinSplitProof};
pub use crate::merkle_root::{merkle_node_hash, merkle_root};
pub use crate::sapling::{Sapling, SaplingOutputDescription, SaplingSpendDescription};
pub use crate::solution::EquihashSolution;
pub use crate::transaction::{OutPoint, Transaction, TransactionInput, TransactionOutput};

pub use crate::indexed_block::IndexedBlock;
pub use crate::indexed_header::IndexedBlockHeader;
pub use crate::indexed_transaction::IndexedTransaction;
pub use crate::read_and_hash::{HashedData, ReadAndHash};

pub type ShortTransactionID = hash::H48;
