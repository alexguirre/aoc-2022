use std::collections::HashSet;

mod profiler;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos { x: i64, y: i64 }
const fn pos(x: i64, y: i64) -> Pos { Pos { x, y } }

const fn manhattan(a: Pos, b: Pos) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    pos: Pos,
    closest_beacon: Pos,
    dist_to_closest_beacon: i64,
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input15");
    let sensors = input
        .lines()
        .map(|l| {
            fn parse_pos(s: &str) -> Option<Pos> {
                let (xs, ys) = s.split_once(", ")?;
                Some(pos(
                    xs.split_once("=")?.1.parse().ok()?,
                    ys.split_once("=")?.1.parse().ok()?,
                ))
            }

            let sensor_pos = parse_pos(l.trim_start_matches("Sensor at ").split_once(":")?.0)?;
            let closest_beacon = parse_pos(l.rsplit_once(" at ")?.1)?;
            Some(Sensor {
                pos: sensor_pos,
                closest_beacon,
                dist_to_closest_beacon: manhattan(sensor_pos, closest_beacon),
            })
        }).collect::<Option<Vec<_>>>().unwrap();

    let (min_x, max_x) = sensors.iter()
        .flat_map(|l| [l.pos.x - l.dist_to_closest_beacon + 1, l.pos.x + l.dist_to_closest_beacon - 1])
        .fold((i64::MAX, i64::MIN), |acc, n| (acc.0.min(n), acc.1.max(n)));

    // println!("{:#?}", sensors);
    // println!("{:?}", (min_x, max_x));

    const Y: i64 = 2000000;
    let num_beacons_at_y = sensors.iter()
        .filter_map(|s| if s.closest_beacon.y == Y { Some(s.closest_beacon.x) } else { None })
        .fold(HashSet::new(), |mut acc, n| {
            acc.insert(n); acc
        }).len();

    let res1 = (min_x..=max_x)
        .filter(|&x| {
            let p = pos(x, Y);
            sensors.iter().any(|s| manhattan(s.pos, p) <= s.dist_to_closest_beacon)
        })
        .count() - num_beacons_at_y;

    const SIZE: i64 = 4000000;
    let sensors2 = sensors.clone();
    let mut distress_beacon = None;
    'outer: for s in &sensors {
        let r = s.dist_to_closest_beacon + 1;
        for x in -r..=r {
            let h = (r - x).abs();
            for p in [
                pos(s.pos.x + x, s.pos.y - h),
                pos(s.pos.x + x, s.pos.y + h),
            ] {
                if (0..=SIZE).contains(&p.x) && (0..=SIZE).contains(&p.y) &&
                    sensors2.iter().all(|s| manhattan(s.pos, p) > s.dist_to_closest_beacon) {
                    distress_beacon = Some(p);
                    break 'outer;
                }
            }
        }
    }
    let distress_beacon = distress_beacon.unwrap();
    let res2 = distress_beacon.x * 4000000 + distress_beacon.y;

    println!("[Part 1] Result is {res1}");
    println!("[Part 2] Result is {res2}");
}
