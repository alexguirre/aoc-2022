use std::collections::{HashMap, VecDeque};

mod profiler;

type Id = [u8; 4];

const fn id_from_str(s: &str) -> Id {
    let s = s.as_bytes();
    [s[0], s[1], s[2], s[3]]
}

fn id_to_str(id: Id) -> String {
    std::str::from_utf8(&id).unwrap().into()
}

#[derive(Debug, Copy, Clone)]
enum Op { Add, Sub, Mul, Div }

#[derive(Debug, Copy, Clone)]
struct Operation {
    op: Op,
    lhs: Id,
    rhs: Id,
}

#[derive(Debug, Copy, Clone)]
struct Monkey {
    id: Id,
    operation: Option<Operation>,
    value: Option<i64>,
}

impl Monkey {
    fn parse(s: &str) -> Option<Monkey> {
        let (id, value_or_operation) = s.split_once(": ")?;
        let id = id_from_str(id);
        let mut operation = None;
        let mut value = None;
        match value_or_operation.chars().next()? {
            'a'..='z' => { // operation
                let mut parts = value_or_operation.split(' ');
                let lhs = id_from_str(parts.next()?);
                let op = match parts.next()? {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => return None,
                };
                let rhs = id_from_str(parts.next()?);
                operation = Some(Operation { op, lhs, rhs });
            },
            '0'..='9' => { // number
                value = Some(value_or_operation.parse().ok()?);
            },
            _ => return None,
        }

        Some(Monkey { id, operation, value })
    }
}

const ROOT: Id = id_from_str("root");
const HUMN: Id = id_from_str("humn");

fn build_id_to_index_map(monkeys: &Vec<Monkey>) -> HashMap<Id, usize> {
    monkeys.iter().enumerate().fold(HashMap::new(), |mut map, (i, m)| {
        map.insert(m.id, i); map
    })
}

fn resolve_monkeys(monkeys: &mut Vec<Monkey>, id_to_index: &HashMap<Id, usize>) {
    let mut stack_to_resolve = Vec::new();
    stack_to_resolve.push(ROOT);
    while !stack_to_resolve.is_empty() {
        let m_id = stack_to_resolve.last().unwrap();
        let mut m = monkeys[*id_to_index.get(m_id).unwrap()];
        if let Some(_) = m.value {
            stack_to_resolve.pop();
        } else if let Some(operation) = m.operation {
            let m_lhs = monkeys[*id_to_index.get(&operation.lhs).unwrap()];
            let m_rhs = monkeys[*id_to_index.get(&operation.rhs).unwrap()];
            if let (Some(lhs_val), Some(rhs_val)) = (m_lhs.value, m_rhs.value) {
                m.value = Some(match operation.op {
                    Op::Add => lhs_val + rhs_val,
                    Op::Sub => lhs_val - rhs_val,
                    Op::Mul => lhs_val * rhs_val,
                    Op::Div => lhs_val / rhs_val,
                });
                monkeys[*id_to_index.get(m_id).unwrap()] = m;
                stack_to_resolve.pop();
            } else {
                if let None = m_lhs.value {
                    stack_to_resolve.push(operation.lhs);
                }

                if let None = m_rhs.value {
                    stack_to_resolve.push(operation.rhs);
                }
            }
        } else {
            panic!("monkey in invalid state");
        }
    }
}

fn get_monkey_dependencies(monkeys: &Vec<Monkey>, id_to_index: &HashMap<Id, usize>, target_id: Id) -> Vec<Id> {
    let mut deps = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(target_id);
    while !queue.is_empty() {
        let m_id = queue.pop_front().unwrap();
        let m = monkeys[*id_to_index.get(&m_id).unwrap()];
        if let Some(operation) = m.operation {
            deps.push(operation.lhs);
            deps.push(operation.rhs);
            queue.push_back(operation.lhs);
            queue.push_back(operation.rhs);
        }
    }

    deps
}

