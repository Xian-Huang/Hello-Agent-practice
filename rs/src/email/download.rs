use tokio::process::Command;

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Serialize, Deserialize)]
pub struct HwDownloadTool;

#[derive(Deserialize)]
pub struct HwDownloadArgs {
    url: String,
    password: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Huawei Cloud Download Error!")]
pub struct HwDownloadError;

impl Tool for HwDownloadTool {
    const NAME: &'static str = "HwDownloadTool";
    type Error = HwDownloadError;
    type Args = HwDownloadArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "HwDownloadTool".to_string(),
            description: "Download data from Huawei Cloud using the provided URL and output path（D:/ldy/Hello-Agent-practice/rs）."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The download URL provided in the email."
                    },
                    "password": {
                        "type": "string",
                        "description": "The password or extraction code required for the download, if applicable."
                    }
                },
                "required": ["url", "password"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // D:/ProgramFiles/obsutil_windows_amd64_5.7.9/obsutil.exe share-cp {url} {outdir} -ac {password} -f -r -vmd5 -u
        // 执行命令 并且流输出下载记录
        let mut child = Command::new("D:/ProgramFiles/obsutil_windows_amd64_5.7.9/obsutil.exe")
            .args([
                "share-cp",
                &args.url,
                "D:/ldy/Hello-Agent-practice/rs",
                "-ac",
                &args.password,
                "-f",
                "-r",
                "-vmd5",
                "-u",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Proceess Error");

        let stdout = child.stdout.take().expect("no stdout");
        let stderr = child.stderr.take().expect("no stderr");

        let stdout_handle = tokio::spawn(async {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                println!("[stdout] {}", line);
            }
        });

        let stderr_handle = tokio::spawn(async {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                eprintln!("[stderr] {}", line);
            }
        });

        let status = child.wait().await.unwrap();
        stdout_handle.await.unwrap();
        stderr_handle.await.unwrap();
        Ok(format!(
            "Simulated download from URL: {} to output path: D:/ldy/Hello-Agent-practice/rs with password: {}",
            args.url, args.password
        ))
    }
}
