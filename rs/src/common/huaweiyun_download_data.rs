// 定义一个下载工具

use std::process::Command;


#[derive(Deserialize)]
struct DownloadArgs {
    url: String,
    output: String,
}
