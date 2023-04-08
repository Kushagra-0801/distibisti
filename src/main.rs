use std::{
    env,
    io::{stdin, stdout, BufRead, Write},
};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

mod workloads;

use workloads::{EchoNode, InitNode, Workload};

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
        _ => bail!("Only echo is currently implemented"),
    };

    let mut init_node = InitNode::default();

    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .context("Failed to read init message")?;
    let init_msg: Message<_> =
        serde_json::from_str(&buffer).context("Failed to deserialize init message")?;
    let response = init_node.process(0, init_msg)?;
    serde_json::to_writer(&mut stdout(), &response).context("Failed to write init response")?;
    stdout()
        .write(b"\n")
        .context("A newline is needed to flush stdout")?;

    main_loop(init_node.this)
}

fn run<T: Workload + Default>(_self_id: String) -> Result<()> {
    let mut msg_id = 1;
    let mut buffer = String::new();
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    loop {
        buffer.clear();
        stdin
            .read_line(&mut buffer)
            .context("Failed to read init message")?;
        msg_id += 1;

        let echo_msg: Message<_> =
            serde_json::from_str(&buffer).context("Failed to deserialize echo message")?;
        let response = T::default().process(msg_id, echo_msg)?;
        serde_json::to_writer(&mut stdout, &response).context("Failed to write echo response")?;
        stdout
            .write(b"\n")
            .context("A newline is needed to flush stdout")?;
    }
}
