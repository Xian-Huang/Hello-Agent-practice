// 写一个测试

#[cfg(test)]
mod tests {
    use std::process::Command;

    fn test_command() {
        let comand = Command::new("ping")
            .arg("www.baidu.com")
            .output()
            .expect("Failed to execute command");
        println!(
            "Command output: {}",
            String::from_utf8_lossy(&comand.stdout)
        );
    }
}
