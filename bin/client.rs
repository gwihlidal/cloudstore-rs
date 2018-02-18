extern crate futures;
extern crate grpc;
extern crate cloudstore;
extern crate file;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate rusoto_credentials;

use std::io::{BufReader, Read};
use std::fs::File;
use std::env;

use cloudstore::cloudstore_grpc::*;
use cloudstore::cloudstore::*;

use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_s3::*;

const BUCKET_NAME: &'static str = "p4content";

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

    // Delete file from cloud
    let mut delete_req = DeleteRequest::new();
    delete_req.set_filename(test_file.clone());
    let delete_res = client.delete(grpc::RequestOptions::new(), delete_req);
    println!("{:?}", delete_res.wait());

    /*let region = Region::Custom {
        name: "hcy-minio".to_owned(),
        endpoint: "http://192.168.1.69:9000".to_owned(),
    };*/

    //let provider = StaticProvider::new_minimal("AJ20P3XYDOURW7WZSHJ1".to_owned(), "EVT9XzEw/77PevdntA88wjcEBF2cANl/Duc09mkl".to_owned())
    //            .credentials().wait();
    /*let param_provider: Option<ParametersProvider>;
    param_provider = Some(
        ParametersProvider::with_params(
            "AJ20P3XYDOURW7WZSHJ1",
            "EVT9XzEw/77PevdntA88wjcEBF2cANl/Duc09mkl",
            None).unwrap()
    );*/

    //let provider = DefaultCredentialsProvider::new(param_provider).unwrap();

    //let tls_client = default_tls_client().unwrap();
    //let provider = DefaultCredentialsProvider::new().unwrap();
   // let credentials = DefaultCredentialsProvider::new().unwrap();
    //let s3 = rusoto_s3::S3Client::new(tls_client, provider, region);
    //let s3 = rusoto_s3::S3Client::simple(region);

    /*let mut request = rusoto_s3::PutObjectRequest::default();
    request.key = test_file.to_string();
    request.bucket = "p4content".to_string();
    request.body = Some(bytes);
    request.content_type = Some("text/plain".to_string());

    let _result = s3.put_object(&request);*/

}
