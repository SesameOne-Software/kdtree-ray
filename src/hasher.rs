use core::hash::{BuildHasherDefault, Hasher};
use hashbrown::HashMap;

#[derive(Default)]
#[repr(transparent)]
pub struct SimpleHasher(u64);

// https://github.com/manuelgustavo/cx_hash/blob/master/cx_hash/cx_hash.cpp
impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0 = 0xcbf29ce484222325;

        for b in bytes {
            self.0 ^= *b as u64;
            self.0 *= 0x00000100000001b3;
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

pub type SimpleBuildHasher = BuildHasherDefault<SimpleHasher>;