fn get_path_to_human(monkeys: &Vec<Monkey>, id_to_index: &HashMap<Id, usize>, from_id: Id) -> Vec<Id> {
    let mut path = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back(from_id);
    while !queue.is_empty() {
        let m_id = queue.pop_front().unwrap();
        let m = monkeys[*id_to_index.get(&m_id).unwrap()];
        if let Some(operation) = m.operation {
            if get_monkey_dependencies(monkeys, id_to_index, operation.lhs).contains(&HUMN) {
                path.push(operation.lhs);
                queue.push_back(operation.lhs);
            } else if get_monkey_dependencies(monkeys, id_to_index, operation.rhs).contains(&HUMN) {
                path.push(operation.rhs);
                queue.push_back(operation.rhs);
            }
        }
    }

    path
}

fn find_value_for_human(monkeys: &Vec<Monkey>, id_to_index: &HashMap<Id, usize>) -> Option<i64> {
    let root = monkeys[*id_to_index.get(&ROOT)?];
    let (path_start_id, mut expected_value) = if let Some(operation) = root.operation {
        let (follow, expected) = if get_monkey_dependencies(monkeys, id_to_index, operation.lhs).contains(&HUMN) {
            (operation.lhs, operation.rhs)
        } else if get_monkey_dependencies(monkeys, id_to_index, operation.rhs).contains(&HUMN) {
            (operation.rhs, operation.lhs)
        } else {
            panic!("missing human")
        };
        (follow, monkeys[*id_to_index.get(&expected)?].value?)
    } else {
        panic!("root must have an operation");
    };

    let mut queue = VecDeque::new();
    queue.push_back(path_start_id);
    while !queue.is_empty() {
        let m_id = queue.pop_front().unwrap();
        // println!("{:?}", id_to_str(m_id));
        if m_id == HUMN {
            return Some(expected_value);
        }

        let m = monkeys[*id_to_index.get(&m_id).unwrap()];
        if let Some(operation) = m.operation {
            // println!(" > {:?} {:?} {:?}", id_to_str(operation.lhs), operation.op, id_to_str(operation.rhs));
            if get_monkey_dependencies(monkeys, id_to_index, operation.lhs).contains(&HUMN) || operation.lhs == HUMN {
                // println!("  > lhs contains HUMN, rhs is known");
                let rhs_val = monkeys[*id_to_index.get(&operation.rhs).unwrap()].value.unwrap();
                let lhs_val = match operation.op {
                    Op::Add => /*lhs_val + rhs_val = expected_value */ expected_value - rhs_val,
                    Op::Sub => /*lhs_val - rhs_val = expected_value */ expected_value + rhs_val,
                    Op::Mul => /*lhs_val * rhs_val = expected_value */ expected_value / rhs_val,
                    Op::Div => /*lhs_val / rhs_val = expected_value */ expected_value * rhs_val,
                };
                expected_value = lhs_val;
                queue.push_back(operation.lhs);
            } else if get_monkey_dependencies(monkeys, id_to_index, operation.rhs).contains(&HUMN) || operation.rhs == HUMN {
                // println!("  > rhs contains HUMN, lhs is known");
                let lhs_val = monkeys[*id_to_index.get(&operation.lhs).unwrap()].value.unwrap();
                let rhs_val = match operation.op {
                    Op::Add => /*lhs_val + rhs_val = expected_value */ expected_value - lhs_val,
                    Op::Sub => /*lhs_val - rhs_val = expected_value */ lhs_val - expected_value,
                    Op::Mul => /*lhs_val * rhs_val = expected_value */ expected_value / lhs_val,
                    Op::Div => /*lhs_val / rhs_val = expected_value */ lhs_val / expected_value,
                };
                expected_value = rhs_val;
                queue.push_back(operation.rhs);
            }
        } else {
            panic!("expected operation")
        }
    }

    None
}

fn main() {
    let _p = profiler::profile();

    const INPUT: &str = include_str!("data/input21");
    let monkeys = INPUT.lines()
        .map(Monkey::parse)
        .collect::<Option<Vec<_>>>().unwrap();
    let id_to_index = build_id_to_index_map(&monkeys);

    {
        let mut monkeys1 = monkeys.clone();
        resolve_monkeys(&mut monkeys1, &id_to_index);
        let res1 = monkeys1[*id_to_index.get(&ROOT).unwrap()].value;
        println!("[Part 1] Result is {res1:?}");
    }

    {
        let mut monkeys2 = monkeys.clone();
        resolve_monkeys(&mut monkeys2, &id_to_index);
        let res2 = find_value_for_human(&monkeys2, &id_to_index);
        println!("[Part 2] Result is {res2:?}");
    }
}
