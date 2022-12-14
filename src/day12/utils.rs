use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Pos(pub usize, pub usize);

pub type Grid = Vec<Vec<char>>;

#[derive(Eq, PartialEq)]
pub struct HeapNode {
    pub pos: Pos,
    pub distance: usize,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_neighbour_valid(pos: Pos, current_value: usize, grid: &Grid) -> bool {
    let value = grid[pos.0][pos.1] as usize;
    return value <= current_value + 1;
}

fn get_possible_neighbours(pos: &Pos, grid: &Grid) -> Vec<Pos> {
    let mut possible_neighbours = vec![];
    let current_value = grid[pos.0][pos.1] as usize;

    // top
    if pos.0 > 0 && is_neighbour_valid(Pos(pos.0 - 1, pos.1), current_value, grid) {
        possible_neighbours.push(Pos(pos.0 - 1, pos.1));
    }

    // bottom
    if pos.0 < grid.len() - 1 && is_neighbour_valid(Pos(pos.0 + 1, pos.1), current_value, grid) {
        possible_neighbours.push(Pos(pos.0 + 1, pos.1));
    }

    // left
    if pos.1 > 0 && is_neighbour_valid(Pos(pos.0, pos.1 - 1), current_value, grid) {
        possible_neighbours.push(Pos(pos.0, pos.1 - 1));
    }

    // right
    if pos.1 < grid[0].len() - 1 && is_neighbour_valid(Pos(pos.0, pos.1 + 1), current_value, grid) {
        possible_neighbours.push(Pos(pos.0, pos.1 + 1));
    }

    return possible_neighbours;
}

fn get_shortest_path(to: Pos, prev_cells: &HashMap<Pos, Option<Pos>>) -> Vec<Pos> {
    let mut path: Vec<Pos> = vec![];
    let mut u: Option<Pos> = Some(to);

    while u.is_some() {
        let u_val = u.unwrap();
        path.push(u_val);

        let prev = prev_cells.get(&u_val).unwrap();
        u = *prev;
    }

    path
}

pub fn find_pos(pattern: &str, input: &str, grid: &Grid) -> Pos {
    let input_string = input.replace("\n", "");
    let index = input_string.find(pattern).unwrap();
    Pos(index / grid[0].len(), index % grid[0].len())
}

pub fn find_all_pos(pattern: char, input: &str, grid: &Grid) -> Vec<Pos> {
    let input_string = input.replace("\n", "");
    let positions = input_string
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == pattern)
        .map(|(i, _)| Pos(i / grid[0].len(), i % grid[0].len()))
        .collect::<Vec<Pos>>();

    positions
}

pub fn find_shortest_path(start_pos: Pos, end_pos: Pos, grid: &Grid) -> Option<Vec<Pos>> {
    let mut distances: HashMap<Pos, usize> = HashMap::new();
    let mut prev_cells: HashMap<Pos, Option<Pos>> = HashMap::new();
    let mut unvisited_nodes: BinaryHeap<HeapNode> = BinaryHeap::new();

    distances.insert(start_pos, 0);
    unvisited_nodes.push(HeapNode {
        pos: start_pos,
        distance: 0,
    });

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let pos = Pos(i, j);

            if pos != start_pos {
                distances.insert(pos, usize::MAX);
            }

            prev_cells.insert(pos, None);
        }
    }

    let mut found_path = false;

    while !unvisited_nodes.is_empty() {
        let curr_node = unvisited_nodes.pop().unwrap();

        if curr_node.pos == end_pos {
            found_path = true;
            break;
        }

        let neighbours = get_possible_neighbours(&curr_node.pos, &grid);

        for neighbour in neighbours {
            let distance = distances.get(&curr_node.pos).unwrap() + 1;
            let curr_distance = distances.get(&neighbour).unwrap();

            if distance < *curr_distance {
                distances.insert(neighbour, distance);
                prev_cells.insert(neighbour, Some(curr_node.pos));
                unvisited_nodes.push(HeapNode {
                    pos: neighbour,
                    distance,
                })
            }
        }
    }

    let result = match found_path {
        true => Some(get_shortest_path(end_pos, &prev_cells)),
        false => None,
    };

    result
}
