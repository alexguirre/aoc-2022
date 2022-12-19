use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

mod profiler;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Cost { ore: u16, clay: u16, obsidian: u16 }

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Blueprint {
    id: u8,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
    max_cost: Cost,
}

impl Blueprint {
    fn parse(s: &str) -> Option<Blueprint> {
        // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 2 ore and 16 obsidian.
        let (bp_id, ore_robot_cost) = s.trim_start_matches("Blueprint ").split_once(": Each ore robot costs ")?;
        let bp_id = bp_id.parse::<u8>().ok()?;
        let (ore_robot_cost, clay_robot_cost) = ore_robot_cost.split_once(" ore. Each clay robot costs ")?;
        let ore_robot_cost = ore_robot_cost.parse::<u16>().ok()?;
        let (clay_robot_cost, obsidian_robot_cost) = clay_robot_cost.split_once(" ore. Each obsidian robot costs ")?;
        let clay_robot_cost = clay_robot_cost.parse::<u16>().ok()?;
        let (obsidian_robot_cost_ore, obsidian_robot_cost_clay) = obsidian_robot_cost.split_once(" ore and ")?;
        let obsidian_robot_cost_ore = obsidian_robot_cost_ore.parse::<u16>().ok()?;
        let (obsidian_robot_cost_clay, geode_robot_cost) = obsidian_robot_cost_clay.split_once(" clay. Each geode robot costs ")?;
        let obsidian_robot_cost_clay = obsidian_robot_cost_clay.parse::<u16>().ok()?;
        let (geode_robot_cost_ore, geode_robot_cost_obsidian) = geode_robot_cost.split_once(" ore and ")?;
        let geode_robot_cost_ore = geode_robot_cost_ore.parse::<u16>().ok()?;
        let (geode_robot_cost_obsidian, _) = geode_robot_cost_obsidian.split_once(" obsidian.")?;
        let geode_robot_cost_obsidian = geode_robot_cost_obsidian.parse::<u16>().ok()?;

        Some(Blueprint {
            id: bp_id,
            ore_robot_cost: Cost { ore: ore_robot_cost, clay: 0, obsidian: 0 },
            clay_robot_cost: Cost { ore: clay_robot_cost, clay: 0, obsidian: 0 },
            obsidian_robot_cost: Cost { ore: obsidian_robot_cost_ore, clay: obsidian_robot_cost_clay, obsidian: 0 },
            geode_robot_cost: Cost { ore: geode_robot_cost_ore, clay: 0, obsidian: geode_robot_cost_obsidian },
            max_cost: Cost {
                ore: [ore_robot_cost, clay_robot_cost, obsidian_robot_cost_ore, geode_robot_cost_ore].into_iter().max()?,
                clay: obsidian_robot_cost_clay,
                obsidian: geode_robot_cost_obsidian
            }
        })
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    time_left: u8,
    num_ore_robots: u16,
    num_clay_robots: u16,
    num_obsidian_robots: u16,
    num_geode_robots: u16,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl State {
    fn collect_resources(self) -> Self {
        assert!(self.time_left >= 1);
        Self {
            ore: self.ore + self.num_ore_robots,
            clay: self.clay + self.num_clay_robots,
            obsidian: self.obsidian + self.num_obsidian_robots,
            geode: self.geode + self.num_geode_robots,
            time_left: self.time_left - 1,
            ..self
        }
    }

    fn prev(self) -> Self {
        Self {
            ore: self.ore - self.num_ore_robots,
            clay: self.clay - self.num_clay_robots,
            obsidian: self.obsidian - self.num_obsidian_robots,
            geode: self.geode - self.num_geode_robots,
            time_left: self.time_left + 1,
            ..self
        }
    }

    fn can_pay(self, cost: Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn pay(self, cost: Cost) -> Self {
        assert!(self.can_pay(cost));
        Self {
            ore: self.ore - cost.ore,
            clay: self.clay - cost.clay,
            obsidian: self.obsidian - cost.obsidian,
            ..self
        }
    }

    // these should be called after collect_resources in simulation
    fn add_ore_robot(self, cost: Cost) -> Self {
        assert!(self.can_pay(cost));
        Self {
            num_ore_robots: self.num_ore_robots + 1,
            ..self.pay(cost)
        }
    }
    fn add_clay_robot(self, cost: Cost) -> Self {
        assert!(self.can_pay(cost));
        Self {
            num_clay_robots: self.num_clay_robots + 1,
            ..self.pay(cost)
        }
    }
    fn add_obsidian_robot(self, cost: Cost) -> Self {
        assert!(self.can_pay(cost));
        Self {
            num_obsidian_robots: self.num_obsidian_robots + 1,
            ..self.pay(cost)
        }
    }
    fn add_geode_robot(self, cost: Cost) -> Self {
        assert!(self.can_pay(cost));
        Self {
            num_geode_robots: self.num_geode_robots + 1,
            ..self.pay(cost)
        }
    }
}

impl State {
    fn new(time: u8) -> Self {
        Self {
            time_left: time,
            num_ore_robots: 1,
            num_clay_robots: 0,
            num_obsidian_robots: 0,
            num_geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

// simulate: slow, works for part 1 but consumes too much memory for part 2
// simulate2: fast, doesn't work for part 1 but does work for part 2?
//      The only difference in part 1 results are the blueprints 12 and 13:
//        simulate:
//         BP 12:   geodes=10   quality=120
//         BP 13:   geodes=1   quality=13
//        simulate2:
//         BP 12:   geodes=9   quality=108
//         BP 13:   geodes=0   quality=0

fn simulate(s: State, bp: &Blueprint, cache: &mut HashMap<u64, State>) -> State {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let s_hash = hasher.finish();
    if let Some(new_state) = cache.get(&s_hash) {
        return *new_state;
    }

    let new_state = if s.time_left > 0 {
        let mut possible_new_states = [None; 5];
        let s_after_collect_resources = s.collect_resources();

        const ROBOT_LIMIT: u16 = 10;
        if s.can_pay(bp.geode_robot_cost) && s.num_geode_robots < ROBOT_LIMIT {
            possible_new_states[0] = Some(s_after_collect_resources.add_geode_robot(bp.geode_robot_cost));
        } else {
            if s.can_pay(bp.ore_robot_cost) && s.num_ore_robots < ROBOT_LIMIT {
                possible_new_states[1] = Some(s_after_collect_resources.add_ore_robot(bp.ore_robot_cost));
            }

            if s.can_pay(bp.clay_robot_cost) && s.num_clay_robots < ROBOT_LIMIT {
                possible_new_states[2] = Some(s_after_collect_resources.add_clay_robot(bp.clay_robot_cost));
            }

            if s.can_pay(bp.obsidian_robot_cost) && s.num_obsidian_robots < ROBOT_LIMIT {
                possible_new_states[3] = Some(s_after_collect_resources.add_obsidian_robot(bp.obsidian_robot_cost));
            }

            possible_new_states[4] = Some(s_after_collect_resources);
        }

        possible_new_states.iter()
            .filter_map(|new_state_opt| if let Some(new_state) = new_state_opt {
                Some(simulate(*new_state, bp, cache))
            } else {
                None
            })
            .max_by_key(|new_state_result| new_state_result.geode).unwrap()
    } else {
        s
    };

    cache.insert(s_hash, new_state);
    new_state
}

fn simulate2(s: State, bp: &Blueprint, cache: &mut HashMap<u64, State>) -> State {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let s_hash = hasher.finish();
    if let Some(new_state) = cache.get(&s_hash) {
        return *new_state;
    }

    let new_state = if s.time_left > 0 {
        let mut possible_new_states = [None; 5];
        let s_prev = s.prev();
        let s_next = s.collect_resources();

        if !s_prev.can_pay(bp.geode_robot_cost) && s.can_pay(bp.geode_robot_cost) {
            possible_new_states[0] = Some(s_next.add_geode_robot(bp.geode_robot_cost));
        } else {
            if !s_prev.can_pay(bp.obsidian_robot_cost) && s.can_pay(bp.obsidian_robot_cost) &&
                s.num_obsidian_robots < bp.max_cost.obsidian {
                possible_new_states[3] = Some(s_next.add_obsidian_robot(bp.obsidian_robot_cost));
            }

            if !s_prev.can_pay(bp.clay_robot_cost) && s.can_pay(bp.clay_robot_cost) &&
                s.num_clay_robots < bp.max_cost.clay  {
                possible_new_states[2] = Some(s_next.add_clay_robot(bp.clay_robot_cost));
            }

            if !s_prev.can_pay(bp.ore_robot_cost) && s.can_pay(bp.ore_robot_cost) &&
                s.num_ore_robots < bp.max_cost.ore {
                possible_new_states[1] = Some(s_next.add_ore_robot(bp.ore_robot_cost));
            }

            possible_new_states[4] = Some(s_next);
        }

        possible_new_states.iter()
            .filter_map(|new_state_opt| if let Some(new_state) = new_state_opt {
                Some(simulate2(*new_state, bp, cache))
            } else {
                None
            })
            .max_by_key(|new_state_result| new_state_result.geode).unwrap()
    } else {
        s
    };

    cache.insert(s_hash, new_state);
    new_state
}

fn main() {
    let _p = profiler::profile();

    const INPUT: &str = include_str!("data/input19");
    let blueprints = INPUT.lines()
        .map(Blueprint::parse)
        .collect::<Option<Vec<_>>>().unwrap();

    {let _p1 = profiler::profile();
    let res1 = blueprints.iter()
        .map(|bp| {
            let max_geodes = simulate2(State::new(24), bp, &mut HashMap::new()).geode;
            println!("BP {}:   geodes={}   quality={}", bp.id, max_geodes, bp.id as u64 * max_geodes as u64);
            bp.id as u64 * max_geodes as u64
        })
        .sum::<u64>();
    println!("[Part 1] Result is {res1}");
    }

    {let _p2 = profiler::profile();
    let res2 = blueprints[..3].iter()
        .map(|bp| {
            let max_geodes = simulate2(State::new(32), bp, &mut HashMap::new()).geode;
            println!("BP {}:   geodes={}", bp.id, max_geodes);
            max_geodes as u64
        })
        .product::<u64>();
    println!("[Part 2] Result is {res2}");
    }
}
