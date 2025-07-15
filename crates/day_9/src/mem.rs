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
}

enum ScannerValue {
    File(FileID),
    Free,
    Eof,
}

struct Scanner<'a> {
    marker: usize, // set marker flags on a position, so it can be compared to later
    pointer: usize,
    memory: &'a BlockMemory,
}

impl<'a> Scanner<'a> {
    fn new(pointer: usize, memory: &'a BlockMemory) -> Self {
        Self { marker: pointer, pointer, memory }
    }
    /// does not guarantee that pointer points to a readable position
    fn inc(&mut self) -> ScannerValue {
        if self.pointer == self.memory.len() {
            return ScannerValue::Eof;
        }
        self.pointer += 1;
        if self.pointer == self.memory.len() {
            ScannerValue::Eof
        } else {
            Self::val_from_option(self.memory.blocks[self.pointer])
        }
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
}


struct FreeSpaceScanner<'a> {
    scanner: Scanner<'a>, 
}

impl<'a> FreeSpaceScanner<'a> {
    fn new(memory: &'a BlockMemory) -> Self {
        Self { scanner: Scanner::new(0, memory) }
    }

    fn next(&mut self) -> Option<(usize, usize)> {
        // seek to next freespace
        loop {
            let value = self.scanner.inc();
            if let ScannerValue::Eof = value { return None }; // reached end of memory
            if let ScannerValue::Free = value { break; } // found freespace
        }

        // remember last pointer
        self.scanner.mark();

        // move past freespace
        loop {
            match self.scanner.inc(){
                ScannerValue::Free => continue,
                _ => break,
            };
        }

        Some((self.scanner.marker, self.scanner.pointer))
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
        assert_eq!(free_space_scanner.next(), Some((1, 3)));
        assert_eq!(free_space_scanner.next(), Some((6, 10)));
        assert_eq!(free_space_scanner.next(), None);
    }
}

