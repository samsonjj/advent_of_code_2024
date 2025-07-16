type FileID = i32;

pub struct BlockMemory {
    blocks: Vec<Option<FileID>>,
}

impl BlockMemory {
    pub fn parse(input: &str) -> Self {
        let mut blocks = vec![];
        for (i, c) in input.chars().enumerate() {
            let is_file = i % 2 == 0;
            let value = if is_file {
                let file_id: FileID = i as FileID / 2;
                Some(file_id)
            } else {
                None
            };

            let length = c.to_digit(10).expect(format!("failed to parse digit from '{c}'").as_str()); // radix = 10
            for i in 0..length {
                blocks.push(value);
            }
        }
        Self {
            blocks,
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for item in self.blocks.iter() {
            let c = match item {
                Some(file_id) => ('0' as u8 + *file_id as u8) as char,
                None => '.',
            };
            s.push(c);
        }
        s
    }

    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn compress(&mut self) {
        let mut file_scanner = FileScanner::new(self);

        // loop over files
        let largest_file_id = self.blocks[self.blocks.len()-1].unwrap();
        'file:  for file_id in (0..=largest_file_id).rev() {
            let Some(file) = file_scanner.next(self, file_id) else {
                println!("breaking!");
                break;
            };
            // file 0 never needs to be moved
            if file.0 == 0 {
                break;
            }
            println!("attempting to swap file {}", file.0);
            let mut freespace_scanner = FreeSpaceScanner::new(self);

            // loop over freespace
            'freespace: loop {
                let Some(freespace) = freespace_scanner.next(self) else { break; };

                let file_len = file.1.1 - file.1.0;
                let freespace_len = freespace.1 - freespace.0;

                if file_len > freespace_len {
                    continue;    
                }

                // don't swap to the right
                if freespace.0 > file.1.0 {
                    continue;
                }

                // perform swap
                let file_id = file.0;
                println!("performing swap on file {file_id}: {} {}", freespace.0, file_len);
                for i in 0..file_len {
                    self.blocks[freespace.0 + i] = Some(file_id);
                }
                for i in file.1.0..file.1.1 {
                    self.blocks[i] = None;
                }

                continue 'file; // we swapped the file! go to the outer loop
            }
        }
    }

    pub fn checksum(&self) -> i64 {
        let mut sum = 0i64;
        for i in 0..self.blocks.len() {
            sum += self.blocks[i].unwrap_or(0) as i64 * i as i64;
        }
        sum
    }
}

#[derive(Clone, Copy, Debug)]
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

    fn next(&mut self, memory: &BlockMemory, file_id: FileID) -> Option<(FileID, (usize, usize))> {
        // seek to file
        let curr_file_id = loop {
            match self.scanner.peek_dec(memory) {
                ScannerValue::Eof => return None,
                ScannerValue::File(f) if f == file_id => break file_id,
                _ => self.scanner.dec(memory),
            };
        };

        dbg!(self.scanner.mark(), self.scanner.marker);

        // move to start of file
        loop {
            match self.scanner.peek_dec(memory) {
                ScannerValue::File(file_id) if file_id == curr_file_id => { self.scanner.dec(memory); },
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
        assert_eq!(file_scanner.next(&memory, 2), Some((2, (10, 15))));
        assert_eq!(file_scanner.next(&memory, 1), Some((1, (3, 6))));
        assert_eq!(file_scanner.next(&memory, 0), Some((0, (0, 1))));
        assert_eq!(file_scanner.next(&memory, 0), None);
        assert_eq!(file_scanner.next(&memory, 0), None);
    }

    #[test]
    fn test_checksum() {
        let memory = BlockMemory::parse("12345");
        assert_eq!(memory.checksum(), 1 * (3 + 4 + 5) + 2 * (10 + 11 + 12 + 13 + 14));
    }

    #[test]
    fn test_example() {
        let mut memory = BlockMemory::parse("2333133121414131402");
        assert_eq!(memory.to_string(), "00...111...2...333.44.5555.6666.777.888899");

        memory.print();

        dbg!(memory.to_string().len());

        let mut file_scanner = FileScanner::new(&memory);
        assert_eq!(file_scanner.next(&memory, 9), Some((9, (40, 42))));
        assert_eq!(file_scanner.next(&memory, 8), Some((8, (36, 40))));
        assert_eq!(file_scanner.next(&memory, 7), Some((7, (32, 35))));
        assert_eq!(file_scanner.next(&memory, 6), Some((6, (27, 31))));
        assert_eq!(file_scanner.next(&memory, 5), Some((5, (22, 26))));
        assert_eq!(file_scanner.next(&memory, 4), Some((4, (19, 21))));
        assert_eq!(file_scanner.next(&memory, 3), Some((3, (15, 18))));
        assert_eq!(file_scanner.next(&memory, 2), Some((2, (11, 12))));
        assert_eq!(file_scanner.next(&memory, 1), Some((1, (5, 8))));
        assert_eq!(file_scanner.next(&memory, 0), Some((0, (0, 2))));
        assert_eq!(file_scanner.next(&memory, 0), None);

        memory.compress();

        assert_eq!(memory.to_string(), "00992111777.44.333....5555.6666.....8888..");
        // assert_eq!(file_scanner.next(&memory), Some((9, (40, 42))));
        // assert_eq!(file_scanner.next(&memory), Some((8, (36, 40))));
        // assert_eq!(file_scanner.next(&memory), Some((7, (32, 35))));
        // assert_eq!(file_scanner.next(&memory), Some((6, (27, 31))));
        // assert_eq!(file_scanner.next(&memory), Some((5, (22, 26))));
        // assert_eq!(file_scanner.next(&memory), Some((4, (19, 21))));
        // assert_eq!(file_scanner.next(&memory), Some((3, (15, 18))));
        // assert_eq!(file_scanner.next(&memory), Some((2, (11, 12))));
        // assert_eq!(file_scanner.next(&memory), Some((1, (5, 8))));
        // assert_eq!(file_scanner.next(&memory), Some((0, (0, 2))));

        assert_eq!(memory.checksum(), 2858);
    }
}

