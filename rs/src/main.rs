use std::process::Command;

use rig::client::{CompletionClient, Nothing};
use rig::completion::{Prompt, ToolDefinition};
use rig::providers::ollama;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

// 定义 AGENT_SYSTEM_PROMPT
const AGENT_SYSTEM_PROMPT: &str =
    "你是一个智能助手，能够回答用户的问题并提供有用的信息。请根据用户的查询提供准确和相关的答案。";

#[derive(Deserialize, Serialize)]
struct Adder;

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(Deserialize)]
struct OperationArgs {
    x: i32,
    y: i32,
}

impl Tool for Adder {
    const NAME: &'static str = "add";
    type Args = OperationArgs;
    type Output = i32;
    type Error = MathError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "run a command".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {"type": "number", "description": "The first number"},
                    "y": {"type": "number", "description": "The second number"},
                },
                "required": ["x", "y"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.x + args.y)
    }
}

#[derive(Deserialize)]
struct CmdOptArgs {
    command: String,
    args: Vec<std::string::String>,
}

#[derive(Debug, thiserror::Error)]
#[error("Command execution error")]
struct CmdOptError;

// Implement conversion from std::io::Error to CmdOptError
impl From<std::io::Error> for CmdOptError {
    fn from(_err: std::io::Error) -> Self {
        CmdOptError
    }
}

#[derive(Deserialize, Serialize)]
struct CommandTool;

impl Tool for CommandTool {
    const NAME: &'static str = "command_tool";
    type Args = CmdOptArgs;
    type Output = String;
    type Error = CmdOptError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "execute a command".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": {"type": "string", "description": "The command to execute"},
                    "args": {"type": "array", "items": {"type": "string"}, "description": "The arguments for the command"},
                },
                "required": ["command","args"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // 在这里执行命令并返回结果
        let status = Command::new(&args.command).args(args.args).status()?;
        Ok(format!(
            "Executed command: {},ouput:\n {}",
            args.command, status
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: ollama::Client = ollama::Client::new(Nothing)?; // 无需参数
    let agent = client
        .agent("qwen3:4b")
        .preamble(AGENT_SYSTEM_PROMPT)
        .tool(Adder)
        .tool(CommandTool)
        .build();
    let response = agent.prompt("执行命令： ping www.baidu.com").await?;

    println!("{response}");

    Ok(())
}
