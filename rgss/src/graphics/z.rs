use std::collections::BTreeMap;
use std::time::Instant;

use crate::{ViewportKey, WindowKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Z {
    value: i32,
    creation_time: Instant,
}

#[derive(Debug, Default)]
pub struct ZList {
    // TODO benchmark replacing with Vec<T> and using a dirty flag
    tree_map: BTreeMap<Z, Drawable>,
}

#[derive(Clone, Copy, Debug)]
pub enum Drawable {
    Viewport(ViewportKey),
    Window(WindowKey),
}

impl PartialOrd for Z {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Z {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .cmp(&other.value)
            .then(self.creation_time.cmp(&other.creation_time))
    }
}

impl Z {
    pub fn new(z: i32) -> Self {
        Self {
            value: z,
            creation_time: Instant::now(),
        }
    }

    pub fn value(self) -> i32 {
        self.value
    }

    pub fn update_value(self, value: i32) -> Self {
        Self { value, ..self }
    }
}

impl ZList {
    pub fn new() -> Self {
        Self {
            tree_map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, z: Z, value: Drawable) {
        let old = self.tree_map.insert(z, value);
        debug_assert!(old.is_none())
    }

    pub fn re_insert(&mut self, old_z: Z, new_z: Z) {
        let value = self.remove(old_z).expect("invalid z");
        self.insert(new_z, value)
    }

    pub fn get(&self, z: Z) -> Option<&Drawable> {
        self.tree_map.get(&z)
    }

    pub fn get_mut(&mut self, z: Z) -> Option<&mut Drawable> {
        self.tree_map.get_mut(&z)
    }

    pub fn remove(&mut self, z: Z) -> Option<Drawable> {
        self.tree_map.remove(&z)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Z, &Drawable)> {
        self.tree_map.iter()
    }

    pub fn retain(&mut self, f: impl FnMut(&Z, &mut Drawable) -> bool) {
        self.tree_map.retain(f)
    }
}
