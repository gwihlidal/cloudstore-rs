extern crate futures;
extern crate grpc;
extern crate cloudstore;
extern crate file;

use std::io::{BufReader, Read};
use std::fs::File;
use std::env;

use cloudstore::cloudstore_grpc::*;
use cloudstore::cloudstore::*;

fn main() {

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let client = CloudStoreClient::new_plain("localhost", 8080, Default::default()).unwrap();

    let file = File::open("MyTestFile.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut bytes:Vec<u8> = Vec::new();
    let _result = reader.read_to_end(&mut bytes);

    let test_file = String::from("MyFile.txt");
    let test_data = file::get(String::from("MyTestFile.txt")).unwrap();
    //let test_data: Vec<u8> = vec![];

    // Store file to cloud
    let mut store_req = StoreRequest::new();
    store_req.set_filename(test_file.clone());
    store_req.set_data(test_data);
    let store_res = client.store(grpc::RequestOptions::new(), store_req);
    println!("{:?}", store_res.wait());

    // Fetch file from cloud
    let mut fetch_req = FetchRequest::new();
    fetch_req.set_filename(test_file.clone());
    let fetch_res = client.fetch(grpc::RequestOptions::new(), fetch_req);
    match fetch_res.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, fetch, _)) => {
            println!("{:?}", fetch);
            let fetch_data = fetch.get_data();
        }
    }

    // List files on cloud
    let list_req = ListRequest::new();
    let list_res = client.list(grpc::RequestOptions::new(), list_req);
    match list_res.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, stream)) => {
            for stream_item in stream {
                let response = stream_item.unwrap();
                println!("> {}", response.get_filename());
            }
        }
    }

    // Delete file from cloud
    let mut delete_req = DeleteRequest::new();
    delete_req.set_filename(test_file.clone());
    let delete_res = client.delete(grpc::RequestOptions::new(), delete_req);
    println!("{:?}", delete_res.wait());
}
