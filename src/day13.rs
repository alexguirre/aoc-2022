use std::cmp::Ordering;

mod profiler;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    Int(u64),
    List(List),
}
type List = Vec<Value>;

fn parse_int(s: &[u8], i: &mut usize) -> Value {
    let start = *i;
    *i += 1;
    while (b'0'..=b'9').contains(&s[*i]) { *i += 1; }

    let end = *i;
    Value::Int(std::str::from_utf8(&s[start..end]).unwrap().parse::<u64>().unwrap())
}

fn parse_list(s: &[u8], i: &mut usize) -> Value {
    assert_eq!(b'[', s[*i]);
    *i += 1;

    let mut res = List::new();
    loop {
        match s[*i] {
            b'0'..=b'9' => res.push(parse_int(s, i)),
            b'[' => res.push(parse_list(s, i)),
            b']' => (),
            c => panic!("unknown format: '{}' (index: {})",
                        std::char::from_u32(c as u32).unwrap(),
                        *i),
        };

        if s[*i] == b']' {
            break;
        } else {
            assert_eq!(b',', s[*i]);
            *i += 1;
        }
    }

    assert_eq!(b']', s[*i]);
    *i += 1;

    Value::List(res)
}

fn cmp(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Int(an), Value::Int(bn)) => an.cmp(bn),
        (Value::List(al), Value::List(bl)) => {
            let n = al.len().min(bl.len());
            for i in 0..n {
                match cmp(&al[i], &bl[i]) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => continue,
                }
            }

            al.len().cmp(&bl.len())
        },
        (Value::Int(an), Value::List(_)) => cmp(&Value::List(vec![Value::Int(*an)]), b),
        (Value::List(_), Value::Int(bn)) => cmp(a, &Value::List(vec![Value::Int(*bn)])),
    }
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input13");
    let pairs = input
        .split("\n\n")
        .map(|pair_str| {
            let (a, b) = pair_str.split_once('\n').unwrap();
            (parse_list(a.as_bytes(), &mut 0), parse_list(b.as_bytes(), &mut 0))
        })
        .collect::<Vec<_>>();


    let res1 = pairs.iter()
        .enumerate()
        .inspect(|(i, (a, b))| println!("#{}: {:?} ({})", i + 1, cmp(a, b), if cmp(a, b) == Ordering::Less { "correct" } else { "incorrect" }))
        .filter(|(_, (a, b))| cmp(a, b) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("[Part 1] Result is {res1}");

    let divider1 = parse_list("[[2]]".as_bytes(), &mut 0);
    let divider2 = parse_list("[[6]]".as_bytes(), &mut 0);

    let mut packets = pairs.into_iter()
        .flat_map(|(a, b)| [a, b])
        .chain([
            divider1.clone(),
            divider2.clone(),
        ])
        .collect::<Vec<_>>();
    packets.sort_unstable_by(cmp);

    let res2 = packets.iter()
        .enumerate()
        .filter_map(|(i, v)| if v == &divider1 || v == &divider2 { Some(i + 1) } else { None })
        .product::<usize>();
    println!("[Part 2] Result is {res2}");
}
