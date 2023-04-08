use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Message, MsgId};

pub trait Workload {
    type Input<'a>: Deserialize<'a>;
    type Output: Serialize;

    fn process(
        &mut self,
        next_id: MsgId,
        msg: Message<Self::Input<'_>>,
    ) -> Result<Message<Self::Output>>;
}

mod echo;
mod init;

pub use echo::EchoNode;
pub use init::InitNode;
