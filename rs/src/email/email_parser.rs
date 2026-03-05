// tool to get email

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

struct EmailContext {
    text: String,
}

#[derive(Deserialize, Serialize)]
pub struct EmailParser;

#[derive(Debug, thiserror::Error)]
#[error("Email Parser Error!")] 
pub struct EmailParserError;

#[derive(Deserialize)]
pub struct EamilParserArgs {
    path: String,
}

impl Tool for EmailParser {``
    const NAME: &'static str = "EmailParser";
    type Error = EmailParserError;
    type Args = EamilParserArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "EmailParser".to_string(),
            description: "Get emial text from txt file".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "the path of file"
                    }
                },
                "required": ["path"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(std::fs::read_to_string(args.path).unwrap())
    }
}
