use futures::StreamExt;
use rig::agent::stream_to_stdout;
use rig::client::{CompletionClient, Nothing};
use rig::providers::ollama;
use rig::streaming::{StreamedAssistantContent, StreamingPrompt};
pub mod common;
pub mod email;
// 定义 AGENT_SYSTEM_PROMPT
const AGENT_SYSTEM_PROMPT: &str =
    "你是一个智能助手，能够回答用户的问题并提供有用的信息。请根据用户的查询提供准确和相关的答案。";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: ollama::Client = ollama::Client::new(Nothing)?; // 无需参数
    let agent = client
        .agent("qwen3:8b")
        .preamble(AGENT_SYSTEM_PROMPT)
        .tool(email::email_parser::EmailParser)
        .tool(email::download::HwDownloadTool)
        .tool(common::walk::WalkTool)
        .default_max_turns(3)
        .build();

    // `stream_prompt` returns the stream directly when awaited, so drop the `?`.
    let mut stream = agent
        .stream_prompt("邮箱文件路径：D:/ldy/Hello-Agent-practice/rs/data/huaweiyun_email.txt，按照邮件内容选择下载数据，邮件内容中包含下载所需参数，并遍历下载后的目录，将所有的基因序列文件（.fq.gz,.fastq.gz）名称中的项目编号提取出来（UOP开头，后续是数字），最后列举出文件对应的项目编号。")
        .await;

    let res = stream_to_stdout(&mut stream).await?;

    println!("Token usage response: {usage:?}", usage = res.usage());
    println!("Final text response: {message:?}", message = res.response());

    Ok(())
}
