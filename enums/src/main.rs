fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let result = match four {
        IpAddrKind::V4(_) => 1,
        other => 2
    };

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max is {}", max),
        _ => ()
    }
    if let Some(max) = config_max {
        println!("The max is {}", max);
    }
}

enum IpAddrKind {
    V4(String),
    V6(String),
    Test
}