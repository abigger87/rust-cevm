pub mod shared;
pub mod EVM;
pub mod server;

use EVM::*;
use server::*;

use std::time::Duration;
use actix::clock::delay_for;
use actix::prelude::*;

#[macro_use]
extern crate serde_json;

#[actix_rt::main]
async fn main() {
    let provider = "https://fee7372b6e224441b747bf1fde15b2bd.eth.rpc.rivet.cloud/";
    let evm_addr = SyncArbiter::start(1, move || EVMService::new(provider));
    let api = Api {
        evm: evm_addr.recipient()
    }.start();

    loop {
        // println!("in loop");
        delay_for(Duration::from_secs(10)).await;
    }
}
