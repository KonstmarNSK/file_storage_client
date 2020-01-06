use std::net::IpAddr;

pub trait Storage{

}

pub fn get_remote(addr: IpAddr, token: &str) -> Box<dyn Storage>{
    Box::new(MockStorage{})
}

struct MockStorage{

}

impl Storage for MockStorage {}