// I was going to implement an enum BufferCell<T> { Empty, Full(T) } but this is essentially an Option<T> type,
// and Option<T> has convenient methods like `take`, so I am instead
// using an alias to semantically differentiate between Option and BufferCell (although they are the same impl)
type BufferCell<T> = Option<T>;

#[derive(Clone, Debug, Default)]
pub struct CircularBuffer<T: core::fmt::Debug> {
    data: Vec<Option<T>>,
    capacity: usize,
    // where to write next
    next_write_pos: usize,
    // where to read next
    next_read_pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: core::fmt::Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: (0..capacity)
                .into_iter()
                .map(|_| BufferCell::None)
                .collect(),
            capacity,
            next_write_pos: 0,
            next_read_pos: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if let Some(BufferCell::Some(_)) = self.data.get(self.next_write_pos) {
            return Err(Error::FullBuffer);
        }

        self.data[self.next_write_pos] = BufferCell::Some(element);
        self.advance_write_ptr();

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(BufferCell::None) = self.data.get(self.next_read_pos) {
            return Err(Error::EmptyBuffer);
        }

        let e = self.data[self.next_read_pos].take().unwrap();
        self.advance_read_ptr();

        Ok(e)
    }

    pub fn clear(&mut self) {
        self.data = (0..self.capacity)
            .into_iter()
            .map(|_| BufferCell::None)
            .collect();
        self.next_read_pos = 0;
        self.next_write_pos = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        // Buffer full: replace the oldest element and skip past it.
        if let Some(BufferCell::Some(_)) = self.data.get(self.next_write_pos) {
            self.data[self.next_read_pos] = BufferCell::Some(element);
            self.advance_read_ptr();
            self.advance_write_ptr();
        } else {
            // Otherwise act exactly like write.
            self.data[self.next_write_pos] = BufferCell::Some(element);
            self.advance_write_ptr();
        }
    }

    fn advance_write_ptr(&mut self) {
        self.next_write_pos = (self.next_write_pos + 1) % (self.capacity);
    }

    fn advance_read_ptr(&mut self) {
        self.next_read_pos = (self.next_read_pos + 1) % (self.capacity);
    }
}
