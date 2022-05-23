use std::convert::TryInto;

// A generational unique IDs generator and storage system

// For what we're doing here, 64bits should be plenty enough
pub type Guid = u64;

// Handles a mostly-contiguous list of generational IDs
#[derive(Debug, PartialEq, Default)]
pub struct GuidManager {
    salt: u16,
    // The list of all ids that have existed and their generation
    // If they were dropped the bool is false
    ids:Vec<(bool, u16)>, 
    dropped:Vec<u32>,
}



fn guid_new(id:u32, gen:u16, salt:u16)->Guid{
    u64::from(salt)<<48 | u64::from(gen)<<32 | u64::from(id)
}

fn guid_id(id: Guid)->u32{
    (id & 0xFFFFFFFF) as u32
}

fn guid_gen(id: Guid)->u16{
    ((id >> 32) & 0xFFFF) as u16
}

fn guid_salt(id: Guid)->u16{
    ((id >> 48) & 0xFFFF) as u16
}

impl GuidManager {
    pub fn new(salt: u16) -> Self {
        GuidManager { salt: salt, ids:vec![], dropped:vec![] }
    }
    // Get a new unique value
    pub fn get(&mut self) -> Guid {
        // First try to recycle an old index
        if let Some(unused_id) = self.dropped.pop(){
            let (in_use, gen) = &mut self.ids[unused_id as usize];
            *in_use = true;
            if u16::MAX == *gen {
                panic!("Generation reached the maximum value");
            }
            *gen += 1;
            return guid_new(unused_id, *gen, self.salt);
        }
        // no index to recycle, create new one
        self.ids.push((true, 0));
        let new_id = self.ids.len()-1;
        if new_id == u32::MAX as usize{
            panic!("Number of IDs reached the maximum value");
        }
        return guid_new(new_id.try_into().unwrap(), 0, self.salt);
    }

    // Drop a value
    pub fn drop(&mut self, guid:Guid){
        if let Some((in_use, gen)) = self.ids.get_mut(guid_id(guid) as usize){
            if *in_use && *gen == guid_gen(guid){
                *in_use = false;
                self.dropped.push(guid_id(guid));
            }
        }
    }
}




// Storage backend based on guids
#[derive(Debug, PartialEq, Default)]
pub struct UidStore<T> {
    items: Vec<Option<(u16, T)>>,
    salt:u16
}

impl<T> UidStore<T> {
    pub fn new(salt:u16) -> Self {
        UidStore { items: Vec::new(), salt:salt }
    }

    pub fn insert(&mut self, id: Guid, t: T) {
        let idx = guid_id(id) as usize;

        // It makes no sense to insert if ID has an other salt
        if guid_salt(id) != self.salt{
            panic!();
        }

        // Don't overwrite existing value with insert()
        if let Some(Some((_, _))) = self.items.get(idx){
            panic!();
        }

        // Grow vec to the required size
        while idx >= self.items.len() {
            self.items.push(None);
        }
        self.items[idx] = Some((guid_gen(id), t));
    }

    pub fn get(&self, id: Guid) -> Option<&T> {
        if let Some(Some((gen, t))) = self.items.get(guid_id(id) as usize) {
            if *gen == guid_gen(id) && self.salt == guid_salt(id) {
                return Some(t);
            }
        }
        None
    }

    pub fn get_mut(&mut self, id: Guid) -> Option<&mut T> {
        if let Some(Some((gen, t))) = self.items.get_mut(guid_id(id) as usize) {
            if *gen == guid_gen(id) && self.salt == guid_salt(id) {
                return Some(t);
            }
        }
        None
    }

    pub fn drop(&mut self, id: Guid) {
        if let Some(Some((gen, _))) = self.items.get(guid_id(id) as usize) {
            if *gen == guid_gen(id) && self.salt == guid_salt(id) {
                self.items[guid_id(id) as usize] = None;
            }
        }
    }

    pub fn for_each<F>(&self, mut f: F)
    where F : FnMut(Guid, &T), {
        for (n, x) in self.items.iter().enumerate() {
            if let Some((gen, t)) = x {
                f(guid_new(n.try_into().unwrap(), *gen, self.salt), t);
            }
        }
    }

    pub fn for_each_mut<F>(&mut self, mut f: F)
    where F : FnMut(Guid, &mut T), {
        for (n, x) in self.items.iter_mut().enumerate() {
            if let Some((gen, t)) = x {
                f(guid_new(n.try_into().unwrap(), *gen, self.salt), t);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_serialize() {
        let gen:u16 = 0xf1f2;
        let salt:u16 = 0xfbfc;
        let id:u32 = 0x12345678;
        let guid = guid_new(id, gen, salt);
        assert_eq!(id, guid_id(guid));
        assert_eq!(gen, guid_gen(guid));
        assert_eq!(salt, guid_salt(guid));
    }

    // Check that two consecutive calls to the ID generator generate different IDs
    #[test]
    fn guids_are_unique() {
        let mut gen = GuidManager::new(42);
        assert!(gen.get() != gen.get());
    }

    // Check that two ID generators with different salts generate different IDs
    #[test]
    fn guids_are_unique2() {
        let mut gen1 = GuidManager::new(42);
        let mut gen2 = GuidManager::new(43);
        assert!(gen1.get() != gen2.get());
    }

    #[test]
    fn guids_reuse() {
        let mut gen = GuidManager::new(42);
        let id1 =gen.get();
        gen.drop(id1);
        let id2 = gen.get();
        // This test shall always fail, it is our contract
        assert!(id1 != id2);
        // These tests are tied to the current guid manager internals and might fail in the future
        assert!(guid_id(id1) == guid_id(id2));
        assert!(guid_gen(id1) != guid_gen(id2));
        assert!(guid_salt(id1) == guid_salt(id2));
    }

    #[test]
    fn store_insert() {
        let mut gen = GuidManager::new(42);
        let mut store :UidStore<u64> = UidStore::new(42);
        let id1 =gen.get();
        store.insert(id1, 123456);
        let id2 =gen.get();
        store.insert(id2, 1234567);
        assert_eq!(store.get(id1), Some(&123456));
        assert_eq!(store.get(id2), Some(&1234567));
    }

    #[test]
    #[should_panic]
    fn store_double_insert() {
        let mut gen = GuidManager::new(42);
        let mut store :UidStore<u64> = UidStore::new(42);
        let id1 =gen.get();
        store.insert(id1, 123456);
        store.insert(id1, 1234567);
    }

    #[test]
    #[should_panic]
    fn store_bad_salt() {
        let mut gen = GuidManager::new(42);
        let mut store :UidStore<u64> = UidStore::new(43);
        store.insert(gen.get(), 123456);
    }


    #[test]
    fn store_drop() {
        let mut gen = GuidManager::new(42);
        let mut store: UidStore<u64> = UidStore::new(42);
        let id1 =gen.get();
        store.insert(id1, 123456);
        let id2 =gen.get();
        store.insert(id2, 1234567);
        assert_eq!(store.get(id1), Some(&123456));

        store.drop(id1);
        assert_eq!(store.get(id1), None);
        gen.drop(id1);

        let id3 =gen.get();
        store.insert(id3, 12345678);
        assert_eq!(store.get(id3), Some(&12345678));
    }
}
