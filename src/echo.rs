use actix::{
    Actor, ActorContext, ActorTryFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, Message, WrapFuture,
};
use actix_service::Service;
use log::info;

use crate::write::{WriteActor, WriteMessage};

#[derive(Debug)]
pub enum EchoMessage {
    Close,
    Value(String),
}

impl Message for EchoMessage {
    type Result = ();
}

pub struct EchoActor {
    write: Addr<WriteActor>,
}

impl EchoActor {
    pub fn new(write: Addr<WriteActor>) -> Self {
        EchoActor { write }
    }
}

impl Actor for EchoActor {
    type Context = Context<Self>;
}

impl Handler<EchoMessage> for EchoActor {
    type Result = ();

    fn handle(&mut self, msg: EchoMessage, ctx: &mut Self::Context) -> Self::Result {
        info!("receive message: {:?}", msg);
        match msg {
            EchoMessage::Close => {
                self.write.do_send(WriteMessage::Close);
                ctx.stop();
            }
            EchoMessage::Value(value) => self
                .write
                .do_send(WriteMessage::Value(value.to_uppercase())),
        }
    }
}
