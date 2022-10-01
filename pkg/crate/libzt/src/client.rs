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

pub struct ZeroTierClient {
    // TODO
}

impl ZeroTierClient {

    pub fn client_create_socket(&self) -> i32 {
        pub const sock_stream: i32 = 1;
        pub const af_inet6: i32 = 10;
        unsafe {
            return zts_socket(af_inet6, sock_stream, 0);
        }
    }

    pub fn client_connect(&self, fd: i32, addr: io::Result<&SocketAddr>) -> i32 {
        let addr = addr.unwrap();
        unsafe {
            // TODO: Find a better way to split this address string
            let full_str = addr.to_string();
            let full_vec = full_str.split(":");
            let lvec: Vec<&str> = full_vec.collect();
            let addr_str = lvec[0];
            let port = addr.port();
            let timeout_ms = 0;

            println! ("socket connect: addr_str = {} port = {}", addr_str, port);
            // TODO: Handle native error code, consider cvt?
            // This is a false-positive by the linter
            // See: https://github.com/rust-lang/rust/issues/78691
            #[allow(temporary_cstring_as_ptr)]
            return zts_connect(fd, 
                CString::new(addr_str).unwrap().as_ptr(),
                port,
                timeout_ms);
        }
    }

    pub fn client_send(&self, fd: i32, buf: &[u8]) -> i64 {
        unsafe {
            return zts_send(
                fd,
                buf.as_ptr() as *const c_void,
                buf.len().try_into().unwrap(),
                0);
        }
    }

    pub fn client_close(&self, fd: i32) -> () {
        unsafe {
            zts_close(fd);
        }
    }
}