// 遍历整个目录的工具函数
use std::fs;
use std::path::Path;

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

pub fn walk_dir(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            } else if path.is_dir() {
                files.extend(walk_dir(path.to_str().unwrap()));
            }
        }
    }
    files
}

#[derive(Serialize, Deserialize)]
pub struct WalkTool;

#[derive(Debug, thiserror::Error)]
#[error("Walk Tool Error!")]
pub struct WalkToolError;

#[derive(Deserialize)]
pub struct WalkToolArgs {
    dir: String,
}

impl Tool for WalkTool {
    const NAME: &'static str = "WalkTool";
    type Error = WalkToolError;
    type Args = WalkToolArgs;
    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "WalkTool".to_string(),
            description:
                "Recursively walk through a directory and return a list of all file paths."
                    .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "dir": {
                        "type": "string",
                        "description": "The directory path to walk through."
                    }
                },
                "required": ["dir"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(walk_dir(&args.dir))
    }
}
