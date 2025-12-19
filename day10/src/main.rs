use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    #[allow(unused_variables)]
    let input = include_str!("../puzzle_input.txt");
    #[allow(unused_variables)]
    let test_input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let machines = input.lines().map(Machine::from);
    let total: usize = machines.map(find_fewest_steps).sum();
    println!("Total steps: {total}");
}

fn part_two(input: &str) {
    let machines = input.lines().map(Machine::from);
    let total = input.lines().count();
    let mut count = 0;

    let total: usize = machines
        .map(|m| {
            count += 1;
            eprintln!("Finding steps for machine {}/{}", count, total);
            find_fewest_jolt_steps(m)
        })
        .sum();
    println!("Total steps: {total}");
}

fn find_fewest_steps(machine: Machine) -> usize {
    // Use a HashMap and a BinaryHeap. the HashMap tracks 'states' we've already
    // reached, and maps them to the number of steps. This way when we try pressing
    // a button, we can tell if we've already reached that step, and if so, if it
    // was in fewer steps
    // The BinaryHeap tracks the states with the lowest 'cost' to the cost function
    // in the A* algorithm. We use this to find the best starting point to continue
    // iterating from.
    let mut map: HashMap<Lights, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let start_state = Lights(vec![false; machine.lights.len()]);

    map.insert(start_state.clone(), 0);
    let cost = heuristic(&start_state, &machine.target, 0);
    let start_state = State {
        lights: start_state,
        steps: 0,
        cost,
    };
    heap.push(Reverse(start_state));

    while !heap.is_empty() {
        // Safe to unwrap, we've already checked heap isn't empty
        let current = heap.pop().unwrap();
        if current.0.lights == machine.target {
            return current.0.steps;
        }

        // Iterate over neighbours
        for button in &machine.buttons {
            let neighbour = button.apply(&current.0.lights);
            // Check if neighbour is already mapped, if not insert with
            // number of steps needed to reach neighbour via this path
            let mut new_state = false;
            let entry = map.entry(neighbour.clone()).or_insert_with(|| {
                new_state = true;
                current.0.steps + 1
            });
            // if current path is quicker, (or we've never encountered
            // this state) update entry
            if new_state || current.0.steps + 1 < *entry {
                *entry = current.0.steps + 1;
                let cost = heuristic(&neighbour, &machine.target, current.0.steps + 1);
                // We should really remove old entry to heap for this state, if it's in there
                heap.push(Reverse(State {
                    lights: neighbour,
                    cost,
                    steps: current.0.steps + 1,
                }));
            }
        }
    }
    panic!("Didn't find target");
}

fn find_fewest_jolt_steps(machine: Machine) -> usize {
    let mut map: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let start_state = vec![0; machine.joltages.len()];

    map.insert(start_state.clone(), 0);
    let cost = joltage_heuristic(&start_state, &machine.joltages, 0);
    let start_state = JoltState {
        joltage: start_state,
        steps: 0,
        cost,
    };
    heap.push(Reverse(start_state));

    while !heap.is_empty() {
        let current = heap.pop().unwrap();
        if current.0.joltage == machine.joltages {
            return current.0.steps;
        }

        // Iterate over neighbours
        for button in &machine.buttons {
            let neighbour = button.apply_jolts(&current.0.joltage);
            if overjolted(&neighbour, &machine.joltages) {
                continue;
            }

            // Check if neighbour already mapped, if not insert
            let mut new_state = false;
            let to_enter = neighbour.clone();
            let entry = map.entry(to_enter).or_insert_with(|| {
                new_state = true;
                current.0.steps + 1
            });
            if new_state || current.0.steps + 1 < *entry {
                *entry = current.0.steps + 1;
                let cost = joltage_heuristic(&neighbour, &machine.joltages, current.0.steps + 1);
                heap.push(Reverse(JoltState {
                    joltage: neighbour,
                    cost,
                    steps: current.0.steps + 1,
                }));
            }
        }
    }
    panic!("Didn't find joltage target!");
}

#[derive(Clone, Debug)]
struct State {
    lights: Lights,
    cost: usize,
    steps: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for State {
    // N.B. only compare cost, since that's all we use
    // for ordering, and we want PartialEq to match our
    // Ord implementation
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

#[derive(Clone, Debug)]
struct JoltState {
    joltage: Vec<usize>,
    cost: usize,
    steps: usize,
}

impl PartialOrd for JoltState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JoltState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for JoltState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for JoltState {}

#[derive(Clone, Debug, PartialEq)]
struct Machine {
    lights: Lights,
    target: Lights,
    buttons: Vec<Button>,
    joltages: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
struct Button {
    light_idxs: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Lights(Vec<bool>);

impl FromIterator<bool> for Lights {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Lights(iter.into_iter().collect())
    }
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let light_idxs = value[1..value.len() - 1]
            .split(',')
            .map(|b| b.parse().expect("Failed to parse button idx"))
            .collect();
        Button { light_idxs }
    }
}

impl Button {
    fn apply(&self, lights: &Lights) -> Lights {
        let mut new_state = lights.clone();
        for &idx in &self.light_idxs {
            new_state.0[idx] = !new_state.0[idx];
        }
        new_state
    }

    fn apply_jolts(&self, joltage: &[usize]) -> Vec<usize> {
        let mut new_state = Vec::from(joltage);
        for &idx in &self.light_idxs {
            new_state[idx] += 1;
        }
        new_state
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let bracket_close_idx = value.find(']').expect("No close bracket for lights");
        let light_str = &value[1..bracket_close_idx];
        let lights = Lights(vec![false; light_str.len()]);
        let target = light_str
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("Found character {c} in light array."),
            })
            .collect();

        let jolt_start_idx = value.rfind('{').expect("No open bracket for joltage");
        let jolt_str = &value[jolt_start_idx + 1..value.len() - 1];
        let joltages = jolt_str
            .split(',')
            .map(|j| {
                j.parse::<usize>()
                    .unwrap_or_else(|_| panic!("Failed to parse jolt from: {}", j))
            })
            .collect();

        let buttons = &value[bracket_close_idx + 1..jolt_start_idx - 1];
        let buttons = buttons.split_whitespace().map(Button::from).collect();

        Machine {
            lights,
            target,
            buttons,
            joltages,
        }
    }
}

fn heuristic(state: &Lights, target: &Lights, steps: usize) -> usize {
    4 * steps + state.calc_dist(target)
}

fn joltage_heuristic(state: &[usize], target: &[usize], steps: usize) -> usize {
    5 * steps
        + state
            .iter()
            .zip(target.iter())
            .map(|(&a, b)| b.abs_diff(a))
            .sum::<usize>()
}

fn overjolted(state: &[usize], target: &[usize]) -> bool {
    state.iter().zip(target.iter()).any(|(a, b)| a > b)
}

impl Lights {
    fn calc_dist(&self, target: &Lights) -> usize {
        self.0
            .iter()
            .zip(target.0.iter())
            .filter(|(a, b)| a != b)
            .count()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}
