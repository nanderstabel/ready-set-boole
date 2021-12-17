use crate::group::Group;
use crate::truthtable::TruthTable;
use rsb::*;

use std::collections::HashSet;
use std::fmt;

pub struct KMap {
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
            map: (0..y)
                .map(|j| gray_code(j))
                .map(|j| {
                    (0..x)
                        .map(|i| gray_code(i))
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

        for x in 0..self.map.len() {
            let mut v: Vec<(u32, bool)> = Vec::new();
            for y in 0..self.map[0].len() {
                v.push(self.map[y][x]);
            }
            transpose.push(v);
        }
        KMap { map: transpose }
    }

    fn find_groups(&mut self, j: usize, i: usize) -> Group {
        let mut set = vec![self.map[j][i].0];
        for i2 in (i + 1)..(i + 4) {
            if self.map[j][i2 % 4].1 == false {
                break;
            }
            set.push(self.map[j][i2 % 4].0);
        }
        if set.len() == 3 {
            set.truncate(2);
        }
        if set.len() == 2 {
            for j2 in (j + 1)..(j + 4) {
                if self.map[j2 % 4][i].1 == false || self.map[j2 % 4][(i + 1) % 4].1 == false {
                    break;
                }
                set.push(self.map[j2 % 4][i].0);
                set.push(self.map[j2 % 4][(i + 1) % 4].0);
            }
        }
        if set.len() == 6 {
            set.truncate(4);
        }
        set.into_iter().collect()
    }

    fn get_groups(&mut self) -> HashSet<Group> {
        let mut sets: HashSet<Group> = HashSet::new();
        for j in 0..4 {
            //TODO: make dynamic
            for i in 0..4 {
                //TODO: make dynamic
                if self.map[j][i].1 == true {
                    sets.insert(vec![self.map[j][i].0].into_iter().collect());
                    sets.insert(self.find_groups(j, i));
                    sets.insert(self.get_transpose().find_groups(i, j));
                }
            }
        }
        sets
    }

    pub fn get_minterms(&mut self) -> HashSet<Group> {
        let groups = self.get_groups();

        // let mut union = Group::new();
        // for group in groups.iter() {
        //     union = union.union(group);
        // }
        // println!("{:?}", union);

        groups
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
