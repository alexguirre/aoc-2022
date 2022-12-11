use std::collections::VecDeque;

mod profiler;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Int(u64),
}

impl Operand {
    fn parse(def: &str) -> Option<Self> {
        use Operand::*;

        Some(match def {
            "old" => Old,
            int => Int(int.parse::<u64>().ok()?),
        })
    }

    fn value(&self, old: u64) -> u64 {
        use Operand::*;

        match self {
            Old => old,
            Int(n) => *n,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl Operation {
    fn parse(def: &str) -> Option<Self> {
        use Operation::*;

        let mut parts = def.split(' ');
        let operand1 = parts.next().and_then(Operand::parse)?;
        let op = parts.next()?;
        let operand2 = parts.next().and_then(Operand::parse)?;
        Some(match op {
            "+" => Add(operand1, operand2),
            "*" => Mul(operand1, operand2),
            _ => None?,
        })
    }

    fn compute(&self, old: u64) -> u64 {
        use Operation::*;

        match self {
            Add(o1, o2) => o1.value(old) + o2.value(old),
            Mul(o1, o2) => o1.value(old) * o2.value(old),
        }
    }

    fn compute_mod(&self, old: u64, mod_value: u64) -> u64 {
        use Operation::*;

        (match self {
            Add(o1, o2) => (o1.value(old) % mod_value) + (o2.value(old) % mod_value),
            Mul(o1, o2) => (o1.value(old) % mod_value) * (o2.value(old) % mod_value),
        }) % mod_value
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisible_test: u64,
    if_true: usize,
    if_false: usize,
    num_inspected_items: usize,
}

impl Monkey {
    fn parse(def: &str) -> Option<Self> {
        let mut lines = def.lines();
        lines.next(); // skip 'Monkey N:'
        let items = lines.next()
            .and_then(
                |s| s.split_once(": ").and_then(
                    |s| s.1.split(", ").map(|s| s.parse::<u64>().ok()).collect::<Option<VecDeque<_>>>()
                )
            )?;

        let operation = lines.next()
            .and_then(
                |s| s.split_once("new = ").and_then(
                    |s| Operation::parse(s.1)
                )
            )?;

        fn parse_last_int<T: std::str::FromStr>(s: &str) -> Option<T> {
            s.rsplit_once(" ").and_then(
                |s| s.1.parse::<T>().ok()
            )
        }

        let divisible_test = lines.next().and_then(parse_last_int::<u64>)?;
        let if_true = lines.next().and_then(parse_last_int::<usize>)?;
        let if_false = lines.next().and_then(parse_last_int::<usize>)?;

        Some(Self {
            items,
            operation,
            divisible_test,
            if_true,
            if_false,
            num_inspected_items: 0
        })
    }
}

fn resolve<const PART: usize>(monkeys: &mut Vec<Monkey>) -> usize {

    // The 'divisible test' values are unique prime numbers, so compute the least-common-multiple to wrap
    // around the operations using modulus for part 2
    let lcm = monkeys.iter().fold(1, |acc, m| acc * m.divisible_test);

    let num_rounds: usize = if PART == 1 { 20 } else { 10_000 };
    for round in 1..=num_rounds {
        for monkey_index in 0..monkeys.len() {
            let (first, second) = monkeys.split_at_mut(monkey_index);
            let (monkey_slice, third) = second.split_at_mut(1);
            let monkey = &mut monkey_slice[0];
            while let Some(item_worry) = monkey.items.pop_front() {
                monkey.num_inspected_items += 1;

                let new_item_worry = if PART == 1 {
                    monkey.operation.compute(item_worry) / 3
                } else {
                    monkey.operation.compute_mod(item_worry, lcm)
                };

                let target_monkey = if new_item_worry % monkey.divisible_test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                if target_monkey < first.len() {
                    &mut first[target_monkey]
                } else {
                    &mut third[target_monkey - 1 - first.len()]
                }.items.push_back(new_item_worry);
            }
        }

        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("== After round {round} ==");
            for monkey_index in 0..monkeys.len() {
                println!("Monkey {monkey_index} inspected items {} times.", monkeys[monkey_index].num_inspected_items);
            }
            println!();
        }
    }

    monkeys.sort_unstable_by_key(|m| usize::MAX - m.num_inspected_items);
    println!("{} * {} = {}", monkeys[0].num_inspected_items, monkeys[1].num_inspected_items, monkeys[0].num_inspected_items * monkeys[1].num_inspected_items);
    monkeys[0].num_inspected_items * monkeys[1].num_inspected_items
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input11");

    let mut monkeys = input
        .split("\n\n")
        .map(Monkey::parse)
        .map(|m| m.unwrap())
        .collect::<Vec<_>>();

    let mut monkeys2 = monkeys.clone();

    let res1 = resolve::<1>(&mut monkeys);
    let res2 = resolve::<2>(&mut monkeys2);
    println!("[Part 1] Result is {res1}");
    println!("[Part 2] Result is {res2}");
}
