use super::storage::{Storage, get_remote};
use std::net::{IpAddr, Ipv4Addr};

pub fn authenticate(login: String, pass: String) -> Box<dyn Storage> {
    // todo: mock impl
    get_remote(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), "mock_token")
}