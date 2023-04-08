use serde::{Deserialize, Serialize};

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
    Echo { echo: String },
}

const MSG: &str = r#"
{
    "src":"123",
    "dest":"456",
    "body": {
        "type": "echo",
        "msg_id": 1,
        "echo": "echo 123"
    }
}"#;

fn main() -> anyhow::Result<()> {
    let payload: Message = serde_json::from_slice(MSG.as_bytes())?;
    println!("{payload:?}");
    Ok(())
}
