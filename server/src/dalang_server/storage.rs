use actix::{Actor, Context};

pub struct Storage {

}

impl Actor for Storage {
    type Context = Context<Self>;
}