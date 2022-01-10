use actix::{Actor, Addr, Context, StreamHandler};
use log::info;
use tokio_util::codec::LinesCodecError;

use crate::echo::{EchoActor, EchoMessage};

pub struct ReadActor {
    echo: Addr<EchoActor>,
}

impl ReadActor {
    pub fn new(echo: Addr<EchoActor>) -> Self {
        ReadActor { echo }
    }
}

impl Actor for ReadActor {
    type Context = Context<Self>;
}

impl StreamHandler<Result<String, LinesCodecError>> for ReadActor {
    fn handle(&mut self, item: Result<String, LinesCodecError>, _: &mut Self::Context) {
        info!("receive message: {:?}", item);
        match item {
            Ok(value) => self.echo.do_send(EchoMessage::Value(value)),
            Err(_) => self.echo.do_send(EchoMessage::Close),
        }
    }
}
