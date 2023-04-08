use std::{
    env,
    io::{stdin, stdout, BufRead, Write},
};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

mod workloads;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

type MsgId = i64;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Body {
    #[serde(rename = "msg_id")]
    id: Option<MsgId>,
    in_reply_to: Option<MsgId>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Init {
        #[serde(rename = "node_id")]
        this: String,
        #[serde(rename = "node_ids")]
        others: Vec<String>,
    },
    InitOk {},
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    let workload = env::args()
        .nth(1)
        .context("Missing workload type argument")?;
    match workload.as_str() {
        "echo" => (),
        _ => bail!("Only echo is currently implemented"),
    }

    let mut msg_id = 0;

    stdin
        .read_line(&mut buffer)
        .context("Failed to read init message")?;
    let init_msg: Message =
        serde_json::from_str(&buffer).context("Failed to deserialize init message")?;
    eprintln!("Received: {init_msg:?}");
    let Payload::Init { .. } = init_msg.body.payload else {
        bail!("First message should have been an Init");
    };
    let response = Message {
        src: init_msg.dst,
        dst: init_msg.src,
        body: Body {
            id: Some(msg_id),
            in_reply_to: init_msg.body.id,
            payload: Payload::InitOk {},
        },
    };
    serde_json::to_writer(&mut stdout, &response).context("Failed to write init response")?;
    stdout
        .write(b"\n")
        .context("A newline is needed to flush stdout")?;

    loop {
        buffer.clear();
        stdin
            .read_line(&mut buffer)
            .context("Failed to read init message")?;
        msg_id += 1;

        let echo_msg: Message =
            serde_json::from_str(&buffer).context("Failed to deserialize echo message")?;
        eprintln!("Received: {echo_msg:?}");
        let Payload::Echo{ echo } = echo_msg.body.payload else {
            bail!("Init workload should get init messages");
        };
        let response = Message {
            src: echo_msg.dst,
            dst: echo_msg.src,
            body: Body {
                id: Some(msg_id),
                in_reply_to: echo_msg.body.id,
                payload: Payload::EchoOk { echo },
            },
        };
        serde_json::to_writer(&mut stdout, &response).context("Failed to write echo response")?;
        stdout
            .write(b"\n")
            .context("A newline is needed to flush stdout")?;
    }
}
