/*
 * Copyright (c)2013-2021 ZeroTier, Inc.
 *
 * Use of this software is governed by the Business Source License included
 * in the LICENSE.TXT file in the project's root directory.
 *
 * Change Date: 2026-01-01
 *
 * On the date above, in accordance with the Business Source License, use
 * of this software will be governed by version 2.0 of the Apache License.
 */
/****/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!(concat!(env!("OUT_DIR"), "/libzt.rs"));

use std::ffi::{c_void, CStr, CString};
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;
use std::io::{self /*, Error, ErrorKind*/};
use std::net::{/*Ipv4Addr, Ipv6Addr,*/ SocketAddr, ToSocketAddrs};
use std::convert::TryInto;

pub struct ZeroTierServer {
    // TODO
}

impl ZeroTierServer {

    pub fn server_create_socket(&self) -> i32 {
        pub const sock_stream: i32 = 1;
        pub const af_inet6: i32 = 10;
        unsafe {
            return zts_socket(af_inet6, sock_stream, 0);
        }
    }

    pub fn server_bind(&self, fd: i32, addr: io::Result<&SocketAddr>) -> i32 {
        let addr = addr.unwrap();
        unsafe {
            let full_str = addr.to_string();
            let full_vec = full_str.split(":");
            let lvec: Vec<&str> = full_vec.collect();
            let addr_str = lvec[0];
            let port = addr.port();
            #[allow(temporary_cstring_as_ptr)]
            return zts_bind(
                fd,
                CString::new(addr_str).unwrap().as_ptr(),
                port,
            );
        }
    }

    pub fn server_listen(&self, fd: i32, backlog: i32) -> i32 {
        unsafe {
            return zts_listen(fd, backlog);
        }
    }

    pub fn server_accept(&self, fd: i32) -> () {
        unsafe {
            zts_close(fd);
        }
    }
}