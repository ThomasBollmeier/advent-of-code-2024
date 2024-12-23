use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use anyhow::*;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let network = read_network(reader)?;
        let groups = find_groups_of_three(&network)
            .into_iter()
            .filter(at_least_one_computer_starts_with_t)
            .collect::<Vec<_>>();

        /*
        for group in &groups {
            println!("{},{},{}", group.0, group.1, group.2);
        }

        */

        let answer = groups.len();
        Ok(answer)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<String> { 
        let network = read_network(reader)?;    
        let groups = find_all_connected(&network);
        let mut max_size = 0;
        let mut max_group_opt: Option<Group> = None;
        
        for group in groups {
            if group.get_size() > max_size {
                max_size = group.get_size();
                max_group_opt = Some(group);
            }
        }
        
        let max_group = max_group_opt.ok_or_else(|| anyhow!("no group found"))?;
        
        Ok(format!("{max_group}"))
    }

    assert_eq!("co,de,ka,ta", part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Network = HashMap<String, HashSet<String>>;
type Group3 = (String, String, String);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Group {
    members: HashSet<String>,
}

impl Group {
    fn new() -> Self {
        Group{members: HashSet::new()}
    }

    fn add_member(&mut self, member: &str) {
        self.members.insert(member.to_string());
    }

    fn contains_member(&self, member: &str) -> bool {
        self.members.contains(member)
    }

    fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
    
    fn get_size(&self) -> usize {
        self.members.len()
    }
}

impl Hash for Group {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let members = &self.members
            .iter()
            .cloned()
            .sorted()
            .collect::<Vec<_>>()
            .join(",");
        members.hash(state);
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let members = &self.members
            .iter()
            .cloned()
            .sorted()
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{members}")
    }
}

fn find_all_connected(network: &Network) -> HashSet<Group> {
    let mut ret = HashSet::new();
    for computer in network.keys() {
        let mut group = Group::new();
        find_connected(network, computer, &mut group);
        if !group.is_empty() {
            ret.insert(group);
        }
    }

    ret
}

fn find_connected(network: &Network, computer: &str, group: &mut Group) {
    if group.contains_member(computer) {
        return;
    }
    if let Some(connected) = network.get(computer) {
        let members = group.members.clone().into_iter().collect::<Vec<_>>();
        if members.iter().all(|c| connected.contains(c)) {
            group.add_member(computer);
        } else {
            return;
        }
        for partner in connected {
            find_connected(network, partner, group);
        }
    }
}

fn at_least_one_computer_starts_with_t(group: &Group3) -> bool {
    [&group.0, &group.1, &group.2]
        .iter()
        .any(|s| s.starts_with("t"))
}

fn find_groups_of_three(network: &Network) -> Vec<Group3> {
    let mut groups = HashSet::new();
    for (computer, others) in network {
        for other in others {
            if other == computer {
                continue;
            }
            if let Some(connections) = network.get(other) {
                for connected_partner in connections {
                    if connected_partner == computer || connected_partner == other {
                        continue;
                    }
                    if let Some(partner_connections) = network.get(connected_partner) {
                        if !partner_connections.contains(computer) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    let mut elements = [
                        computer.clone(),
                        other.clone(),
                        connected_partner.clone()];
                    elements.sort();
                    let group = (elements[0].clone(), elements[1].clone(), elements[2].clone());
                    groups.insert(group);
                }
            }
        }
    }

    let mut ret = Vec::from_iter(groups.clone());
    ret.sort();

    ret
}

fn read_network(reader: impl BufRead) -> Result<Network> {
    let mut network: HashMap<String, HashSet<String>> = HashMap::new();
    for line in read_lines(reader) {
        let parts = line.split('-').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(anyhow!("invalid input"));
        }
        let computer1 = parts[0].to_string();
        let computer2 = parts[1].to_string();
        network.entry(computer1.clone()).or_default().insert(computer2.clone());
        network.entry(computer2.clone()).or_default().insert(computer1.clone());
    }

    Ok(network)
}
