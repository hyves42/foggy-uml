pub struct UIDGenerator {
    salt: u32,
    id: u32,
}

impl UIDGenerator {
    pub fn new(salt: u32) -> Self {
        UIDGenerator { salt: salt, id: 0 }
    }
    // Get next unique value
    pub fn get(&mut self) -> u64 {
        self.id += 1;
        return u64::from(self.salt) << 32 | u64::from(self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check that two consecutive calls to the ID generator generate different IDs
    #[test]
    fn ids_are_unique() {
        let mut gen = UIDGenerator::new(42);
        assert!(gen.get() != gen.get());
    }

    // Check that two ID generators with different salts generate different IDs
    #[test]
    fn ids_are_unique2() {
        let mut gen1 = UIDGenerator::new(42);
        let mut gen2 = UIDGenerator::new(43);
        assert!(gen1.get() != gen2.get());
    }
}
