use std::ffi::OsString;
use std::net::IpAddr;

use fuse::{BackgroundSession, Filesystem};

mod fs_impl;

pub fn mount(fs: Box<dyn Filesystem+Send>, mountpoint: &OsString) -> BackgroundSession<'static> {

    let bg_session;
    unsafe {
        bg_session = fuse::spawn_mount(
            *fs,
            mountpoint,
            &[])
            .unwrap();
    }

    bg_session
}

pub struct UserCredentials {
    login: String,
    password: String,
}

pub struct HostInfo {
    host: IpAddr,
    port: u32,
}

pub fn get_remote(credentials: &UserCredentials, host: &HostInfo) -> Box<dyn Filesystem> {
    // mock
//    fs_impl::RemoteFS{
//        user_token :
//    }
}