#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    println!("four: {:?}", four);
    println!("six: {:?}", six);
}

