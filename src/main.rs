use std::collections::HashSet;
use std::time::Instant;
use std::hash::{BuildHasherDefault, BuildHasher};
use std::collections::hash_map::RandomState;
use hashers::fx_hash::FxHasher;
use hashers::fnv::{FNV1aHasher64, FNV1aHasher32};
use hashers::jenkins::spooky_hash::SpookyHasher;
use hashers::jenkins::{Lookup3Hasher, OAATHasher};
use hashers::oz::{DJB2Hasher, SDBMHasher};
use hashers::pigeon::Bricolage;

const GRID_SIZE: i32 = 800;


fn main() {
    println!("Default HashSet");
    let mut default: HashSet<Point, RandomState> = HashSet::default();
    test(&mut default);
    println!("----------");
    println!("FXHash");
    let mut fx: HashSet<Point, BuildHasherDefault<FxHasher>> = HashSet::default();
    test(&mut fx);
    println!("----------");
    println!("Fnv 64");
    let mut fnv: HashSet<Point, BuildHasherDefault<FNV1aHasher64>> = HashSet::default();
    test(&mut fnv);
    println!("----------");
    println!("Fnv 32");
    let mut fnv: HashSet<Point, BuildHasherDefault<FNV1aHasher32>> = HashSet::default();
    test(&mut fnv);
    println!("----------");
    println!("Spooky");
    let mut spooky: HashSet<Point, BuildHasherDefault<SpookyHasher>> = HashSet::default();
    test(&mut spooky);
    println!("----------");
    println!("Lookup3");
    let mut lookup3: HashSet<Point, BuildHasherDefault<Lookup3Hasher>> = HashSet::default();
    test(&mut lookup3);
    println!("----------");
    println!("OAATHasher");
    let mut lookup3: HashSet<Point, BuildHasherDefault<OAATHasher>> = HashSet::default();
    test(&mut lookup3);
    println!("----------");
    println!("DJB2Hasher");
    let mut djb2: HashSet<Point, BuildHasherDefault<DJB2Hasher>> = HashSet::default();
    test(&mut djb2);
    println!("----------");
    println!("SDBMHasher");
    let mut sdbm: HashSet<Point, BuildHasherDefault<SDBMHasher>> = HashSet::default();
    test(&mut sdbm);
    println!("----------");
    println!("Bricolage");
    let mut bricolage: HashSet<Point, BuildHasherDefault<Bricolage>> = HashSet::default();
    test(&mut bricolage);
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}


fn test(hashset: &mut HashSet<Point, impl BuildHasher>) {
    let start = Instant::now();
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            hashset.insert(Point::new(i, j));
        }
    }
    println!("Insert completed in {}s", start.elapsed().as_secs_f32());

    let start = Instant::now();
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            assert!(hashset.get(&Point::new(i, j)).is_some());
        }
    }
    println!("Retrieve completed in {}s", start.elapsed().as_secs_f32());
}
