use zephyr::raw;
use core::ffi;

pub type RawFd = ffi::c_int;

pub trait AsRawFd {
    fn as_raw_fd(&self) -> RawFd;
}

pub trait FromRawFd {
    
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

pub trait IntoRawFd {

    fn into_raw_fd(self) -> RawFd;
}

impl AsRawFd for RawFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        *self
    }
}

impl IntoRawFd for RawFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self
    }
}

impl FromRawFd for RawFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> RawFd {
        fd
    }
}