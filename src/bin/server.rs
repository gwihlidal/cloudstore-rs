extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate cloudstore;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate rusoto_credential;
extern crate rustracing;
extern crate rustracing_jaeger;
#[macro_use]
extern crate trackable;

use std::thread;
use std::env;
use std::net::ToSocketAddrs;

use rustracing::tag::Tag;
use rustracing_jaeger::Tracer;
use rustracing_jaeger::reporter::JaegerCompactReporter;

use cloudstore::cloudstore_grpc::*;
use cloudstore::cloudstore::*;

//use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_s3::*;

struct CloudStoreServiceImpl {
    s3_provider: rusoto_credential::StaticProvider,
    //s3_client: &rusoto_s3::S3Client,
    s3_region: rusoto_core::region::Region,
    s3_bucket: String,
    reporter: JaegerCompactReporter,
}

impl CloudStore for CloudStoreServiceImpl {
    fn store(
        &self,
        _m: grpc::RequestOptions,
        req: StoreRequest,
    ) -> grpc::SingleResponse<StoreResponse> {
        let mut r = StoreResponse::new();
        let (tracer, span_rx) = Tracer::new(rustracing::sampler::AllSampler);
        {
            let mut span0 = tracer.span("store").start();

            let file_name = req.get_filename().to_string();
            
            println!("Store request - filename: {}", file_name);

            let s3 = rusoto_s3::S3Client::new(
                rusoto_core::default_tls_client().expect(
                    "Unable to create default TLS client for Rusoto",
                ),
                self.s3_provider.clone(),
                self.s3_region.clone(),
            );

            let request = rusoto_s3::PutObjectRequest {
                bucket: self.s3_bucket.clone(),
                key: file_name,
                body: Some(req.get_data().to_vec()),
                //content_type: Some("text/plain".to_string()),
                ..Default::default()
            };

            match s3.put_object(&request) {
                Ok(out) => {
                    println!("put object: {:?}", out);
                    r.set_filename(req.get_filename().to_string());
                },
                Err(err) => {
                    println!("put object failed: {:?}", err);
                    span0.log(|log| {
                        log.error().message("Put object failed");
                    });
                }
            }
        }

        track_try_unwrap!(self.reporter.report(&span_rx.try_iter().collect::<Vec<_>>()));
        grpc::SingleResponse::completed(r)
    }

    fn delete(
        &self,
        _m: grpc::RequestOptions,
        req: DeleteRequest,
    ) -> grpc::SingleResponse<DeleteResponse> {
        let mut r = DeleteResponse::new();
        let (tracer, span_rx) = Tracer::new(rustracing::sampler::AllSampler);
        {
            let mut span0 = tracer.span("store").start();
            let file_name = req.get_filename().to_string();
            
            println!("Delete request - filename: {}", file_name);

            let s3 = rusoto_s3::S3Client::new(
                rusoto_core::default_tls_client().expect(
                    "Unable to create default TLS client for Rusoto",
                ),
                self.s3_provider.clone(),
                self.s3_region.clone(),
            );

            let request = rusoto_s3::DeleteObjectRequest {
                bucket: self.s3_bucket.clone(),
                key: file_name,
                ..Default::default()
            };

            match s3.delete_object(&request) {
                Ok(out) => {
                    println!("delete object: {:?}", out);
                    r.set_filename(req.get_filename().to_string());
                },
                Err(err) => {
                    println!("delete object failed: {:?}", err);
                    span0.log(|log| {
                        log.error().message("Delete object failed");
                    });
                }
            }
        }

        track_try_unwrap!(self.reporter.report(&span_rx.try_iter().collect::<Vec<_>>()));
        grpc::SingleResponse::completed(r)
    }

