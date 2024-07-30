contract;

use std::block::timestamp;

abi MyContract {
    fn get_timestamp() -> u64;
}

impl MyContract for Contract {
    fn get_timestamp() -> u64 {
       timestamp()
    }
}
