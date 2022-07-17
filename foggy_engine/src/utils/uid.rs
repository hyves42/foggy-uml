use std::convert::TryInto;
use derive_more::{From, Into};

// A generational unique IDs generator and storage system

// A typical type definition for generational UIDs
// It is advised to define your own guid types when using generational data store to take advantage of static type checking
#[derive(Hash, Eq, Debug, PartialEq, From, Into, Copy, Clone)]
pub struct Guid(u64);

// Handles a mostly-contiguous list of generational IDs
#[derive(Debug, PartialEq, Default)]
pub struct GuidManager<U> {
    // The list of all ids that have existed and their generation
    // If they were dropped the bool is false
    ids:Vec<(bool, u32)>, 
    dropped:Vec<u32>,
    yo:Option<U>//FIXME
}


impl<U> GuidManager<U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new() -> Self {
        GuidManager { ids:vec![], dropped:vec![], yo:None }
    }
    // Get a new unique value
    pub fn get(&mut self) -> U {
        // First try to recycle an old index
        if let Some(unused_id) = self.dropped.pop(){
            let (in_use, gen) = &mut self.ids[unused_id as usize];
            *in_use = true;
            if u32::MAX == *gen {
                panic!("Generation reached the maximum value");
            }
            *gen += 1;
            return Self::guid_new(unused_id, *gen);
        }
        // no index to recycle, create new one
        self.ids.push((true, 0));
        let new_id = self.ids.len()-1;
        if new_id == u32::MAX as usize{
            panic!("Number of IDs reached the maximum value");
        }
        return Self::guid_new(new_id.try_into().unwrap(), 0);
    }

    // Drop a value
    pub fn drop(&mut self, guid:U){
        if let Some((in_use, gen)) = self.ids.get_mut(Self::guid_id(guid) as usize){
            if *in_use && *gen == Self::guid_gen(guid){
                *in_use = false;
                self.dropped.push(Self::guid_id(guid));
            }
        }
    }

    fn guid_new(id:u32, gen:u32)->U{
        U::from(u64::from(gen)<<32 | u64::from(id))
    }

    fn guid_id(id: U)->u32{
        let i:u64 = id.into();
        (i & 0xFFFFFFFF) as u32
    }

    fn guid_gen(id: U)->u32{
        let i:u64 = id.into();
        ((i >> 32) & 0xFFFFFFFF) as u32
    }

}




// Storage backend based on guids
#[derive(Debug, PartialEq, Default)]
pub struct UidStore<T,U> {
    items: Vec<Option<(u32, T)>>,
    yo:Option<U>//FIXME
}

// Iterate over children elements directly
pub struct UidStoreIterator<'a, T, U> {
    store: &'a UidStore<T, U>,
    id:Option<usize>
}

impl<T,U> UidStore<T,U>
where U:From<u64>, U:Into<u64>, U:Copy {
    pub fn new() -> Self {
        UidStore { items: Vec::new() , yo:None }
    }

    pub fn insert(&mut self, id: U, t: T)->Result<U,&str> {
        let idx = GuidManager::guid_id(id) as usize;

        // Don't overwrite existing value with insert()
        if let Some(Some((_, _))) = self.items.get(idx){
            return Err("Index is in use");
        }

        // Grow vec to the required size
        while idx >= self.items.len() {
            self.items.push(None);
        }
        self.items[idx] = Some((GuidManager::guid_gen(id), t));
        return Ok(id);
    }

    pub fn get(&self, id: U) -> Option<&T> {
        if let Some(Some((gen, t))) = self.items.get(GuidManager::guid_id(id) as usize) {
            if *gen == GuidManager::guid_gen(id) {
                return Some(t);
            }
        }
        None
    }

    pub fn get_mut(&mut self, id: U) -> Option<&mut T> {
        if let Some(Some((gen, t))) = self.items.get_mut(GuidManager::guid_id(id) as usize) {
            if *gen == GuidManager::guid_gen(id) {
                return Some(t);
            }
        }
        None
    }

    pub fn drop(&mut self, id: U) {
        if let Some(Some((gen, _))) = self.items.get(GuidManager::guid_id(id) as usize) {
            if *gen == GuidManager::guid_gen(id) {
                self.items[GuidManager::guid_id(id) as usize] = None;
            }
        }
    }

    pub fn clear(&mut self){
        self.items.clear();
    }

    pub fn for_each<F>(&self, mut f: F)
    where F : FnMut(U, &T), {
        for (n, x) in self.items.iter().enumerate() {
            if let Some((gen, t)) = x {
                f(GuidManager::guid_new(n.try_into().unwrap(), *gen), t);
            }
        }
    }

    pub fn for_each_mut<F>(&mut self, mut f: F)
    where F : FnMut(U, &mut T), {
        for (n, x) in self.items.iter_mut().enumerate() {
            if let Some((gen, t)) = x {
                f(GuidManager::guid_new(n.try_into().unwrap(), *gen), t);
            }
        }
    }

    pub fn iter(&self)
    -> UidStoreIterator<T,U>
    {
        UidStoreIterator::new(&self)
    }
}



