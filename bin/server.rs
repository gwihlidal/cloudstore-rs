#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate cloudstore;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate rusoto_credential;

use std::thread;
use std::env;

use cloudstore::cloudstore_grpc::*;
use cloudstore::cloudstore::*;

use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_s3::*;

const BUCKET_NAME: &'static str = "p4content";

struct CloudStoreServiceImpl;

impl CloudStore for CloudStoreServiceImpl {
    fn store(
        &self,
        _m: grpc::RequestOptions,
        req: StoreRequest,
    ) -> grpc::SingleResponse<StoreResponse> {
        let endpoint = "192.168.1.69:9000".to_string();
        let bucket_name = "p4content".to_string();
        let file_name = req.get_filename().to_string();
        let mut r = StoreResponse::new();
        println!("Store request - filename: {}", file_name);

        let region = Region::Custom {
            name: "hcy-storage".to_owned(),
            endpoint: "http://192.168.1.69:9000".to_owned(),
        };

        let tls_client = default_tls_client().unwrap();
        let provider = DefaultCredentialsProvider::new().unwrap();
        let s3 = rusoto_s3::S3Client::new(tls_client, provider, region);

        /*let credentials = rusoto_credential::StaticProvider::new_minimal(
            "AJ20P3XYDOURW7WZSHJ1".to_string(),
            "EVT9XzEw/77PevdntA88wjcEBF2cANl/Duc09mkl".to_string());

        let s3 = rusoto_s3::S3Client::new(
            rusoto_core::default_tls_client().expect(
                "Unable to create default TLS client for Rusoto",
            ),
            credentials,
            rusoto_core::region::Region::Custom {
                name: "hcy-minio".to_string(),
                endpoint: endpoint,
            },
        );*/

        let request = rusoto_s3::PutObjectRequest {
            bucket: bucket_name,
            key: file_name,
            body: Some(req.get_data().to_vec()),
            //content_type: Some("text/plain".to_string()),
            ..Default::default()
        };

        match s3.put_object(&request) {
            Ok(out) => {
                println!("put object: {:?}", request);
                r.set_filename(req.get_filename().to_string());
            },
            Err(err) => {
                println!("put object failed: {:?}", err);
            }
        }

        grpc::SingleResponse::completed(r)
    }

    fn delete(
        &self,
        _m: grpc::RequestOptions,
        req: DeleteRequest,
    ) -> grpc::SingleResponse<DeleteResponse> {
        let endpoint = "192.168.1.69:9000".to_string();
        let bucket_name = "p4content".to_string();
        let file_name = req.get_filename().to_string();
        let mut r = DeleteResponse::new();
        println!("Delete request - filename: {}", file_name);

        /*let credentials = rusoto_credential::StaticProvider::new_minimal(
            "AJ20P3XYDOURW7WZSHJ1".to_string(),
            "EVT9XzEw/77PevdntA88wjcEBF2cANl/Duc09mkl".to_string());

        let s3 = rusoto_s3::S3Client::new(
            rusoto_core::default_tls_client().expect(
                "Unable to create default TLS client for Rusoto",
            ),
            credentials,
            rusoto_core::region::Region::Custom {
                name: "minio".to_string(),
                endpoint: endpoint,
            },
        );*/

        let region = Region::Custom {
            name: "hcy-storage".to_owned(),
            endpoint: "http://192.168.1.69:9000".to_owned(),
        };

        let tls_client = default_tls_client().unwrap();
        let provider = DefaultCredentialsProvider::new().unwrap();
        let s3 = rusoto_s3::S3Client::new(tls_client, provider, region);

        let request = rusoto_s3::DeleteObjectRequest {
            bucket: bucket_name,
            key: file_name,
            ..Default::default()
        };

        match s3.delete_object(&request) {
            Ok(out) => {
                println!("delete object: {:?}", request);
                r.set_filename(req.get_filename().to_string());
            },
            Err(err) => {
                println!("delete object failed: {:?}", err);
            }
        }

        grpc::SingleResponse::completed(r)
    }

    fn fetch(
        &self,
        _m: grpc::RequestOptions,
        req: FetchRequest,
    ) -> grpc::SingleResponse<FetchResponse> {
        let endpoint = "192.168.1.69:9000".to_string();
        let bucket_name = "p4content".to_string();
        let file_name = req.get_filename().to_string();
        let mut r = FetchResponse::new();
        println!("Fetch request - filename: {}", file_name);

        /*let credentials = rusoto_credential::StaticProvider::new_minimal(
            "AJ20P3XYDOURW7WZSHJ1".to_string(),
            "EVT9XzEw/77PevdntA88wjcEBF2cANl/Duc09mkl".to_string());

        let s3 = rusoto_s3::S3Client::new(
            rusoto_core::default_tls_client().expect(
                "Unable to create default TLS client for Rusoto",
            ),
            credentials,
            rusoto_core::region::Region::Custom {
                name: "minio".to_string(),
                endpoint: endpoint,
            },
        );*/

        let region = Region::Custom {
            name: "hcy-storage".to_owned(),
            endpoint: "http://192.168.1.69:9000".to_owned(),
        };

        let tls_client = default_tls_client().unwrap();
        let provider = DefaultCredentialsProvider::new().unwrap();
        let s3 = rusoto_s3::S3Client::new(tls_client, provider, region);

        let request = rusoto_s3::GetObjectRequest {
            bucket: bucket_name,
            key: file_name,
            //range: Some("bytes=0-1".to_owned()),
            ..Default::default()
        };

        let mut data = Vec::<u8>::new();

        let result = s3.get_object(&request);
        match result {
            //Err(GetObjectError::NoSuchKey(_)) => (),
            Ok(out) => {
                println!("fetch object: {:?}", out);
                match std::io::copy(&mut out.body.unwrap(), &mut data) {
                    Err(err) => {
                        println!("Failed to copy object data");
                        return grpc::SingleResponse::err(grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                            grpc_status: 15,//grpc::grpc::GrpcStatus::DataLoss,
                            grpc_message: "Failed to copy object data".to_string()
                        }));
                    },
                    Ok(out) => r.set_data(data.to_owned()),
                }
                r.set_data(data);
            },
            Err(err) => {
                println!("fetch object failed: {:?}", err);
                return grpc::SingleResponse::err(grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                    grpc_status: 15,//grpc::grpc::GrpcStatus::DataLoss,
                    grpc_message: "Failed to fetch object".to_string()
                }));
            }
        }

        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8080);
    server.add_service(CloudStoreServer::new_service_def(CloudStoreServiceImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}