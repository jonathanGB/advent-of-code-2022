use crate::solver::Solver;

const GRID_SIZE: usize = 99;

struct TreePart1 {
    height: i8,
    visible: bool,
}

impl TreePart1 {
    fn new(height: i8) -> Self {
        Self {
            height,
            visible: false,
        }
    }
}

#[derive(Default)]
struct Visibility {
    left: u8,
    right: u8,
    top: u8,
    bottom: u8,
}

struct TreePart2 {
    height: i8,
    visibility: Visibility,
}

struct TreePart2Info {
    height: i8,
    visibility: u8,
}

impl TreePart2 {
    fn new(height: i8) -> Self {
        Self {
            height,
            visibility: Visibility::default(),
        }
    }

    fn get_info(&self, traversal: Traversal, direction: Direction) -> TreePart2Info {
        let visibility = match (traversal, direction) {
            (Traversal::Horizontal, Direction::InOrder) => self.visibility.left,
            (Traversal::Horizontal, Direction::Reversed) => self.visibility.right,
            (Traversal::Vertical, Direction::InOrder) => self.visibility.top,
            (Traversal::Vertical, Direction::Reversed) => self.visibility.bottom,
        };

        TreePart2Info {
            height: self.height,
            visibility,
        }
    }

    fn set_visibility(&mut self, score: u8, traversal: Traversal, direction: Direction) {
        match (traversal, direction) {
            (Traversal::Horizontal, Direction::InOrder) => self.visibility.left = score,
            (Traversal::Horizontal, Direction::Reversed) => self.visibility.right = score,
            (Traversal::Vertical, Direction::InOrder) => self.visibility.top = score,
            (Traversal::Vertical, Direction::Reversed) => self.visibility.bottom = score,
        }
    }

    fn scenic_score(&self) -> u32 {
        self.visibility.left as u32
            * self.visibility.right as u32
            * self.visibility.top as u32
            * self.visibility.bottom as u32
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Traversal {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    InOrder,
    Reversed,
}

type TreeMapPart1 = Vec<Vec<TreePart1>>;
type TreeMapPart2 = Vec<Vec<TreePart2>>;

pub struct Day8Solver {}
impl Solver for Day8Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day8/input.txt").unwrap();

        let mut tree_map = TreeMapPart1::new();
        for line in file.lines() {
            tree_map.push(
                line.chars()
                    .map(|character| character.to_digit(10).unwrap() as i8)
                    .map(TreePart1::new)
                    .collect(),
            );
        }

        let mut num_trees_visible = 0;
        for traversal in [Traversal::Horizontal, Traversal::Vertical] {
            for direction in [Direction::InOrder, Direction::Reversed] {
                let is: Vec<_> = (0..GRID_SIZE).collect();
                let js: Vec<_> = match direction {
                    Direction::InOrder => (0..GRID_SIZE).collect(),
                    Direction::Reversed => (0..GRID_SIZE).rev().collect(),
                };

                for i in &is {
                    // Going over a new row/column, or going over a previously
                    // visited row/column, but from a different direction.
                    // From the outside, any perimeter tree is visible.
                    let mut tallest_tree = -1;
                    for j in &js {
                        let tree = match traversal {
                            Traversal::Horizontal => &mut tree_map[*i][*j],
                            Traversal::Vertical => &mut tree_map[*j][*i],
                        };

                        if !tree.visible && tree.height > tallest_tree {
                            num_trees_visible += 1;
                            tree.visible = true;
                        }

                        tallest_tree = tallest_tree.max(tree.height);
                    }
                }
            }
        }

        println!("Number of visible trees: {num_trees_visible}");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day8/input.txt").unwrap();

        let mut tree_map = TreeMapPart2::new();
        for line in file.lines() {
            tree_map.push(
                line.chars()
                    .map(|character| character.to_digit(10).unwrap() as i8)
                    .map(TreePart2::new)
                    .collect(),
            );
        }

        for traversal in [Traversal::Horizontal, Traversal::Vertical] {
            for direction in [Direction::InOrder, Direction::Reversed] {
                let is: Vec<_> = (0..GRID_SIZE).collect();
                let js: Vec<_> = match direction {
                    Direction::InOrder => (0..GRID_SIZE).collect(),
                    Direction::Reversed => (0..GRID_SIZE).rev().collect(),
                };

                for i in is.iter() {
                    for (mut j_index, j) in js.iter().enumerate() {
                        let tree_height = match traversal {
                            Traversal::Horizontal => tree_map[*i][*j].height,
                            Traversal::Vertical => tree_map[*j][*i].height,
                        };

                        // This loop will visit the neighbours until we either reach one
                        // that is taller, or if we reach the end of the grid. We calculate
                        // the visibility score of that tree using the previously computed
                        // scores of its neighbours (à la dynamic programming), so that we
                        // skip some work.
                        let mut visibility_score = 0;
                        loop {
                            if j_index == 0 {
                                break;
                            }

                            let neighbour_j = js[j_index - 1];
                            let neighbour_tree = match traversal {
                                Traversal::Horizontal => &tree_map[*i][neighbour_j],
                                Traversal::Vertical => &tree_map[neighbour_j][*i],
                            };

                            let neighbour_info = neighbour_tree.get_info(traversal, direction);
                            if neighbour_info.height >= tree_height
                                || neighbour_info.visibility == 0
                            {
                                visibility_score += 1;
                                break;
                            }

                            visibility_score += neighbour_info.visibility;
                            j_index -= neighbour_info.visibility as usize;
                        }

                        let tree = match traversal {
                            Traversal::Horizontal => &mut tree_map[*i][*j],
                            Traversal::Vertical => &mut tree_map[*j][*i],
                        };

                        tree.set_visibility(visibility_score, traversal, direction);
                    }
                }
            }
        }

        let max_scenic_score = tree_map
            .into_iter()
            .flatten()
            .map(|tree| tree.scenic_score())
            .max()
            .unwrap();
        println!("The maximum scenic score is: {max_scenic_score}.");
    }
}
