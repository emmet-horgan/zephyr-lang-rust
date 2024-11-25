pub mod raw;
pub mod owned;

use zephyr::raw;
use owned::OwnedFd;

#[derive(Debug)]
pub struct FileDesc(OwnedFd);
