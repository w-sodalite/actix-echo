use actix::{Actor, ActorContext, Context, Handler, Message};
use actix::io::{FramedWrite, WriteHandler};
use log::info;
use tokio::net::tcp::OwnedWriteHalf;
use tokio_util::codec::{LinesCodec, LinesCodecError};

#[derive(Debug)]
pub enum WriteMessage {
    Close,
    Value(String),
}

impl Message for WriteMessage {
    type Result = ();
}

pub struct WriteActor {
    write: FramedWrite<String, OwnedWriteHalf, LinesCodec>,
}

impl WriteHandler<LinesCodecError> for WriteActor {}

impl WriteActor {
    pub fn new(write: FramedWrite<String, OwnedWriteHalf, LinesCodec>) -> Self {
        WriteActor { write }
    }
}

impl Actor for WriteActor {
    type Context = Context<Self>;
}

impl Handler<WriteMessage> for WriteActor {
    type Result = ();

    fn handle(&mut self, msg: WriteMessage, ctx: &mut Self::Context) -> Self::Result {
        info!("receive message: {:?}", msg);
        match msg {
            WriteMessage::Close => ctx.stop(),
            WriteMessage::Value(value) => self.write.write(value),
        }
    }
}
