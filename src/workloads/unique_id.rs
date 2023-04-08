use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Body, Message};

use super::{Context, Workload};

pub struct UniqueIdNode {
    counter: u64,
    timestamp: SystemTime,
}

impl Default for UniqueIdNode {
    fn default() -> Self {
        Self {
            counter: Default::default(),
            timestamp: SystemTime::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "generate")]
#[serde(tag = "type")]
pub struct Generate {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "generate_ok")]
#[serde(tag = "type")]
pub struct GenerateOk {
    id: String,
}

impl Workload for UniqueIdNode {
    type Input<'a> = Generate;

    type Output = GenerateOk;

    fn process(
        &mut self,
        ctx: &mut Context,
        msg: Message<Self::Input<'_>>,
    ) -> Result<Message<Self::Output>> {
        let now = SystemTime::now();
        if now
            .duration_since(self.timestamp)
            .unwrap_or(Duration::ZERO)
            .as_millis()
            > 0
        {
            self.timestamp = now;
            self.counter = 0;
        } else {
            self.counter += 1;
        }
        let timestamp = self.timestamp.duration_since(UNIX_EPOCH)?.as_millis() as u64;
        let this_node = &ctx.node_id;
        let counter = self.counter % (1 << 32);
        let id = format!("{timestamp:016X}-{this_node:->5}-{counter:08X}");
        Ok(Message {
            src: msg.dst,
            dst: msg.src,
            body: Body {
                id: Some(ctx.next_id),
                in_reply_to: msg.body.id,
                payload: GenerateOk { id },
            },
        })
    }
}
