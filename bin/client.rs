extern crate futures;
extern crate grpc;
extern crate cloudstore;

use cloudstore::cloudstore_grpc::*;
use cloudstore::cloudstore::*;

fn main() {
    let client = CloudStoreClient::new_plain("localhost", 8080, Default::default()).unwrap();

}
