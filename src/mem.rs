use std::collections::TryReserveError;
use thiserror::Error;

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

/// MemoryError is an error type that is returned by all Memory methods.
/// It is used to indicate that a memory allocation request was denied by the system,
/// or that a conversion from a larger unit to bytes caused an overflow.
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("System denied memory allocation request")]
    CannotAllocate(TryReserveError),
    #[error("The requested byte quantity is too large to fit in a usize")]
    ByteOverflow,
}

impl From<TryReserveError> for MemoryError {
    fn from(e: TryReserveError) -> Self {
        MemoryError::CannotAllocate(e)
    }
}

impl Memory {
    /// Creates a new Memory block with the specified size in bytes.
    pub fn new(size: usize) -> Result<Self, MemoryError> {
        let mut _bytes = Vec::new();
        _bytes.try_reserve_exact(size)?;
        _bytes = vec![0xFF; size];

        Ok(Self { _bytes })
    }

    /// Creates a new Memory block with the specified size in bytes.
    /// Does the same thing as Memory::new(), but is more explicit and thus more readable.
    pub fn from_bytes(bytes: usize) -> Result<Self, MemoryError> {
        Self::new(bytes)
    }

    /// Creates a new Memory block with the specified size in kilobytes.
    /// This method will return an error if the conversion from kilobytes to bytes causes an overflow.
    pub fn from_kilobytes(kilobytes: usize) -> Result<Self, MemoryError> {
        Self::new(kilobytes.checked_mul(BYTES_IN_KILOBYTE).ok_or(MemoryError::ByteOverflow)?)
    }

    /// Creates a new Memory block with the specified size in megabytes.
    /// This method will return an error if the conversion from megabytes to bytes causes an overflow.
    pub fn from_megabytes(megabytes: usize) -> Result<Self, MemoryError> {
        Self::new(megabytes.checked_mul(BYTES_IN_MEGABYTE).ok_or(MemoryError::ByteOverflow)?)
    }

    /// Creates a new Memory block with the specified size in gigabytes.
    /// This method will return an error if the conversion from gigabytes to bytes causes an overflow.
    pub fn from_gigabytes(gigabytes: usize) -> Result<Self, MemoryError> {
        Self::new(gigabytes.checked_mul(BYTES_IN_GIGABYTE).ok_or(MemoryError::ByteOverflow)?)
    }

    /// Creates a new Memory block with the specified size in terabytes.
    /// This method will return an error if the conversion from terabytes to bytes causes an overflow.
    pub fn from_terabytes(terabytes: usize) -> Result<Self, MemoryError> {
        Self::new(terabytes.checked_mul(BYTES_IN_TERABYTE).ok_or(MemoryError::ByteOverflow)?)
    }
}
