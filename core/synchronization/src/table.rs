use core_storage::{relational::table::BsonBytes, single_sql_return};
use db_xsql::rbatis::crud_table;

use serde::{Deserialize, Serialize};

single_sql_return!(ScriptHash, script_hash, BsonBytes);

#[crud_table(table_name: "mercury_sync_status")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncStatus {
    pub block_number: u64,
}

impl SyncStatus {
    pub fn new(block_number: u64) -> SyncStatus {
        SyncStatus { block_number }
    }
}