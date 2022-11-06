use std::ops::{Deref, DerefMut};
use std::{
    fmt,
    fs::File,
    io::Read,
    convert::From,
};
use core::convert::AsRef;

/// Heap memory plus offset pointer.
/// 
/// The offset keeps track of how far data was written to the Buffer.
/// Use the unsafe `increment` function to update the offset, and
/// use the `reset` function to reset the offset to 0.
pub struct Buffer<'a, T> {
    buf: &'a mut [T],
    offset: usize
}

/*
impl <'a, T> AsRef<[T]> for Buffer<'a, T> {
    fn as_ref(&self) -> &[T] {
        self.buf.deref()
    }
}
*/

impl<'a, T> From<&'a mut Vec<T>> for Buffer<'a, T>
{
    fn from(vec: &'a mut Vec<T>) -> Self {
        Buffer {buf: vec, offset: 0 }
    }
}

impl<'a, T> Buffer<'a, T>
{
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

impl<'a, T> Deref for Buffer<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'a, T> DerefMut for Buffer<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target  {
        self.as_mut_slice()
    }
}

impl<'a, T> fmt::Debug for Buffer<'a, T> where T: fmt::Debug {
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