    fn fetch(
        &self,
        _m: grpc::RequestOptions,
        req: FetchRequest,
    ) -> grpc::SingleResponse<FetchResponse> {
        let mut r = FetchResponse::new();
        let (tracer, span_rx) = Tracer::new(rustracing::sampler::AllSampler);
        {
            let mut span0 = tracer.span("store").start();
            let file_name = req.get_filename().to_string();
            
            println!("Fetch request - filename: {}", file_name);

            let s3 = rusoto_s3::S3Client::new(
                rusoto_core::default_tls_client().expect(
                    "Unable to create default TLS client for Rusoto",
                ),
                self.s3_provider.clone(),
                self.s3_region.clone(),
            );

            let request = rusoto_s3::GetObjectRequest {
                bucket: self.s3_bucket.clone(),
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
                        Err(_err) => {
                            println!("Failed to copy object data");
                            span0.log(|log| {
                                log.error().message("Failed to copy object data");
                            });
                            track_try_unwrap!(self.reporter.report(&span_rx.try_iter().collect::<Vec<_>>()));
                            return grpc::SingleResponse::err(grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                                grpc_status: 15,//grpc::grpc::GrpcStatus::DataLoss,
                                grpc_message: "Failed to copy object data".to_string()
                            }));
                        },
                        Ok(_out) => r.set_data(data.to_owned()),
                    }
                    r.set_data(data);
                },
                Err(err) => {
                    println!("fetch object failed: {:?}", err);
                    span0.log(|log| {
                        log.error().message("Fetch object failed");
                    });
                    track_try_unwrap!(self.reporter.report(&span_rx.try_iter().collect::<Vec<_>>()));
                    return grpc::SingleResponse::err(grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                        grpc_status: 15,//grpc::grpc::GrpcStatus::DataLoss,
                        grpc_message: "Failed to fetch object".to_string()
                    }));
                }
            }
        }

        track_try_unwrap!(self.reporter.report(&span_rx.try_iter().collect::<Vec<_>>()));
        grpc::SingleResponse::completed(r)
    }

    fn list(
        &self,
        _m: grpc::RequestOptions,
        _req: ListRequest,
    ) -> grpc::StreamingResponse<ListResponse> {
        let mut responses: Vec<ListResponse> = vec![];

        //println!("Starting list");

        let (tracer, span_rx) = Tracer::new(rustracing::sampler::AllSampler);
        {
            let span0 = tracer.span("list").start();

            //let mut span1 = tracer
             //   .span("initialize")
             //   .child_of(&span0)
             //   .start();

            let s3 = rusoto_s3::S3Client::new(
                rusoto_core::default_tls_client().expect(
                    "Unable to create default TLS client for Rusoto",
                ),
                self.s3_provider.clone(),
                self.s3_region.clone(),
            );

            let request = rusoto_s3::ListObjectsV2Request {
                bucket: self.s3_bucket.clone(),
                //start_after: Some("foo".to_owned()),
                ..Default::default()
            };

            let mut span2 = tracer
                .span("iterator")
                .child_of(&span0)
                .start();

            match s3.list_objects_v2(&request) {
                Ok(items) => {
                    //println!("list objects: {:#?}", items); 
                    for item in items.contents.iter() {
                        for object in item {
                            let key = &object.key;
                            //let e_tag = &object.e_tag;

                            let mut response = ListResponse::new();
                            response.set_filename(key.clone().unwrap());

                            span2.log(|log| {
                                log.std().message(key.clone().unwrap());
                            });

                            /* let mut span1 = tracer
                                .span("list::object")
                                .child_of(&span0)
                                .tag(Tag::new("filename", key.clone().unwrap()))
                                .start();*/

                            //let mut tags = ::protobuf::RepeatedField<ObjectTag>::new();

                            // SLOWWW - does it download the actual data?...
                            /*let tagging_req = rusoto_s3::GetObjectTaggingRequest {
                                bucket: self.s3_bucket.clone(),
                                key: key.clone().unwrap(),
                                .. Default::default()
                            };

                            match s3.get_object_tagging(&tagging_req) {
                                Ok(out) => {
                                    for tag_entry in out.tag_set.iter() {
                                        let mut tag = ObjectTag::new();
                                        tag.set_key(tag_entry.key.clone());
                                        tag.set_value(tag_entry.value.clone());
                                        response.mut_tags().push(tag);
                                    }

                                    println!("get object tagging: {:?}", out);
                                },
                                Err(_err) => {
                                }        
                            };*/

                            //println!("\t\"{}\", size: {} bytes", object.key.clone().unwrap(), object.size.unwrap());

                            responses.push(response);
                        }
                    }
                },
                Err(err) => {
                    println!("delete object failed: {:?}", err);
                }
            }
        }

        //println!("Ending list");

        track_try_unwrap!(self.reporter.report(&span_rx.try_iter().collect::<Vec<_>>()));
        grpc::StreamingResponse::iter(responses.into_iter())
    }
}

fn main() {
    println!("Starting service");

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    //let tracing_url = String::from("gw-jaeger-agent.s33d.cloud:6831");
    let tracing_url = String::from("127.0.0.1:6831");
    let s3_endpoint = "http://gw-minio.s33d.cloud:9000".to_string();
    let bucket_name = "p4content".to_string();

    let credentials = rusoto_credential::StaticProvider::new_minimal(
        "ACCESS_KEY".to_string(),
        "SECRET_KEY".to_string());

    let s3_region = rusoto_core::region::Region::Custom {
        name: "us-east-1".to_string(),
        endpoint: s3_endpoint.clone(),
    };

    /*let s3 = rusoto_s3::S3Client::new(
        rusoto_core::default_tls_client().expect(
            "Unable to create default TLS client for Rusoto",
        ),
        credentials,
        s3_region,
    );*/

    //let mut reporter = track_try_unwrap!(JaegerCompactReporter::new("cloudstore"));
    let mut service_impl = CloudStoreServiceImpl {
        s3_provider: credentials.to_owned(),
        s3_bucket: bucket_name.clone(),
        s3_region: s3_region.to_owned(),
        reporter: track_try_unwrap!(JaegerCompactReporter::new("cloudstore"))
    };

    // Configure Jaeger / Rust OpenTracing
    service_impl.reporter.add_service_tag(Tag::new("action", "testing"));
    if let Ok(mut addrs) = tracing_url.to_socket_addrs() {
        if let Some(addr) = addrs.next() {
            println!("Setting tracing endpoint to: {}", addr);
            //info!(log, "Setting tracing endpoint to: {}", addr);
            service_impl.reporter.set_agent_addr(addr);
        }
    }

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8080);
    server.add_service(CloudStoreServer::new_service_def(service_impl));
    server.http.set_cpu_pool_threads(8);
    let _server = server.build().expect("server");

    println!("Service started");

    loop {
        thread::park();
    }
}