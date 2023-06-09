use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Body, Message};

use super::{Context, Workload};

#[derive(Default)]
pub struct EchoNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "echo")]
#[serde(tag = "type")]
pub struct Echo {
    echo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "echo_ok")]
#[serde(tag = "type")]
pub struct EchoOk {
    echo: String,
}

impl Workload for EchoNode {
    type Input<'a> = Echo;

    type Output = EchoOk;

    fn process(
        &mut self,
        ctx: &mut Context,
        msg: Message<Self::Input<'_>>,
    ) -> Result<Message<Self::Output>> {
        let echo = msg.body.payload.echo;
        Ok(Message {
            src: msg.dst,
            dst: msg.src,
            body: Body {
                id: Some(ctx.next_id),
                in_reply_to: msg.body.id,
                payload: EchoOk { echo },
            },
        })
    }
}
