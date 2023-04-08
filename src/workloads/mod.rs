use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Message, MsgId};

#[derive(Debug, Clone)]
pub struct Context {
    pub next_id: MsgId,
    pub node_id: String,
    pub neighbours: Vec<String>,
}

pub trait Workload {
    type Input<'a>: Deserialize<'a>;
    type Output: Serialize;

    fn process(
        &mut self,
        ctx: &mut Context,
        msg: Message<Self::Input<'_>>,
    ) -> Result<Message<Self::Output>>;
}

mod echo;
mod init;
mod unique_id;

pub use echo::EchoNode;
pub use init::InitNode;
pub use unique_id::UniqueIdNode;
