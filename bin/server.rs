extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate cloudstore;

use std::thread;

use cloudstore::cloudstore_grpc::*;
use cloudstore::cloudstore::*;

struct CloudStoreServiceImpl;

impl CloudStore for CloudStoreServiceImpl {

}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8080);
    server.add_service(CloudStoreServer::new_service_def(CloudStoreServiceImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}