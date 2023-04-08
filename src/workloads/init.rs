use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Body, Message};

use super::{Context, Workload};

#[derive(Default)]
pub struct InitNode {
    pub this: String,
    pub others: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "init")]
#[serde(tag = "type")]
pub struct Init {
    #[serde(rename = "node_id")]
    this: String,
    #[serde(rename = "node_ids")]
    others: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "init_ok")]
#[serde(tag = "type")]
pub struct InitOk {}

impl Workload for InitNode {
    type Input<'a> = Init;

    type Output = InitOk;

    fn process(
        &mut self,
        ctx: &mut Context,
        msg: Message<Self::Input<'_>>,
    ) -> Result<Message<Self::Output>> {
        self.this = msg.body.payload.this;
        self.others = msg.body.payload.others;
        Ok(Message {
            src: msg.dst,
            dst: msg.src,
            body: Body {
                id: Some(ctx.next_id),
                in_reply_to: msg.body.id,
                payload: InitOk {},
            },
        })
    }
}
