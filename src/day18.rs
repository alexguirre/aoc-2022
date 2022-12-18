use std::collections::{HashSet, VecDeque};
use std::ops;

mod profiler;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos { x: i64, y: i64, z: i64 }
const fn pos(x: i64, y: i64, z: i64) -> Pos { Pos { x, y, z } }

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        pos(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        pos(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

const INPUT: &str = include_str!("data/input18");

fn get_neighbors(p: Pos) -> impl Iterator<Item=Pos> {
    [
        pos(1, 0, 0),
        pos(-1, 0, 0),
        pos(0, 1, 0),
        pos(0, -1, 0),
        pos(0, 0, 1),
        pos(0, 0, -1),
    ].into_iter()
        .map(move |offset| p + offset)
}

fn count_neighbors(set: &HashSet<Pos>, p: Pos) -> usize {
    get_neighbors(p)
        .filter(|neighbor_pos| set.contains(neighbor_pos))
        .count()
}

fn measure_surface_area(cubes: &Vec<Pos>) -> usize {
    let mut set = HashSet::new();
    let mut surface_area = 0usize;
    for cube in cubes {
        surface_area += 6;
        surface_area -= count_neighbors(&set, *cube) * 2;
        set.insert(*cube);
    }
    surface_area
}

fn get_bounding_box(cubes: &Vec<Pos>) -> (Pos, Pos) {
    let (min_x, max_x) = cubes.iter()
        .map(|p| p.x)
        .fold((i64::MAX, i64::MIN), |acc, n| (acc.0.min(n), acc.1.max(n)));
    let (min_y, max_y) = cubes.iter()
        .map(|p| p.y)
        .fold((i64::MAX, i64::MIN), |acc, n| (acc.0.min(n), acc.1.max(n)));
    let (min_z, max_z) = cubes.iter()
        .map(|p| p.z)
        .fold((i64::MAX, i64::MIN), |acc, n| (acc.0.min(n), acc.1.max(n)));

    (pos(min_x, min_y, min_z), pos(max_x, max_y, max_z))
}

fn is_inside_bounding_box(p: Pos, min: Pos, max: Pos) -> bool {
    p.x >= min.x && p.x <= max.x &&  p.y >= min.y && p.y <= max.y && p.z >= min.z && p.z <= max.z
}

fn measure_external_surface_area(cubes: &Vec<Pos>) -> usize {
    let cubes_set = cubes.iter()
        .fold(HashSet::new(), |mut set, pos| {
            set.insert(*pos); set
        });

    let (min, max) = get_bounding_box(cubes);
    // increase the bounding box a bit to allow the flood-fill to cover the whole sphere
    let (min, max) = (min - pos(1,1,1), max + pos(1,1,1));
    let dims = max - min + pos(1,1,1);
    // println!("min:{min:?}    max:{max:?}    dims:{dims:?}");

    // Flood-fill around the sphere made of the cubes list.
    // This flood-fill will form a bigger cube that encloses the sphere, its inner surface
    // equals the exterior surface of the sphere
    let mut fill_surface_area = 0usize;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(min);
    while let Some(p) = queue.pop_front() {
        if is_inside_bounding_box(p, min, max) && !cubes_set.contains(&p) && !visited.contains(&p) {
            fill_surface_area += 6;
            fill_surface_area -= count_neighbors(&visited, p) * 2;
            visited.insert(p);

            get_neighbors(p)
                .for_each(|neighbor_pos| queue.push_back(neighbor_pos));
        }
    }

    // remove surface from external faces
    let face_surface_top_bottom = ((dims.x * dims.y) * 2) as usize;
    let face_surface_front_back = ((dims.x * dims.z) * 2) as usize;
    let face_surface_left_right = ((dims.y * dims.z) * 2) as usize;
    fill_surface_area - face_surface_top_bottom - face_surface_front_back - face_surface_left_right
}

fn main() {
    let _p = profiler::profile();

    let cubes = INPUT.lines()
        .map(|l| l.split(','))
        .map(|mut n| pos(n.next().unwrap().parse().unwrap(), n.next().unwrap().parse().unwrap(), n.next().unwrap().parse().unwrap()))
        .collect::<Vec<_>>();

    let res1 = measure_surface_area(&cubes);
    let res2 = measure_external_surface_area(&cubes);

    println!("[Part 1] Result is {res1}");
    println!("[Part 2] Result is {res2}");
}