impl <'a, T, U> UidStoreIterator<'a, T, U> {
    pub fn new(store :&'a UidStore<T, U>) -> Self {
        UidStoreIterator {
            store,
            id: None
        }
    }
}

impl <'a, T, U> Iterator for UidStoreIterator<'a, T, U>
where U:From<u64>, U:Into<u64>, U:Copy {
    type Item = (U, &'a T);

    fn next(&mut self) -> Option<(U, &'a T)> {
        let mut i = match self.id{
            None =>0,
            Some(id)=>id+1
        };  

        while i < self.store.items.len() {
            if let Some(Some((gen, t))) = self.store.items.get(i) {
                let uid:U = GuidManager::guid_new(i.try_into().unwrap(), *gen);
                self.id = Some(i);
                return Some((uid, t));
            }
            i+=1;
        }
        return None;
    }    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_serialize() {
        let gen:u32 = 0xf1f2;
        let id:u32 = 0x12345678;
        let guid:Guid = GuidManager::guid_new(id, gen);
        assert_eq!(id, GuidManager::guid_id(guid));
        assert_eq!(gen, GuidManager::guid_gen(guid));
    }

    // Check that two consecutive calls to the ID generator generate different IDs
    #[test]
    fn guids_are_unique() {
        let mut gen:GuidManager<Guid> = GuidManager::new();
        assert!(gen.get() != gen.get());
    }

    #[test]
    fn guids_reuse() {
        let mut gen:GuidManager<Guid> = GuidManager::new();
        let id1 =gen.get();
        gen.drop(id1);
        let id2 = gen.get();
        // This test shall always fail, it is our contract
        assert!(id1 != id2);
        // These tests are tied to the current guid manager internals and might fail in the future
        assert!(GuidManager::guid_id(id1) == GuidManager::guid_id(id2));
        assert!(GuidManager::guid_gen(id1) != GuidManager::guid_gen(id2));
    }

    #[test]
    fn store_insert() {
        let mut gen:GuidManager<Guid> = GuidManager::new();
        let mut store :UidStore<u64, Guid> = UidStore::new();
        let id1 =gen.get();
        store.insert(id1, 123456).unwrap();
        let id2 =gen.get();
        store.insert(id2, 1234567).unwrap();
        assert_eq!(store.get(id1), Some(&123456));
        assert_eq!(store.get(id2), Some(&1234567));
    }

    #[test]
    fn store_double_insert() {
        let mut gen:GuidManager<Guid> = GuidManager::new();
        let mut store :UidStore<u64, Guid> = UidStore::new();
        let id1 =gen.get();
        assert!(store.insert(id1, 123456).is_ok());
        assert!(store.insert(id1, 1234567).is_err());
    }


    #[test]
    fn store_drop() {
        let mut gen:GuidManager<Guid> = GuidManager::new();
        let mut store: UidStore<u64, Guid> = UidStore::new();
        let id1 =gen.get();
        store.insert(id1, 123456).unwrap();
        let id2 =gen.get();
        store.insert(id2, 1234567).unwrap();
        assert_eq!(store.get(id1), Some(&123456));

        store.drop(id1);
        assert_eq!(store.get(id1), None);
        gen.drop(id1);

        let id3 =gen.get();
        store.insert(id3, 12345678).unwrap();
        assert_eq!(store.get(id3), Some(&12345678));
    }


    #[test]
    fn store_iter() {
        let mut gen:GuidManager<Guid> = GuidManager::new();
        let mut store: UidStore<u64, Guid> = UidStore::new();
        let id1 =gen.get();
        store.insert(id1, 123456).unwrap();
        let id2 =gen.get();
        store.insert(id2, 1234567).unwrap();
        let id3 =gen.get();
        store.insert(id3, 12345678).unwrap();
        store.drop(id2);
        gen.drop(id2);
        let id4 =gen.get();
        store.insert(id4, 743567).unwrap();

        let mut it = store.iter();
        assert_eq!(it.next(), Some((id1, &123456)));
        assert_eq!(it.next(), Some((id4, &743567)));
        assert_eq!(it.next(), Some((id3, &12345678)));
        assert_eq!(it.next(), None);
    }
}
