// 写一个测试

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_command() {
        let comand = Command::new("ping").arg("www.baidu.com").status().unwrap();
        println!("Command executed with status: {}", comand);
    }
}
