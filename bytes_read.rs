/// # Safety
/// 
/// This is an unchecked function. The caller needs to ensure that the length of the input slice
/// and the length of the output array (`N`) are the same.
#[inline]
pub const unsafe fn slice_to_array_unchecked<T, const N: usize>(slice: &[T]) -> &[T; N] {
    let ptr = slice.as_ptr() as *const [T; N];
    // SAFETY: this is a unchecked function
    unsafe { &*ptr }
}

type BytesReadResult<T> = Result<T, (usize, usize)>;

pub trait BytesRead<'a> {
    fn read(&mut self, buf: &mut [u8]) -> BytesReadResult<()>;
    fn steal(&mut self, sz: usize) -> BytesReadResult<&'a [u8]>;
    fn read_byte(&mut self) -> Option<u8>;

    #[inline]
    fn steal_array<const N: usize>(&mut self) -> BytesReadResult<&'a [u8; N]> {
        // SAFETY: we just stealed a slice with the same length
        Ok(unsafe { slice_to_array_unchecked(self.steal(N)?) })
    }

    #[inline]
    fn read_to_array<const N: usize>(&mut self) -> BytesReadResult<[u8; N]> {
        Ok(*self.steal_array()?)
    }

    #[inline]
    fn read_to_vec(&mut self, sz: usize) -> BytesReadResult<Vec<u8>> {
        Ok(self.steal(sz)?.to_vec())
    }
}

// Originally copied from std impl Read::read_exact for &[u8]
impl<'a> BytesRead<'a> for &'a [u8] {
    fn read(&mut self, buf: &mut [u8]) -> BytesReadResult<()> {
        if buf.len() > self.len() {
            return Err((self.len(), buf.len()));
        }
        let (a, b) = self.split_at(buf.len());

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if buf.len() == 1 {
            buf[0] = a[0];
        } else {
            buf.copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }

    fn steal(&mut self, sz: usize) -> BytesReadResult<&'a [u8]> {
        if sz > self.len() {
            return Err((self.len(), sz));
        }
        let (a, b) = self.split_at(sz);

        *self = b;
        Ok(a)
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.is_empty() /* 1 > self.len() */ {
            return None /* Err((0, 1)) */;
        }
        let (a, b) = self.split_at(1);

        let byte = a[0];

        *self = b;
        Some(byte)
    }
}
