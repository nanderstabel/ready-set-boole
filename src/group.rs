use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Group(HashSet<u32>);

impl Group {
    pub fn new() -> Self {
        Group(HashSet::new())
    }

    pub fn insert(&mut self, i: u32) -> bool {
        self.0.insert(i)
    }

    // pub fn union<'a>(&'a self, other: &'a Group) -> Group {
    //     let this = self.0.iter().cloned();
    //     let that = other.0.iter().cloned();

    //     Group(this.union(&that))
    // }
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}

impl Eq for Group {}

impl Hash for Group {
    fn hash<H>(&self, group: &mut H)
    where
        H: Hasher,
    {
        let mut a: Vec<&u32> = self.0.iter().collect();
        a.sort();
        for i in a.iter() {
            i.hash(group);
        }
    }
}

impl FromIterator<u32> for Group {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut g = Group::new();
        for i in iter {
            g.insert(i);
        }
        g
    }
}
