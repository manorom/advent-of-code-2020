use std::collections::HashSet;
use std::iter::{from_fn, Iterator};

fn coord_permutation3d() -> impl Iterator<Item=(i64, i64, i64)> {
    let perm = [0, 1, -1];
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    from_fn(move || {
        if z >= 3 {
            z = 0;
            y += 1;
            if y >= 3 {
                y = 0;
                x += 1;
                if x >= 3 {
                    return None;
                }
            }
        }
        let zc = z;
        z += 1;
        Some((perm[x], perm[y], perm[zc]))
    })
        .filter(|(x, y, z)| !(*x == 0 && *y == 0 && *z == 0))
}


fn coord_permutation4d() -> impl Iterator<Item=(i64, i64, i64, i64)> {
    let perm = [0, 1, -1];
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut w = 0;
    from_fn(move || {
        if w >= 3 {
            w = 0;
            z += 1;
            if z >= 3 {
                z = 0;
                y += 1;
                if y >= 3 {
                    y = 0;
                    x += 1;
                    if x >= 3 {
                        return None;
                    }
                }
            }
        }
        let wc = w;
        w += 1;
        Some((perm[x], perm[y], perm[z], perm[wc]))
    })
        .filter(|(x, y, z, w)| !(*x == 0 && *y == 0 && *z == 0 && *w == 0))
}

#[derive(Debug)]
struct Grid {
    front: HashSet<(i64, i64, i64, i64)>,
    back: HashSet<(i64, i64, i64, i64)>,
    dimensions_from: (i64, i64, i64, i64),
    dimensions_to: (i64, i64, i64, i64)
}

impl Grid {

    fn new() -> Self {
        Grid {
            front: HashSet::new(),
            back: HashSet::new(),
            dimensions_from: (0,0,0,0),
            dimensions_to: (0,0,0,0)
        }
    }
    fn commit(&mut self) {
        self.front = self.back.clone();
    }
    fn update_dimensions(&mut self) {
        for (i, j, k, h) in self.front.iter() {
            if *i < self.dimensions_from.0 {
                self.dimensions_from.0 = *i;
            }
            if *i > self.dimensions_to.0 {
                self.dimensions_to.0 = *i;
            }
            if *j < self.dimensions_from.1 {
                self.dimensions_from.1 = *j;
            }
            if *j > self.dimensions_to.1 {
                self.dimensions_to.1 = *j;
            }
            if *k < self.dimensions_from.2 {
                self.dimensions_from.2 = *k;
            }
            if *k > self.dimensions_to.2 {
                self.dimensions_to.2 = *k;
            }
            if *h < self.dimensions_from.3 {
                self.dimensions_from.3 = *h;
            }
            if *h > self.dimensions_to.3{
                self.dimensions_to.3 = *h;
            }
        }
    }
    fn round3d(&mut self) {
        for i in (self.dimensions_from.0-1)..=(self.dimensions_to.0+1) {
            for j in (self.dimensions_from.1 - 1)..=(self.dimensions_to.1 +1) {
                for k in (self.dimensions_from.2 -1)..=(self.dimensions_to.2 +1) {
                    let num_neighbors = self.lookup_num_neighbors3d(&(i,j,k));
                    if self.front.get(&(i,j,k,0)).is_some() {
                        if num_neighbors == 2 || num_neighbors == 3 {
                            self.back.insert((i, j, k, 0));
                        } else {
                            self.back.remove(&(i,j,k, 0));
                        }
                    } else {
                        if num_neighbors == 3 {
                            self.back.insert((i,j,k, 0));
                        } else {
                            self.back.remove(&(i,j,k, 0));
                        }
                    }
                }
            }
        }
        self.commit();
        self.update_dimensions();
    }
    fn round4d(&mut self) {
        for i in (self.dimensions_from.0-1)..=(self.dimensions_to.0+1) {
            for j in (self.dimensions_from.1 - 1)..=(self.dimensions_to.1 +1) {
                for k in (self.dimensions_from.2 -1)..=(self.dimensions_to.2 +1) {
                    for l in (self.dimensions_from.3 -1)..=(self.dimensions_to.3 + 1) {
                        let num_neighbors = self.lookup_num_neighbors4d(&(i,j,k,l));
                        if self.front.get(&(i,j,k,l)).is_some() {
                            if num_neighbors == 2 || num_neighbors == 3 {
                                self.back.insert((i, j, k, l));
                            } else {
                                self.back.remove(&(i,j,k, l));
                            }
                        } else {
                            if num_neighbors == 3 {
                                self.back.insert((i,j,k,l));
                            } else {
                                self.back.remove(&(i,j,k,l));
                            }
                        }
                    }
                }
            }
        }
        self.commit();
        self.update_dimensions();
    }

    fn lookup_num_neighbors3d(&self, coords: &(i64, i64, i64)) -> usize {
        coord_permutation3d()
            .map(|(x,y,z)| self.front.get(&(coords.0 + x, coords.1 + y, coords.2 + z, 0)))
            .filter(|r| r.is_some())
            .count()
    }

    fn lookup_num_neighbors4d(&self, coords: &(i64, i64, i64, i64)) -> usize {
        coord_permutation4d()
            .map(|(x,y,z,w)| self.front.get(&(coords.0 + x, coords.1 + y, coords.2 + z, coords.3 + w)))
            .filter(|r| r.is_some())
            .count()
    }

    fn parse_grid(&mut self, input: &str) {
        for (y, line) in input.split('\n').filter(|l| !l.is_empty()).enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    self.front.insert((x as i64, y as i64, 0, 0));
            
                }
            }
        }
        self.update_dimensions();
    }

    fn count_active(&self) -> usize {
        self.front.len()
    }
}

fn main() {
    let mut grid = Grid::new();
    grid.parse_grid(include_str!("input.txt"));
    dbg!(grid.count_active());
    //dbg!(coord_permutation4d().collect::<Vec<_>>());
    for _ in  0..6 {
        grid.round4d();
    }
    dbg!(grid.count_active());
    //dbg!(grid.front);
}
