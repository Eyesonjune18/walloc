const BYTES_IN_KILOBYTE: usize = 1024;
const BYTES_IN_MEGABYTE: usize = BYTES_IN_KILOBYTE * 1024;
const BYTES_IN_GIGABYTE: usize = BYTES_IN_MEGABYTE * 1024;
const BYTES_IN_TERABYTE: usize = BYTES_IN_GIGABYTE * 1024;

pub struct Memory {
    _bytes: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            _bytes: vec![0xFF; size],
        }
    }

    pub fn from_bytes(bytes: usize) -> Self {
        Self::new(bytes)
    }

    pub fn from_kilobytes(kilobytes: usize) -> Self {
        Self::new(kilobytes * BYTES_IN_KILOBYTE)
    }

    pub fn from_megabytes(megabytes: usize) -> Self {
        Self::new(megabytes * BYTES_IN_MEGABYTE)
    }

    pub fn from_gigabytes(gigabytes: usize) -> Self {
        Self::new(gigabytes * BYTES_IN_GIGABYTE)
    }

    pub fn from_terabytes(terabytes: usize) -> Self {
        Self::new(terabytes * BYTES_IN_TERABYTE)
    }
}
