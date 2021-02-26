
/// 可以使用into trait 方便函数参数，可以接受 String 参数和 &str 参数
pub fn print_string<S:Into<String>> (info : S) {
    println!("{}",info.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_string() {
        // hello
        let info = "hello into";
        print_string(info);

        let info = "hello world".to_string();
        print_string(info)

    }
}