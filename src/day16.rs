use std::collections::{HashMap, HashSet, VecDeque};

mod profiler;

#[derive(Debug, Clone)]
struct Valve {
    id: u16,
    rate: u64,
    connections: Vec<u16>,
}

impl Valve {

    fn parse(s: &str) -> Option<Valve> {
        let id = Self::parse_id(s.split_once("Valve ")?.1.as_bytes());
        let rate_str = s.split_once("rate=")?.1.split_once(';')?.0;
        let rate = rate_str.parse::<u64>().ok()?;
        let connections = s.rsplit(", ")
            .map(|c| {
                let c = c.as_bytes();
                let id_str = [c[c.len() - 2], c[c.len() - 1]];
                Self::parse_id(&id_str)
            })
            .collect::<Vec<u16>>();

        Some(Valve {
            id,
            rate,
            connections,
        })
    }

    fn parse_id(id_str: &[u8]) -> u16 {
        ((id_str[0] as u16) << 8) | id_str[1] as u16
    }
}

#[allow(dead_code)]
fn id_str(id: u16) -> String {
    let c = [((id >> 8) & 0xFF) as u8, (id & 0xFF) as u8];
    std::str::from_utf8(&c).unwrap().into()
}

fn pathfind_bfs(map: &HashMap<u16, Valve>, start: u16, end: u16) -> Option<Vec<u16>> {
    type Step = (u16, Vec<u16>);

    let mut visited: HashSet<u16> = HashSet::new();
    let mut queue = VecDeque::<Step>::new();
    visited.insert(start);
    queue.push_back((start, Vec::new()));
    while let Some((curr, path)) = queue.pop_front() {
        if curr == end {
            return Some(path);
        }
        for conn in &map.get(&curr)?.connections  {
            if !visited.contains(&conn) {
                let mut new_path = path.clone();
                new_path.push(*conn);
                queue.push_back((*conn, new_path));
            }
        };
    }

    return None;
}

type ValvePaths = HashMap<(u16, u16), Option<Vec<u16>>>;
fn build_paths(map: &HashMap<u16, Valve>) -> ValvePaths {
    let ids = map.keys().collect::<Vec<_>>();
    let mut paths = ValvePaths::new();
    for from_idx in 0..ids.len() {
        for to_idx in 0..ids.len() {
            if from_idx != to_idx {
                let from = *ids[from_idx];
                let to = *ids[to_idx];
                let path = pathfind_bfs(&map, from, to);
                paths.insert((from, to), path);
            }
        }
    }
    paths
}

fn explore(map: &HashMap<u16, Valve>) -> u64 {
    fn explore_rec(map: &HashMap<u16, Valve>, paths: &ValvePaths, from: u16, open_valves: &mut HashSet<u16>, time_remaining: u64) -> u64 {
        let valve = map.get(&from).unwrap();

        let mut total_pressure_released = 0u64;
        if valve.rate > 0 {
            total_pressure_released += time_remaining * valve.rate;
            open_valves.insert(from);
        }

        if time_remaining == 0 {
            return total_pressure_released;
        }

        let mut best_pressure_released = 0u64;
        let mut best_target_valve_id = None;
        for &target_valve_id in map.keys() {
            if from == target_valve_id || open_valves.contains(&target_valve_id) {
                continue;
            }

            let Some(path) = paths.get(&(from, target_valve_id)).unwrap() else {
                continue;
            };

            let path_and_open_time = path.len() as u64 + 1;
            if time_remaining < path_and_open_time {
                continue;
            }

            let target_valve = map.get(&target_valve_id).unwrap();
            if target_valve.rate == 0 {
                continue;
            }

            let pressure_released = explore_rec(map, paths, target_valve_id, &mut open_valves.clone(), time_remaining - path_and_open_time);
            if pressure_released > best_pressure_released {
                best_pressure_released = pressure_released;
                best_target_valve_id = Some(target_valve_id);
            }
        }

        if let Some(_) = best_target_valve_id {
            total_pressure_released += best_pressure_released;
        }

        total_pressure_released
    }

    explore_rec(map, &build_paths(map), Valve::parse_id("AA".as_bytes()), &mut HashSet::new(), 30)
}

