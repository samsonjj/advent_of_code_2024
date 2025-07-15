type FileID = i32;

pub struct BlockMemory {
    blocks: Vec<Option<FileID>>,
}

impl BlockMemory {
    fn parse(input: &str) -> Self {
        let mut blocks = vec![];
        for (i, c) in input.chars().enumerate() {
            let is_file = i % 2 == 0;
            let value = if is_file {
                let file_id: FileID = i as FileID / 2;
                dbg!(file_id, i);
                Some(file_id)
            } else {
                None
            };

            let length = c.to_digit(10).expect("failed to parse digit from '{c}'"); // radix = 10
            for i in 0..length {
                blocks.push(value);
            }
        }
        Self {
            blocks,
        }
    }

    fn len(&self) -> usize {
        self.blocks.len()
    }

    fn compress(&mut self) {
        loop {
            let mut file_scanner = FileScanner::new(self);
            let mut freespace_scanner = FreeSpaceScanner::new(self);

            // loop over files
            'file: loop {
                let Some(file) = file_scanner.next(self) else { break; };

                // loop over freespace
                'freespace: loop {
                    let Some(freespace) = freespace_scanner.next(self) else { break; };

                    let file_len = file.1.1 - file.1.0;
                    let freespace_len = freespace.1 - freespace.0;

                    if file_len > freespace_len {
                        continue;    
                    }

                    // perform swap
                    for i in freespace.0..freespace.1 {
                        
                    }
                }
            }
        }
    }
}

enum ScannerValue {
    File(FileID),
    Free,
    Eof,
}

struct Scanner {
    marker: usize, // set marker flags on a position, so it can be compared to later
    pointer: usize,
}

impl Scanner {
    fn start() -> Self {
        Self { marker: 0, pointer: 0 }
    }

    fn end(memory: &BlockMemory) -> Self {
        let end = memory.len();
        Self { marker: end, pointer: end }
    }

    /// does not guarantee that pointer points to a readable position
    fn inc(&mut self, memory: &BlockMemory) -> ScannerValue {
        if self.pointer == memory.len() {
            return ScannerValue::Eof;
        }
        self.pointer += 1;
        if self.pointer == memory.len() {
            ScannerValue::Eof
        } else {
            Self::val_from_option(memory.blocks[self.pointer])
        }
    }

    fn peek_dec(&self, memory: &BlockMemory) -> ScannerValue {
        if self.pointer == 0 {
            return ScannerValue::Eof;
        }
        Self::val_from_option(memory.blocks[self.pointer - 1])
    }

    fn dec(&mut self, memory: &BlockMemory) -> ScannerValue {
        self.pointer -=1;
        Self::val_from_option(memory.blocks[self.pointer])
    }


    fn val_from_option(val: Option<FileID>) -> ScannerValue {
        match val {
            Some(file_id) => ScannerValue::File(file_id),
            None => ScannerValue::Free,
        }
    }

    fn mark(&mut self) {
        self.marker = self.pointer;
    }

    fn diff(&mut self) -> usize {
        (self.marker as i32 - self.pointer as i32).abs() as usize
    }

    fn read(&self, memory: &BlockMemory) -> Option<FileID> {
        memory.blocks[self.pointer]
    }
}


struct FreeSpaceScanner {
    scanner: Scanner, 
}

impl FreeSpaceScanner {
    fn new(memory: & BlockMemory) -> Self {
        Self { scanner: Scanner::start() }
    }

    fn next(&mut self, memory: &BlockMemory) -> Option<(usize, usize)> {
        // seek to next freespace
        loop {
            let value = self.scanner.inc(memory);
            if let ScannerValue::Eof = value { return None }; // reached end of memory
            if let ScannerValue::Free = value { break; } // found freespace
        }

        // remember last pointer
        self.scanner.mark();

        // move past freespace
        loop {
            match self.scanner.inc(memory) {
                ScannerValue::Free => continue,
                _ => break,
            };
        }

        Some((self.scanner.marker, self.scanner.pointer))
    }
}

struct FileScanner {
    scanner: Scanner,
}

impl FileScanner {
    fn new(memory: & BlockMemory) -> Self {
        Self { scanner: Scanner::end(memory) }
    }

    fn next(&mut self, memory: &BlockMemory) -> Option<(FileID, (usize, usize))> {
        // seek to file
        loop {
            match self.scanner.peek_dec(memory) {
                ScannerValue::Eof => return None,
                ScannerValue::Free => { self.scanner.dec(memory); } ,
                ScannerValue::File(_) => break,
            }
        }

        self.scanner.mark();

        // move to start of file
        loop {
            match self.scanner.peek_dec(memory) {
                ScannerValue::File(_) => { self.scanner.dec(memory); },
                _ => { break; },
            }
        }

        let file_id = self.scanner.read(memory).unwrap();
        Some((file_id, (self.scanner.pointer, self.scanner.marker)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_parse() {
        let memory = BlockMemory::parse("12345");
        assert_eq!(memory.blocks, vec![
            Some(0),
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
        ]);
    }

    #[test]
    fn test_free_space_scanner() {
        let memory = BlockMemory::parse("12345");
        let mut free_space_scanner = FreeSpaceScanner::new(&memory);
        assert_eq!(free_space_scanner.next(&memory), Some((1, 3)));
        assert_eq!(free_space_scanner.next(&memory), Some((6, 10)));
        assert_eq!(free_space_scanner.next(&memory), None);
        assert_eq!(free_space_scanner.next(&memory), None);
    }

    #[test]
    fn test_file_scanner() {
        let memory = BlockMemory::parse("12345");
        let mut file_scanner = FileScanner::new(&memory);
        assert_eq!(file_scanner.next(&memory), Some((2, (10, 15))));
        assert_eq!(file_scanner.next(&memory), Some((1, (3, 6))));
        assert_eq!(file_scanner.next(&memory), Some((0, (0, 1))));
        assert_eq!(file_scanner.next(&memory), None);
        assert_eq!(file_scanner.next(&memory), None);
    }
}

