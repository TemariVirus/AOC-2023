use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("day20.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(Vec<&'static str>, bool),
    Conjuntion(Vec<&'static str>, HashMap<&'static str, bool>),
    Broadcast(Vec<&'static str>),
}

fn parse_input() -> HashMap<&'static str, Module> {
    let mut modules: HashMap<_, _> = INPUT
        .lines()
        .map(|line| {
            let (module, outputs) = line.split_once(" -> ").unwrap();
            let outputs = outputs.split(", ").collect::<Vec<_>>();
            let (name, module) = match module.chars().next().unwrap() {
                '%' => (&module[1..], Module::FlipFlop(outputs, false)),
                '&' => (&module[1..], Module::Conjuntion(outputs, HashMap::new())),
                _ => {
                    if module == "broadcaster" {
                        (module, Module::Broadcast(outputs))
                    } else {
                        panic!("Unknown module: {}", module);
                    }
                }
            };
            (name, module)
        })
        .collect();

    let outputs_to_update: Vec<_> = modules
        .iter()
        .flat_map(|(name, module)| {
            let outputs = match module {
                Module::FlipFlop(outputs, _) => outputs,
                Module::Conjuntion(outputs, _) => outputs,
                Module::Broadcast(outputs) => outputs,
            };
            outputs
                .iter()
                .filter(|&&output| matches!(modules.get(output), Some(Module::Conjuntion(_, _))))
                .map(move |output| (*output, *name))
        })
        .collect();

    for (output, name) in outputs_to_update {
        if let Module::Conjuntion(_, inputs) = modules.get_mut(output).unwrap() {
            inputs.insert(name, false);
        }
    }

    modules
}

#[allow(dead_code)]
pub fn part1() -> u32 {
    let mut modules = parse_input();
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = count_pulses(&mut modules);
        low += l;
        high += h;
    }
    low * high
}

fn count_pulses(modules: &mut HashMap<&'static str, Module>) -> (u32, u32) {
    let mut low = 0;
    let mut high = 0;
    let mut pulses = VecDeque::new();

    // Press button
    pulses.push_back(("button", "broadcaster", false));
    low += 1;

    while let Some((from, to, is_high)) = pulses.pop_front() {
        let module = if let Some(m) = modules.get_mut(to) {
            m
        } else {
            continue;
        };

        let (count, pulse) = match module {
            Module::FlipFlop(outputs, is_off) => {
                if is_high {
                    continue;
                }

                *is_off = !*is_off;
                for output in outputs.iter() {
                    pulses.push_back((to, output, *is_off));
                }
                (outputs.len(), *is_off)
            }
            Module::Conjuntion(outputs, inputs) => {
                inputs.insert(from, is_high);
                let out_pulse = !inputs.values().all(|&is_on| is_on);
                for output in outputs.iter() {
                    pulses.push_back((to, output, out_pulse));
                }
                (outputs.len(), out_pulse)
            }
            Module::Broadcast(outputs) => {
                for output in outputs.iter() {
                    pulses.push_back((to, output, is_high));
                }
                (outputs.len(), is_high)
            }
        };

        if pulse {
            high += count as u32;
        } else {
            low += count as u32;
        }
    }

    (low, high)
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let mut modules = parse_input();

    // Get cycle lengths of each flip-flop
    let mut bits = HashMap::new();
    count_pulses(&mut modules);
    for pow in 1.. {
        let n_cycle = cycle_thingy(1 << pow, &mut modules);
        if n_cycle.len() != 4 {
            break;
        }

        for &s in n_cycle.iter() {
            bits.insert(s, pow - 1);
        }
    }

    let inv = modules
        .iter()
        .find(|(_, m)| {
            if let Module::Conjuntion(outputs, _) = m {
                if outputs.contains(&"rx") {
                    return true;
                }
            }
            false
        })
        .unwrap()
        .0;
    modules
        .iter()
        .filter(|(_, m)| {
            // Get inv's inputs
            if let Module::Conjuntion(outputs, _) = m {
                if outputs.contains(inv) {
                    return true;
                }
            }
            false
        })
        .map(|(name, _)| {
            // These are NOT gates; get their inputs
            modules
                .iter()
                .find(|(_, m)| {
                    if let Module::Conjuntion(outputs, _) = m {
                        if outputs.contains(name) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap()
                .0
        })
        .map(|&s| {
            // Get the cycle length of each conjuntion
            let mut cycle_len = 0;
            if let Module::Conjuntion(_, inputs) = modules.get(s).unwrap() {
                for &s in inputs.keys() {
                    cycle_len |= 1 << bits.get(s).unwrap();
                }
            }
            cycle_len
        })
        .product()
}

fn cycle_thingy(n: u64, modules: &mut HashMap<&'static str, Module>) -> Vec<&'static str> {
    let on: Vec<_> = modules
        .iter()
        .filter_map(|(&s, m)| {
            if let Module::FlipFlop(_, is_on) = m {
                if *is_on {
                    return Some(s);
                }
            }
            None
        })
        .collect();

    for _ in 0..n / 2 {
        count_pulses(modules);
    }
    on
}
