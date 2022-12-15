use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn points_between(pos1: &Pos, pos2: &Pos) -> Vec<Pos> {
        let delta = (pos2.x - pos1.x, pos2.y - pos1.y);
        let mut temp = pos1.clone();
        let mut points = vec![];

        while temp != *pos2 {
            points.push(temp);
            temp.x += delta.0.clamp(-1, 1);
            temp.y += delta.1.clamp(-1, 1);
        }

        points.push(pos2.clone());

        points
    }

    pub fn is_steady(&self, obstacles: &HashSet<Pos>) -> bool {
        vec![-1, 0, 1].into_iter().all(|delta| {
            obstacles.contains(&Pos {
                x: self.x + delta,
                y: self.y + 1,
            })
        })
    }

    pub fn fall(&mut self, obstacles: &mut HashSet<Pos>) {
        for delta_x in [0, -1, 1] {
            let pos = Pos {
                x: self.x + delta_x,
                y: self.y + 1,
            };
            if !obstacles.contains(&pos) {
                self.x = pos.x;
                self.y = pos.y;
                return;
            }
        }
    }
}

pub fn generate_rock_positions(input: &str) -> HashSet<Pos> {
    let lines: Vec<&str> = input.lines().collect();
    let mut obstacles: HashSet<Pos> = HashSet::new();

    for line in lines {
        let nodes: Vec<Pos> = line
            .split(" -> ")
            .map(|node| {
                let (x, y) = node.split_once(",").unwrap();
                Pos {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect();

        nodes.windows(2).for_each(|nodes| {
            for point in Pos::points_between(&nodes[0], &nodes[1]) {
                obstacles.insert(point);
            }
        });
    }

    obstacles
}

pub fn get_max_y(positions: &HashSet<Pos>) -> isize {
    positions.iter().map(|x| x.y).max().unwrap()
}

pub fn simulate_sand_unit(obstacles: &mut HashSet<Pos>, max_y: Option<isize>) -> bool {
    let mut pos = Pos { x: 500, y: 0 };

    while !pos.is_steady(obstacles) {
        match max_y {
            Some(val) => {
                if pos.y >= val {
                    return false;
                }
            }
            None => {}
        }

        pos.fall(obstacles);
    }

    obstacles.insert(pos);
    return true;
}
