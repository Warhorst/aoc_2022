use std::collections::{HashMap, HashSet};
use crate::input_reader::read_input;

pub fn solve_p16() {
    let input = read_input(16);
    let mut runner = Runner::from(input.as_str());
    runner.run();

    println!("Solution 1: {}", runner.flown);
}

struct Runner {
    current: String,
    remaining_minutes: usize,
    flown: usize,
    valves: HashMap<String, Valve>,
    graph: Graph,
    opened: HashSet<Valve>,
}

impl Runner {
    fn run(&mut self) {
        while self.remaining_minutes > 0 {
            println!("Remaining {}", self.remaining_minutes);
            let best_next = self.get_best_next();

            match best_next {
                Some((ident, len)) => {
                    println!("move to {} with len {}", ident, len);
                    self.move_to_best_and_open((ident, len))
                },
                None => {
                    println!("idle");
                    self.idle()
                }
            }
        }
    }

    fn get_best_next(&self) -> Option<(String, usize)> {
        let paths = self.valves
            .values()
            .filter(|valve| !self.opened.contains(valve))
            .filter(|valve| valve.ident != self.current)
            .map(|valve| (
                valve.ident.clone(),
                valve.flow_rate,
                a_star(
                    self.current.clone(),
                    valve.ident.clone(),
                    &self.graph,
                )
                    .map(|vec| vec.len() - 1)
                    .unwrap_or(usize::MAX)))
            .collect::<Vec<_>>();

        let mut best_nearest = None;

        for i in 1..=self.valves.len() + 1 {
            let nearest = paths
                .iter()
                .filter(|(_, _, len)| *len == i)
                .filter(|(_, flow, _)| *flow > 0)
                .map(|(ident, flow, len)| (ident, len, (*flow as f32 / *len as f32).ceil()))
                .max_by(|(_, _, flow_a), (_, _, flow_b)| flow_a.partial_cmp(&flow_b).unwrap());

            match nearest {
                None => (),
                Some((ident,len, flow)) => match best_nearest {
                    None => best_nearest = Some((ident.clone(), len, flow)),
                    Some((_,_, best_flow)) => if flow > best_flow {
                        best_nearest = Some((ident.clone(), len, flow))
                    }
                }
            }
        }

        let bes_nearest = best_nearest?;
        Some((bes_nearest.0, *bes_nearest.1))
    }

    fn move_to_best_and_open(&mut self, (ident, len): (String, usize)) {
        self.flown += self.get_current_flow() * (len + 1); // +1 because i open the valve later
        self.remaining_minutes = self.remaining_minutes.checked_sub(len).unwrap_or(0);
        self.open_valve(&ident);
        self.current = ident;
    }

    fn get_current_flow(&self) -> usize {
        self.opened.iter().map(|valve| valve.flow_rate).sum()
    }

    fn open_valve(&mut self, ident: &String) {
        let valve = self.valves.get(ident).unwrap();
        self.opened.insert(valve.clone());
        self.remaining_minutes = self.remaining_minutes.checked_sub(1).unwrap_or(0);
    }

    fn idle(&mut self) {
        self.flown += self.get_current_flow();
        self.remaining_minutes -= 1;
    }
}

impl From<&str> for Runner {
    fn from(input: &str) -> Self {
        let valves = input.lines().map(Valve::from).collect::<Vec<_>>();
        let graph = Graph::new(valves.clone());

        let current = valves[0].ident.clone();

        Runner {
            current,
            remaining_minutes: 30,
            flown: 0,
            valves: valves.into_iter().map(|valve| (valve.ident.clone(), valve)).collect(),
            graph,
            opened: HashSet::new()
        }
    }
}

struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    fn new(valves: impl IntoIterator<Item=Valve>) -> Self {
        let mut edges = Vec::new();

        valves
            .into_iter()
            .for_each(|valve| {
                for n in &valve.neighbours {
                    edges.push(Edge::new(valve.ident.clone(), n.clone()))
                }
            });

        Graph {
            edges,
        }
    }

    fn adjacent_edges<'a>(&'a self, node: &'a String) -> impl IntoIterator<Item=&Edge> + 'a {
        self.edges.iter().filter(move |edge| &edge.from == node)
    }
}

// Thanks wikipedia
fn a_star(start: String, goal: String, graph: &Graph) -> Option<Vec<String>> {
    let mut open_set: HashSet<String> = HashSet::new();
    open_set.insert(start.clone());

    let mut came_from: HashMap<String, String> = HashMap::new();

    let mut g_score: HashMap<String, usize> = HashMap::new();
    g_score.insert(start.clone(), 0);

    let mut f_score: HashMap<String, usize> = HashMap::new();
    f_score.insert(start.clone(), 1);

    while !open_set.is_empty() {
        let current = open_set.iter().next().unwrap().clone();

        if current == goal {
            return Some(construct_path(came_from, current));
        }

        open_set.remove(&current);

        for edge in graph.adjacent_edges(&current).into_iter() {
            let to = &edge.to;
            let tentative_g_score = g_score.get(&current).unwrap() + 1;

            if tentative_g_score < *g_score.get(to).unwrap_or(&usize::MAX) {
                came_from.insert(to.clone(), current.clone());
                g_score.insert(to.clone(), tentative_g_score);
                f_score.insert(to.clone(), tentative_g_score + 1);

                if !open_set.contains(to) {
                    open_set.insert(to.clone());
                }
            }
        }
    }

    None
}

fn construct_path(came_from: HashMap<String, String>, mut current: String) -> Vec<String> {
    let mut total_path = vec![current.clone()];

    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap().clone();
        total_path.push(current.clone());
    }

    total_path
}

struct Edge {
    from: String,
    to: String,
}

impl Edge {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Valve {
    ident: String,
    flow_rate: usize,
    neighbours: Vec<String>,
}

impl From<&str> for Valve {
    fn from(line: &str) -> Self {
        let cleaned = line
            .replace("Valve ", "")
            .replace(" has flow rate=", "|")
            .replace("; tunnels lead to valves ", "|")
            .replace("; tunnel leads to valve ", "|");

        let mut split = cleaned.split("|");

        Valve {
            ident: split.next().unwrap().to_string(),
            flow_rate: split.next().unwrap().parse::<usize>().unwrap(),
            neighbours: split.next().unwrap().split(",").map(|s| s.trim()).map(String::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p16::{a_star, Graph, Runner, Valve};

    #[test]
    fn valve_from_str_works() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let valve = Valve::from(input);
        println!("{:?}", valve)
    }

    #[test]
    fn graph_works() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        let graph = Graph::new(input.lines().map(Valve::from));

        let path = a_star("DD".to_string(), "JJ".to_string(), &graph);
        println!("{:?}", path)
    }

    #[test]
    fn examples_work() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        let mut runner = Runner::from(input);
        runner.run();

        assert_eq!(runner.flown, 1651);
    }
}