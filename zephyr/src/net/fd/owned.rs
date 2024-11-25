use zephyr::raw;
use core::ffi;
use super::raw::{ RawFd, AsRawFd, IntoRawFd, FromRawFd };

#[repr(transparent)]

pub struct OwnedFd {
    fd: RawFd,
}

impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        ManuallyDrop::new(self).fd
    }
}


impl FromRawFd for OwnedFd {
    /// Constructs a new instance of `Self` from the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must be open and suitable for assuming
    /// [ownership][io-safety]. The resource must not require any cleanup other than `close`.
    ///
    /// [io-safety]: io#io-safety
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        assert_ne!(fd, u32::MAX as RawFd);
        // SAFETY: we just asserted that the value is in the valid range and isn't `-1` (the only value bigger than `0xFF_FF_FF_FE` unsigned)
        unsafe { Self { fd } }
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // Note that errors are ignored when closing a file descriptor. According to POSIX 2024,
            // we can and indeed should retry `close` on `EINTR`
            // (https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/functions/close.html),
            // but it is not clear yet how well widely-used implementations are conforming with this
            // mandate since older versions of POSIX left the state of the FD after an `EINTR`
            // unspecified. Ignoring errors is "fine" because some of the major Unices (in
            // particular, Linux) do make sure to always close the FD, even when `close()` is
            // interrupted, and the scenario is rare to begin with. If we retried on a
            // not-POSIX-compliant implementation, the consequences could be really bad since we may
            // close the wrong FD. Helpful link to an epic discussion by POSIX workgroup that led to
            // the latest POSIX wording: http://austingroupbugs.net/view.php?id=529
            
            
            //crate::sys::fs::debug_assert_fd_is_open(self.fd);
            let _ = raw::zsock_close(self.fd);
            
        }
    }
}