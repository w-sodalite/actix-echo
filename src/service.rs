use std::task::Poll;

use actix::{Actor, AsyncContext};
use actix::io::FramedWrite;
use actix_service::{Service, ServiceFactory};
use futures::future::{ok, Ready};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::echo::EchoActor;
use crate::read::ReadActor;
use crate::write::WriteActor;

pub struct ActorService;

impl Service<TcpStream> for ActorService {
    type Response = ();
    type Error = anyhow::Error;
    type Future = Ready<Result<(), Self::Error>>;

    fn poll_ready(&self, _: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: TcpStream) -> Self::Future {
        let (read, write) = req.into_split();
        ReadActor::create(|c1| {
            c1.add_stream(FramedRead::new(read, LinesCodec::new()));
            let echo = EchoActor::create(|_| {
                let write = WriteActor::create(|c3| {
                    let write = FramedWrite::new(write, LinesCodec::new(), c3);
                    WriteActor::new(write)
                });
                EchoActor::new(write)
            });
            ReadActor::new(echo)
        });
        ok(())
    }
}

pub struct ActorServiceFactory;

impl ServiceFactory<TcpStream> for ActorServiceFactory {
    type Response = ();
    type Error = anyhow::Error;
    type Config = ();
    type Service = ActorService;
    type InitError = anyhow::Error;
    type Future = Ready<Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        ok(ActorService)
    }
}
