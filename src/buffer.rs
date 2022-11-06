use std::ops::{Deref, DerefMut};
use std::{
    fmt,
    fs::File,
    io::Read,
};
use core::convert::AsRef;

/// Heap memory plus offset pointer.
/// 
/// The offset keeps track of how far data was written to the Buffer.
/// Use the unsafe `increment` function to update the offset, and
/// use the `reset` function to reset the offset to 0.
pub struct Buffer<T> {
    buf: Box<[T]>,
    offset: usize
}

impl <T> AsRef<[T]> for Buffer<T> {
    fn as_ref(&self) -> &[T] {
        self.buf.deref()
    }
}

impl<T> Buffer<T>
{
    /// Constructs a new, empty `Buffer<T>` with at least the specified capacity.
    pub fn with_capacity(len: usize) -> Self {
        let mut v = Vec::with_capacity(len);
        unsafe { v.set_len(len); }
        Buffer {buf: v.into_boxed_slice(), offset: 0 }
    }
    /// Constructs a new buffer from Vec
    pub fn from_vec(vec: Vec<T>) -> Self {
        Buffer {buf: vec.into_boxed_slice(), offset: 0 }
    }
    /// Constructs a new buffer from boxed
    pub fn from_boxed(boxed: Box<[T]>) -> Self {
        Buffer {buf: boxed, offset: 0 }
    }
    /// Return length of initialised (consumed) `Buffer`
    #[inline]
    pub fn len(&self) -> usize {
        return self.offset
    }
    /// Return length of `Buffer`
    #[inline]
    pub fn capacity(&self) -> usize {
        return self.buf.len()
    }
    /// Return remaining space of `Buffer`
    #[inline]
    pub fn space(&self) -> usize {
        return self.capacity() - self.len()
    }
    /// Increment the offset
    #[inline]
    pub unsafe fn increment(&mut self, by:usize) {
        self.offset += by;
    }
    /// Resets the offset to zero.
    #[inline]
    pub fn reset(&mut self) {
        self.offset = 0;
    }
    /// Returns an unsafe mutable pointer to the remaining space
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.as_mut_ptr().add(self.offset)
    }
    /// Returns an unsafe mutable pointer to the remaining space
    #[inline]
    pub unsafe fn as_ptr(&mut self) -> *const T {
        self.buf.as_ptr().add(self.offset)
    }
    /// return slice of initialised Buffer (for reading)
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &(*self.buf)[0..self.offset]
    }
    /// return mut slice of uninitialised Buffer (for writing)
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut (*self.buf)[self.offset..]
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for Buffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target  {
        self.as_mut_slice()
    }
}

impl<T> fmt::Debug for Buffer<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.buf)
    }
}

/// Read file into buffer
pub fn read(file: &mut File, buf: &mut Buffer<u8>) -> Result<usize, std::io::Error> {
    let len = file.read(buf.as_mut_slice())?;
    //unsafe { buf.increment(len) };
    Ok(len)
}