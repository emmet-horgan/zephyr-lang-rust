pub mod fd;
use core::net::{
    AddrParseError,
    Ipv4Addr, Ipv6Addr,
    SocketAddrV4, SocketAddrV6,
    IpAddr,
    SocketAddr
};
use core::ffi;
use fd::FileDesc;
use crate::error::{ Result, Error, to_result };

pub struct Socket(FileDesc);

impl Socket {
    pub fn new(addr: &SocketAddr, ty: ffi::c_int) -> Result<Socket> {
        let fam = match *addr {
            SocketAddr::V4(..) => raw::AF_INET as ffi::c_int,
            SocketAddr::V6(..) => raw::AF_INET6 as fff::c_int,
        };
        Socket::new_raw(fam, ty)
    }

    pub fn new_raw(fam: c_int, ty: c_int) -> Result<Socket> {
        unsafe {
            let fd = to_result(raw::zsock_socket(fam, ty, 0))?;
            let socket = Socket(FileDesc::from_raw_fd(fd));
            Ok(socket)
        }
    }

    pub fn connect(&self, addr: &SocketAddr) -> Result<()> {
        let (addr, len) = addr.into_inner();
        loop {
            let result = unsafe { raw::zsock_connect(self.as_raw_fd(), addr.as_ptr(), len) };
            if result < 0 {
                match result {
                    -1 * (raw::EINTR as ffi::c_int) => continue,
                    -1 * (raw::EISCONN as ffi::c_int) => return Ok(()),
                    _ => return Err(Error(result)),
                }
            }
            return Ok(());
        }
    }
}