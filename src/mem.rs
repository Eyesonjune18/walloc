use std::collections::TryReserveError;

const BYTES_IN_KILOBYTE: usize = 1024;
const BYTES_IN_MEGABYTE: usize = BYTES_IN_KILOBYTE * 1024;
const BYTES_IN_GIGABYTE: usize = BYTES_IN_MEGABYTE * 1024;
const BYTES_IN_TERABYTE: usize = BYTES_IN_GIGABYTE * 1024;

/// Memory is a struct that represents a block of memory.
/// The memory is filled with 0xFF bytes (all bits set to 1).
/// It does nothing other than take up a specified amount of memory.
/// It can be used to simulate large amounts of memory utilization.
/// 
/// You can create a new Memory instance by using the `new` method, or by using
/// One of the "from" methods, which allow you to create blocks without having to
/// deal with unit conversions to bytes.
pub struct Memory {
    _bytes: Vec<u8>,
}

impl Memory {
    /// Creates a new Memory block with the specified size in bytes.
    pub fn new(size: usize) -> Result<Self, TryReserveError> {
        let mut _bytes = Vec::new();
        _bytes.try_reserve_exact(size)?;
        _bytes = vec![0xFF; size];

        Ok(Self { _bytes })
    }

    /// Creates a new Memory block with the specified size in bytes.
    /// Does the same thing as Memory::new(), but is more explicit and thus more readable.
    pub fn from_bytes(bytes: usize) -> Result<Self, TryReserveError> {
        Self::new(bytes)
    }

    /// Creates a new Memory block with the specified size in kilobytes.
    pub fn from_kilobytes(kilobytes: usize) -> Result<Self, TryReserveError> {
        Self::new(kilobytes * BYTES_IN_KILOBYTE)
    }

    /// Creates a new Memory block with the specified size in megabytes.
    pub fn from_megabytes(megabytes: usize) -> Result<Self, TryReserveError> {
        Self::new(megabytes * BYTES_IN_MEGABYTE)
    }

    /// Creates a new Memory block with the specified size in gigabytes.
    pub fn from_gigabytes(gigabytes: usize) -> Result<Self, TryReserveError> {
        Self::new(gigabytes * BYTES_IN_GIGABYTE)
    }

    /// Creates a new Memory block with the specified size in terabytes.
    pub fn from_terabytes(terabytes: usize) -> Result<Self, TryReserveError> {
        Self::new(terabytes * BYTES_IN_TERABYTE)
    }
}
