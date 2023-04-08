use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{Body, Message};

use super::{Context, Workload};

type GossipMsg = u64;

#[derive(Default)]
pub struct GossipNode {
    store: HashSet<GossipMsg>,
    neighbours: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Incoming {
    Broadcast {
        message: GossipMsg,
    },
    Read {},
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
pub enum Outgoing {
    BroadcastOk {},
    ReadOk { messages: Vec<GossipMsg> },
    TopologyOk {},
}

impl Workload for GossipNode {
    type Input<'a> = Incoming;

    type Output = Outgoing;

    fn process(
        &mut self,
        ctx: &mut Context,
        msg: Message<Self::Input<'_>>,
    ) -> Result<Message<Self::Output>> {
        let response = match msg.body.payload {
            Incoming::Broadcast { message } => {
                self.store.insert(message);
                Outgoing::BroadcastOk {}
            }
            Incoming::Read {} => {
                let messages = self.store.iter().copied().collect();
                Outgoing::ReadOk { messages }
            }
            Incoming::Topology { mut topology } => {
                let Some(neighbours) = topology.remove(&ctx.node_id) else {
                    bail!("Topology did not include neighbours")
                };
                self.neighbours = neighbours;
                Outgoing::TopologyOk {}
            }
        };
        Ok(Message {
            src: msg.dst,
            dst: msg.src,
            body: Body {
                id: Some(ctx.next_id),
                in_reply_to: msg.body.id,
                payload: response,
            },
        })
    }
}
