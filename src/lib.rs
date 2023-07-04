mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::{Hex, store};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use substreams::store::StoreSetProto;
use substreams::store::StoreNew;
use substreams_entity_change::tables::Tables;

use substreams_entity_change::pb::entity::EntityChanges;

use sha1::{Sha1, Digest};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use substreams::store::{DeltaProto, StoreSet};
use crate::pb::contract::v1::{BlockInfo, BlockInfos, HashedBlockInfo};

const TRACKED_CONTRACT: [u8; 20] = hex!("bc4ca0eda7647a8ab7c2061c2e118a18a936f13d");

substreams_ethereum::init!();

#[substreams::handlers::store]
fn store_info(blk: eth::Block, s: substreams::store::StoreSetProto<contract::BlockInfo>) {
    let header = blk.header.unwrap();

    let info: contract::BlockInfo = contract::BlockInfo {
        block_number: blk.number,
        block_hash: Hex(&blk.hash).to_string(),
        parent_hash: Hex(&header.parent_hash).to_string(),
    };

    s.set(info.block_number, format!("block:{}", info.block_number), &info);
}

#[substreams::handlers::store]
fn store_hashed_block(info_deltas: store::Deltas<DeltaProto<contract::BlockInfo>>, s: substreams::store::StoreSetProto<contract::HashedBlockInfo>) {
    for d in info_deltas.deltas {
        s.set(d.ordinal, format!("hashed_block:{}", d.key), &contract::HashedBlockInfo {
            block_number: d.new_value.block_number,
            hash: calculate_sha1_hash(d.new_value.block_hash),
        });
    }
}

#[substreams::handlers::map]
fn graph_out(deltas: store::Deltas<DeltaProto<HashedBlockInfo>>) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    hashed_block_to_entities_changes(&mut tables, deltas);

    Ok(tables.to_entity_changes())
}

fn calculate_sha1_hash(s: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    let sha1_hash = format!("{:x}", result);
    sha1_hash
}

pub fn hashed_block_to_entities_changes(
    tables: &mut Tables,
    deltas: store::Deltas<DeltaProto<HashedBlockInfo>>,
) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.deltas {
        match delta.operation {
            Operation::Create => push_create(
                tables,
                &delta.key,
                delta.new_value,
            ),
            Operation::Update => push_update(tables, &delta.key, delta.new_value),
            x => panic!("unsupported opeation {:?}", x),
        }
    }
}

fn push_create(tables: &mut Tables, key: &str, value: HashedBlockInfo) {
    tables
        .create_row("HashedBlock", key)
        .set("block_number", value.block_number)
        .set("block_hash", value.hash);
}

fn push_update(tables: &mut Tables, key: &str, value: HashedBlockInfo) {
    tables
        .update_row("HashedBlock", key)
        .set("block_number", value.block_number)
        .set("block_hash", value.hash);
}