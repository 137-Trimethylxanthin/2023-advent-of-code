use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;


const LOW: bool = false;
const HIGH: bool = true;

struct Module<'a> {
    module_type: char,
    flip_flop_state: bool,
    conjunction_connections: HashMap<&'a str, bool>,
    receivers: Vec<&'a str>,
}

struct Pulse<'a> {
    destination: &'a str,
    signal: bool,
    source: &'a str,
}

pub fn solve() {
    let mut modules: HashMap<&str, Module> = HashMap::new();
    let mut reverse: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        let (sender_and_type, receivers_raw): (&str, &str) = line.split_once(" -> ").unwrap();
        let module_type: char = sender_and_type.chars().nth(0).unwrap();
        let sender: &str = &sender_and_type[1..];
        let mut receivers: Vec<&str> = Vec::new();
        let mut receiver_iterator = receivers_raw.split(", ");
        while let Some(x) = receiver_iterator.next() {
            receivers.push(x);
        }
        for receiver in &receivers {
            if let Some(x) = reverse.get_mut(receiver) {
                x.push(sender)
            } else {
                let mut new_vec: Vec<&str> = Vec::new();
                new_vec.push(sender);
                reverse.insert(receiver, new_vec);
            }
        }
        modules.insert(
            sender,
            Module {
                module_type,
                flip_flop_state: LOW,
                conjunction_connections: HashMap::new(),
                receivers,
            },
        );
    }

    for (key, value) in &mut modules {
        if value.module_type == '&' {
            for m in reverse.get_mut(key).unwrap() {
                value.conjunction_connections.insert(m, LOW);
            }
        }
    }

    let mut targets: HashMap<&str, usize> = HashMap::new();
    let last_conv = reverse.get("rx").unwrap()[0]; // rx is the last conjunction. this is very hacky but it works. this does NOT work for the examples
    for target in reverse.get(last_conv).unwrap() {
        targets.insert(target, 0);
    }

    let mut counter = 0;
    let mut low_count = 0;
    let mut high_count = 0;
    let mut pulses = 0;
    let mut p2;

    'sim: loop {
        counter += 1;
        let mut queue: VecDeque<Pulse> = VecDeque::new();
        queue.push_back(Pulse {
            destination: "roadcaster",
            signal: LOW,
            source: "",
        });
        low_count += 1;
        'queue: while let Some(current) = queue.pop_front() {
            let next = update(&mut modules, current);
            if next.0.len() == 0 {
                continue 'queue;
            }
            for next_pulse in next.0 {
                if targets
                    .get(next_pulse.destination)
                    .is_some_and(|_x| next_pulse.signal == LOW)
                {
                    targets.insert(next_pulse.destination, counter);
                }
                p2 = 1;
                for (_, v) in &targets {
                    p2 *= *v;
                }
                if p2 != 0 {
                    break 'sim;
                }
                queue.push_back(next_pulse);
            }
            low_count += next.1;
            high_count += next.2;
        }

        if counter == 1000 {
            pulses = low_count * high_count;
        }
    }

    println!("Part 1: {}", pulses);
    println!("Part 2: {}", p2);
}

fn update<'a>(
    modules: &mut HashMap<&'a str, Module<'a>>,
    pulse: Pulse<'a>,
) -> (Vec<Pulse<'a>>, usize, usize) {
    let mut res_vec: Vec<Pulse<'a>> = Vec::new();
    let mut low_count = 0;
    let mut high_count = 0;
    if let Some(value) = modules.get_mut(pulse.destination) {
        if value.module_type == 'b' {
            // broadcaster
            low_count = value.receivers.len();
            for dest in &value.receivers {
                res_vec.push(Pulse {
                    destination: dest,
                    signal: LOW,
                    source: pulse.destination,
                });
            }
        } else if value.module_type == '%' && pulse.signal == LOW {
            // flip flop
            value.flip_flop_state = !value.flip_flop_state;
            if value.flip_flop_state == LOW {
                low_count = value.receivers.len();
            } else {
                high_count = value.receivers.len();
            }
            for dest in &value.receivers {
                res_vec.push(Pulse {
                    destination: dest,
                    signal: value.flip_flop_state,
                    source: pulse.destination,
                });
            }
        } else if value.module_type == '&' {
            // conjunction
            value
                .conjunction_connections
                .insert(pulse.source, pulse.signal);
            let mut new_signal = LOW;
            for (_, v) in &value.conjunction_connections {
                if *v == LOW {
                    new_signal = HIGH;
                    break;
                }
            }
            if new_signal == LOW {
                low_count = value.receivers.len();
            } else {
                high_count = value.receivers.len();
            }
            for dest in &value.receivers {
                res_vec.push(Pulse {
                    destination: dest,
                    signal: new_signal,
                    source: pulse.destination,
                });
            }
        }
    }
    return (res_vec, low_count, high_count);
}

fn main() {
    solve();
}