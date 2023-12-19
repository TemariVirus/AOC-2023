use std::collections::HashMap;

const INPUT: &str = include_str!("day19.txt");

type Workflow = [(Condition, &'static str); 4];
type WorkflowMap = HashMap<&'static str, Workflow>;
type Part = [u16; 4];
// Includes start, excludes end
type PartsRange = [(u16, u16); 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Gt(u8, u16),
    Lt(u8, u16),
    True,
}

impl Condition {
    const CAT_MAP: [char; 4] = ['x', 'm', 'a', 's'];
}

impl std::str::FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = if s.contains('<') {
            '<'
        } else if s.contains('>') {
            '>'
        } else {
            return Err(());
        };

        let cat = Condition::CAT_MAP
            .iter()
            .position(|&c| c == s.chars().next().unwrap())
            .unwrap();
        let tresh = s[2..].parse().unwrap();
        match split {
            '<' => Ok(Condition::Lt(cat as u8, tresh)),
            '>' => Ok(Condition::Gt(cat as u8, tresh)),
            _ => unreachable!(),
        }
    }
}

fn parse_input() -> (WorkflowMap, Vec<Part>) {
    let (workflows, parts) = INPUT.split_once("\n\n").unwrap();

    let mut workflow_map = HashMap::new();
    for line in workflows.lines() {
        let (name, rest) = line.split_once('{').unwrap();

        let mut rules = [(Condition::True, ""); 4];
        for (i, rule) in rest[..rest.len() - 1].split(',').enumerate() {
            rules[i] = if !rule.contains(':') {
                (Condition::True, rule)
            } else {
                let (cond, dest) = rule.split_once(':').unwrap();
                (cond.parse().unwrap(), dest)
            };
        }

        workflow_map.insert(name, rules);
    }

    let parts = parts
        .lines()
        .map(|line| {
            let mut part = [0; 4];
            for (i, cat) in line[1..line.len() - 1].split(',').enumerate() {
                part[i] = cat[2..].parse().unwrap();
            }
            part
        })
        .collect();

    (workflow_map, parts)
}

fn match_part(workflows: &WorkflowMap, part: Part) -> bool {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        for rule in workflow {
            let matched = match rule.0 {
                Condition::True => true,
                Condition::Lt(cat, tresh) => part[cat as usize] < tresh,
                Condition::Gt(cat, tresh) => part[cat as usize] > tresh,
            };
            if !matched {
                continue;
            }

            match rule.1 {
                "A" => return true,
                "R" => return false,
                dest => workflow = workflows.get(dest).unwrap(),
            }
            break;
        }
    }
}

#[allow(dead_code)]
pub fn part1() -> u32 {
    let (workflows, parts) = parse_input();
    parts
        .iter()
        .filter(|&&part| match_part(&workflows, part))
        .flat_map(|part| part.iter().map(|&x| x as u32))
        .sum()
}

fn count_matched(workflows: &WorkflowMap, workflow_name: &str, mut range: PartsRange) -> u64 {
    if workflow_name == "R" {
        return 0;
    }
    if workflow_name == "A" {
        return range
            .iter()
            .map(|(start, end)| (end - start) as u64)
            .product();
    }

    let workflow = workflows.get(workflow_name).unwrap();
    let mut count = 0;

    for rule in workflow {
        let mut matched = range;
        match rule.0 {
            Condition::True => (),
            Condition::Lt(cat, tresh) => {
                matched[cat as usize].1 = tresh;
                range[cat as usize].0 = tresh;
            }
            Condition::Gt(cat, tresh) => {
                matched[cat as usize].0 = tresh + 1;
                range[cat as usize].1 = tresh + 1;
            }
        }
        count += count_matched(workflows, rule.1, matched);

        if rule.0 == Condition::True {
            break;
        }
    }

    count
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let (workflows, _) = parse_input();
    count_matched(&workflows, "in", [(1, 4001); 4])
}
