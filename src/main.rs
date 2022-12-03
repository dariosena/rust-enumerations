#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

impl IpAddrKind {
    fn route(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct IpV4Addr {
    address: String,
}

#[derive(Debug)]
struct IpV6Addr {
    address: String,
}

#[derive(Debug)]
enum IpAddrWithStructs {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

impl IpAddrWithStructs {
    fn address_with_structs(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum IpAddrWithState {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrWithState {
    fn address_with_state(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn new(kind: IpAddrKind, address: String) -> Self {
        Self { kind, address }
    }

    fn address(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4;
    four.route();

    let six = IpAddrKind::V6;
    six.route();

    let home = IpAddr::new(IpAddrKind::V4, "127.0.0.1".to_string());
    home.address();

    let loopback = IpAddr::new(IpAddrKind::V6, "::1".to_string());
    loopback.address();

    let home = IpAddrWithState::V4(127, 0, 0, 1);
    home.address_with_state();

    let loopback = IpAddrWithState::V6("::1".to_string());
    loopback.address_with_state();

    let home_address = IpV4Addr {
        address: "127.0.0.1".to_string(),
    };

    let home = IpAddrWithStructs::V4(home_address);
    home.address_with_structs();

    let loopback_address = IpV6Addr {
        address: "::1".to_string(),
    };

    let loopback = IpAddrWithStructs::V6(loopback_address);
    loopback.address_with_structs();
}
