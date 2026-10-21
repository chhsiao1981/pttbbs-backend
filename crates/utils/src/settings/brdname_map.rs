use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    sync::{RwLock, TryLockError},
};

pub struct BrdnameMap<'a> {
    filename: &'a String,
    lock: RwLock<HashMap<String, i32>>,
}

impl<'a> BrdnameMap<'a> {
    pub fn new(filename: &'a String) -> BrdnameMap<'a> {
        BrdnameMap {
            filename,
            lock: RwLock::new(HashMap::new()),
        }
    }
}

impl<'a> BrdnameMap<'a> {
    pub fn try_refresh(self: &Self) -> Result<()> {
        let content = fs::read_to_string(&self.filename)?;
        let the_map: HashMap<String, i32> = deser_hjson::from_str(&content)?;

        println!(
            "BrdnameMap::try_refresh: filename: {:#?} map: {the_map:#?}",
            self.filename
        );

        let mut p_map = self.lock.try_write().map_err(|e| match e {
            TryLockError::WouldBlock => anyhow::anyhow!("RwLock write would block"),
            TryLockError::Poisoned(_) => anyhow::anyhow!("RwLock is poisoned"),
        })?;

        *p_map = the_map;

        Ok(())
    }
}

impl<'a> BrdnameMap<'a> {
    pub fn with_try_read<F, R>(self: &Self, f: F) -> Result<R>
    where
        F: FnOnce(&HashMap<String, i32>) -> R,
    {
        let the_map = self.lock.try_read().map_err(|e| match e {
            TryLockError::WouldBlock => anyhow::anyhow!("RwLock read would block"),
            TryLockError::Poisoned(_) => anyhow::anyhow!("RwLock is poisoned"),
        })?;

        // Pass the reference `&HashMap` to the closure while the guard is active
        Ok(f(&the_map))
    }
}
