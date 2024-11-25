// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use zephyr::printkln;
use zephyr::raw;

// Reference the Zephyr crate so that the panic handler gets used.  This is only needed if no
// symbols from the crate are directly used.
//extern crate zephyr;

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        // Define constants
        let port: u16 = 8080;
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 33\r\n\r\nHello, world from rust on zephyr!";

        // Create a socket
        let sock_fd = raw::zsock_socket(raw::AF_INET.try_into().unwrap(), 
            raw::net_sock_type_SOCK_STREAM.try_into().unwrap(), 0);
        if sock_fd < 0 {
            printkln!("Failed to create socket");
            return;
        }

        // Set socket options
        let opt_val: i32 = 1;
        if raw::zsock_setsockopt(
            sock_fd,
            raw::SOL_SOCKET.try_into().unwrap(),
            raw::SO_REUSEADDR.try_into().unwrap(),
            &opt_val as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&opt_val) as raw::socklen_t,
        ) < 0
        {
            printkln!("Failed to set socket options");
            raw::zsock_close(sock_fd);
            return;
        }

        // Bind the socket to an address
        
        let mut server_addr = raw::sockaddr_in {
            sin_family: raw::AF_INET as raw::sa_family_t,
            sin_port: port.to_be(),
            sin_addr: raw::in_addr {
                __bindgen_anon_1: raw::in_addr__bindgen_ty_1 {
                    s4_addr: raw::__BindgenUnionField::new(),
                    s4_addr16: raw::__BindgenUnionField::new(),
                    s4_addr32: raw::__BindgenUnionField::new(),
                    s_addr: raw::__BindgenUnionField::new(),
                    bindgen_union_field: 0
                }
            },
        };

        server_addr.sin_addr.__bindgen_anon_1.s4_addr.as_mut().copy_from_slice(
            &[192, 168, 1, 132]
        );

        if raw::zsock_bind(
            sock_fd,
            &server_addr as *const _ as *const raw::sockaddr,
            core::mem::size_of_val(&server_addr) as raw::socklen_t,
        ) < 0
        {
            printkln!("Failed to bind socket");
            raw::zsock_close(sock_fd);
            return;
        }

        // Start listening on the socket
        if raw::zsock_listen(sock_fd, 128) < 0 {
            printkln!("Failed to listen on socket");
            raw::zsock_close(sock_fd);
            return;
        }

        printkln!("Server listening on port {}", port);

        loop {
            // Accept a connection
            let mut client_addr = raw::sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: raw::in_addr {
                    __bindgen_anon_1: raw::in_addr__bindgen_ty_1 {
                        s4_addr: raw::__BindgenUnionField::new(),
                        s4_addr16: raw::__BindgenUnionField::new(),
                        s4_addr32: raw::__BindgenUnionField::new(),
                        s_addr: raw::__BindgenUnionField::new(),
                        bindgen_union_field: 0
                    }
                },
            };
            let mut client_addr_len = core::mem::size_of_val(&client_addr) as raw::socklen_t;

            let client_fd = raw::zsock_accept(
                sock_fd,
                &mut client_addr as *mut _ as *mut raw::sockaddr,
                &mut client_addr_len,
            );

            if client_fd < 0 {
                printkln!("Failed to accept connection");
                continue;
            }

            // Send response
            raw::zsock_send(
                client_fd,
                response.as_ptr() as *const core::ffi::c_void,
                response.len(),
                0,
            );

            // Close the client connection
            raw::zsock_close(client_fd);
        }
    }
}
