#![allow(private_in_public)]

use std::{
    env,
    io::{stdin, stdout, BufRead, Write},
};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

mod workloads;

use workloads::{Context as Ctx, EchoNode, GossipNode, InitNode, UniqueIdNode, Workload};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Message<Payload> {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body<Payload>,
}

type MsgId = i64;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Body<Payload> {
    #[serde(rename = "msg_id")]
    id: Option<MsgId>,
    in_reply_to: Option<MsgId>,
    #[serde(flatten)]
    payload: Payload,
}

fn main() -> Result<()> {
    let workload = env::args()
        .nth(1)
        .context("Missing workload type argument")?;
    let main_loop = match workload.as_str() {
        "echo" => run::<EchoNode>,
        "uniqueid" => run::<UniqueIdNode>,
        "gossip" => run::<GossipNode>,
        _ => bail!("Only echo is currently implemented"),
    };

    let mut init_node = InitNode::default();

    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .context("Failed to read init message")?;
    let init_msg: Message<_> =
        serde_json::from_str(&buffer).context("Failed to deserialize init message")?;
    let response = init_node.process(
        &mut Ctx {
            next_id: 0,
            node_id: String::new(),
            neighbours: Vec::new(),
        },
        init_msg,
    )?;
    serde_json::to_writer(&mut stdout(), &response).context("Failed to write init response")?;
    stdout()
        .write(b"\n")
        .context("A newline is needed to flush stdout")?;

    main_loop(init_node.this, init_node.others)
}

fn run<T: Workload + Default>(self_id: String, others: Vec<String>) -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    let mut ctx = Ctx {
        next_id: 1,
        node_id: self_id,
        neighbours: others,
    };

    let mut node = T::default();
    loop {
        buffer.clear();
        stdin
            .read_line(&mut buffer)
            .context("Failed to read message")?;
        ctx.next_id += 1;

        let msg: Message<_> =
            serde_json::from_str(&buffer).context("Failed to deserialize message")?;
        let response = node.process(&mut ctx, msg)?;
        serde_json::to_writer(&mut stdout, &response).context("Failed to write response")?;
        stdout
            .write(b"\n")
            .context("A newline is needed to flush stdout")?;
    }
}
