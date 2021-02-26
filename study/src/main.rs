use study::from_and_into_trait::print_string;

fn main() {
    // &str
    let info = "hello into";
    print_string(info);

    // String
    let info = "hello world".to_string();
    print_string(info);


    println!("Hello, world!");
}