fn explore2(map: &HashMap<u16, Valve>) -> u64 {
    fn explore_rec(map: &HashMap<u16, Valve>, paths: &ValvePaths, from1: u16, from2: u16, open_valves: &mut HashSet<u16>, time_remaining1: u64, time_remaining2: u64) -> u64 {
        let valve1 = map.get(&from1).unwrap();
        let valve2 = map.get(&from2).unwrap();

        let mut total_pressure_released = 0u64;
        if time_remaining1 != 0 && valve1.rate > 0 {
            total_pressure_released += time_remaining1 * valve1.rate;
            open_valves.insert(from1);
        }
        if valve1.id != valve2.id && time_remaining2 != 0 && valve2.rate > 0 {
            total_pressure_released += time_remaining2 * valve2.rate;
            open_valves.insert(from2);
        }

        if time_remaining1 == 0 && time_remaining2 == 0 {
            return total_pressure_released;
        }

        let mut best_pressure_released = 0u64;
        let mut best_target_valve_id = None;
        if time_remaining1 > 0 && time_remaining2 > 0 { // both me and elephant move
            for &target_valve_id1 in map.keys() {
                if from1 == target_valve_id1 || open_valves.contains(&target_valve_id1) {
                    continue;
                }

                let Some(path1) = paths.get(&(from1, target_valve_id1)).unwrap() else {
                    continue;
                };

                let path_and_open_time1 = path1.len() as u64 + 1;
                if time_remaining1 < path_and_open_time1 {
                    continue;
                }

                let target_valve1 = map.get(&target_valve_id1).unwrap();
                if target_valve1.rate == 0 {
                    continue;
                }

                for &target_valve_id2 in map.keys() {
                    if from2 == target_valve_id2 || target_valve_id1 == target_valve_id2 || open_valves.contains(&target_valve_id2) {
                        continue;
                    }

                    let Some(path2) = paths.get(&(from2, target_valve_id2)).unwrap() else {
                        continue;
                    };

                    let path_and_open_time2 = path2.len() as u64 + 1;
                    if time_remaining2 < path_and_open_time2 {
                        continue;
                    }

                    let target_valve2 = map.get(&target_valve_id2).unwrap();
                    if target_valve2.rate == 0 {
                        continue;
                    }

                    let pressure_released = explore_rec(map, paths,
                                                        target_valve_id1, target_valve_id2,
                                                        &mut open_valves.clone(),
                                                        time_remaining1 - path_and_open_time1,
                                                        time_remaining2 - path_and_open_time2);
                    if pressure_released > best_pressure_released {
                        best_pressure_released = pressure_released;
                        best_target_valve_id = Some((target_valve_id1, target_valve_id2));
                    }
                }
            }
        } else if time_remaining1 > 0 { // only I move
            for &target_valve_id in map.keys() {
                if from1 == target_valve_id || open_valves.contains(&target_valve_id) {
                    continue;
                }

                let Some(path) = paths.get(&(from1, target_valve_id)).unwrap() else {
                    continue;
                };

                let path_and_open_time = path.len() as u64 + 1;
                if time_remaining1 < path_and_open_time {
                    continue;
                }

                let target_valve = map.get(&target_valve_id).unwrap();
                if target_valve.rate == 0 {
                    continue;
                }

                let pressure_released = explore_rec(map, paths,
                                                    target_valve_id, from2, &mut open_valves.clone(),
                                                    time_remaining1 - path_and_open_time,
                                                    0);
                if pressure_released > best_pressure_released {
                    best_pressure_released = pressure_released;
                    best_target_valve_id = Some((target_valve_id, 0xFFFF));
                }
            }
        } else if time_remaining2 > 0 { // only elephant moves
            for &target_valve_id in map.keys() {
                if from2 == target_valve_id || open_valves.contains(&target_valve_id) {
                    continue;
                }

                let Some(path) = paths.get(&(from2, target_valve_id)).unwrap() else {
                    continue;
                };

                let path_and_open_time = path.len() as u64 + 1;
                if time_remaining2 < path_and_open_time {
                    continue;
                }

                let target_valve = map.get(&target_valve_id).unwrap();
                if target_valve.rate == 0 {
                    continue;
                }

                let pressure_released = explore_rec(map, paths,
                                                    from1, target_valve_id, &mut open_valves.clone(),
                                                    0,
                                                    time_remaining2 - path_and_open_time);
                if pressure_released > best_pressure_released {
                    best_pressure_released = pressure_released;
                    best_target_valve_id = Some((0xFFFF, target_valve_id));
                }
            }
        }

        if let Some(_) = best_target_valve_id {
            total_pressure_released += best_pressure_released;
        }

        total_pressure_released
    }

    let start = Valve::parse_id("AA".as_bytes());
    explore_rec(map, &build_paths(map), start, start, &mut HashSet::new(), 26, 26)
}



fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input16");
    let valves = input
        .lines()
        .map(Valve::parse)
        .collect::<Option<Vec<_>>>().unwrap();
    let valves_map = valves.iter().fold(HashMap::new(), |mut map, v| {
        map.insert(v.id, v.clone());
        map
    });

    let res1 = explore(&valves_map);
    println!("[Part 1] Result is {res1}");
    let res2 = explore2(&valves_map);
    println!("[Part 2] Result is {res2}");
}
