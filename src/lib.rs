use std::io::prelude::*;
use std::fs;

use bit::BitIndex;

pub fn kompress(buffer: Vec<u8>) -> Vec<u8> {
    // Split the original vector into 7-byte chunks.
    // The last chunk may be shorter than the others.
    let chunks = buffer.chunks(7);

    let mut result = Vec::<u8>::new();
    for chunk in chunks {
        let mut high_bits = Vec::<bool>::new();

        // Collect the high bits
        for b in chunk {
            high_bits.push(b.bit(7));
        }

        let mut index_byte = 0u8;
        // Start from bit 0:
        for (index, value) in high_bits.iter().enumerate() {
            index_byte.set_bit(index, *value);
        }
        result.push(index_byte);

        for b in chunk {
            result.push(b & 0x7f);  // use only bits 0...6
        }
    }

    result
}

pub fn unkompress(buffer: Vec<u8>) -> Vec<u8> {
    // Split the original vector into 8-byte chunks:
    let chunks = buffer.chunks(8);

    let mut result = Vec::<u8>::new();
    for chunk in chunks {
        let index_byte = chunk[0];
        let mut index = 0;
        for b in chunk[1..].iter() {  // process bytes 1..7 of chunk
            let mut v = *b;

            // Set the top bit of this byte with the corresponding index bit
            v.set_bit(7, index_byte.bit(index));
            result.push(v);

            index += 1;
        }
    }

    result
}

pub fn read_file(name: &String) -> Option<Vec<u8>> {
    match fs::File::open(&name) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            match f.read_to_end(&mut buffer) {
                Ok(_) => Some(buffer),
                Err(_) => None
            }
        },
        Err(_) => {
            eprintln!("Unable to open file {}", &name);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kompress() {
        let unpacked: Vec<u8> = vec![101, 202, 103, 204, 105, 206, 107];
        let packed: Vec<u8> = vec![42, 101, 74, 103, 76, 105, 78, 107];
        assert_eq!(kompress(unpacked), packed);
    }

    #[test]
    fn test_unkompress() {
        let unpacked: Vec<u8> = vec![101, 202, 103, 204, 105, 206, 107];
        let packed: Vec<u8> = vec![42, 101, 74, 103, 76, 105, 78, 107];

        assert_eq!(unkompress(packed), unpacked);
    }
}
