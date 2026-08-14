use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
    inner: R,
    num_bytes_read: usize,
    num_read_ops: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            inner: wrapped,
            num_bytes_read: 0,
            num_read_ops: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes_read
    }

    pub fn reads(&self) -> usize {
        self.num_read_ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let num_bytes = self.inner.read(buf)?;
        self.num_bytes_read += num_bytes;
        self.num_read_ops += 1;

        Ok(num_bytes)
    }
}

pub struct WriteStats<W: Write> {
    inner: W,
    num_bytes_written: usize,
    num_write_ops: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            inner: wrapped,
            num_bytes_written: 0,
            num_write_ops: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes_written
    }

    pub fn writes(&self) -> usize {
        self.num_write_ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        self.num_bytes_written += bytes_written;
        self.num_write_ops += 1;

        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
