use std::collections::{HashMap, HashSet};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Body, Message};

use super::{Context, Workload};

type GossipMsg = u64;

#[derive(Default)]
pub struct GossipNode {
    store: HashSet<GossipMsg>,
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
        todo!()
    }
}
