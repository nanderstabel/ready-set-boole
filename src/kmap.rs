use crate::group::Group;
use crate::truthtable::TruthTable;
use crate::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;

pub struct KMap {
    x: usize,
    y: usize,
    map: Vec<Vec<(u32, bool)>>,
}

impl KMap {
    pub fn from(table: TruthTable) -> Self {
        let (y, x) = match table.variables.len() {
            2 => (2, 2),
            3 => (4, 2),
            4 => (4, 4),
            _ => panic!("Not implemented yet"),
        };
        KMap {
            x,
            y,
            map: (0..y)
                .map(|j| gray_code(j as u32))
                .map(|j| {
                    (0..x)
                        .map(|i| gray_code(i as u32))
                        .map(|i| {
                            (
                                (i + (j << x / 2)),
                                table.results[(i + (j << x / 2)) as usize],
                            )
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn get_transpose(&mut self) -> KMap {
        let mut transpose: Vec<Vec<(u32, bool)>> = Vec::new();

        for x in 0..self.x {
            let mut v: Vec<(u32, bool)> = Vec::new();
            for y in 0..self.y {
                v.push(self.map[y][x]);
            }
            transpose.push(v);
        }
        KMap {
            x: self.y,
            y: self.x,
            map: transpose,
        }
    }

    fn find_groups(&mut self, j: usize, i: usize) -> Group {
        let (x, y) = (self.x, self.y);
        let mut set = vec![self.map[j][i].0];
        for i2 in (i + 1)..(i + x) {
            if self.map[j][i2 % x].1 == false {
                break;
            }
            set.push(self.map[j][i2 % x].0);
        }
        if set.len() == 3 {
            set.truncate(2);
        }
        if set.len() == 2 {
            for j2 in (j + 1)..(j + y) {
                if self.map[j2 % y][i].1 == false || self.map[j2 % y][(i + 1) % x].1 == false {
                    break;
                }
                set.push(self.map[j2 % y][i].0);
                set.push(self.map[j2 % y][(i + 1) % x].0);
            }
        }
        if set.len() == 6 {
            set.truncate(4);
        }
        set.into_iter().collect()
    }

    fn get_groups(&mut self) -> HashSet<Group> {
        let mut sets: HashSet<Group> = HashSet::new();
        for j in 0..self.y {
            for i in 0..self.x {
                if self.map[j][i].1 == true {
                    sets.insert(vec![self.map[j][i].0].into_iter().collect());
                    sets.insert(self.find_groups(j, i));
                    sets.insert(self.get_transpose().find_groups(i, j));
                }
            }
        }
        sets
    }

    pub fn get_minterms(&mut self) -> Option<Vec<Group>> {
        let groups = self.get_groups();
        let mut u = Group::new();
        for group in groups.iter() {
            u = u.union(&group);
        }
        let mut collection = groups.iter().collect::<Vec<_>>();
        collection.sort_by(|a, b| b.len().cmp(&a.len()));
        for count in 2..collection.len() {
            for combination in collection.iter().permutations(count) {
                let mut union = Group::new();
                for group in &combination {
                    union = union.union(&group);
                }
                if union.len() == u.len() {
                    return Some(combination.iter().map(|c| Group(c.0.clone())).collect());
                }
            }
        }
        None
    }
}

impl fmt::Display for KMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in &self.map {
            for (bit, b) in y {
                write!(f, " {:2}:{:5} ", bit, b)?;
                // write!(f, " {:04b}:{:5} ", bit, b)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
