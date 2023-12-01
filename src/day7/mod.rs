use crate::solver::Solver;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Lines;

const FILE_SYSTEM_ALLOWED_SPACE: usize = 40_000_000;

lazy_static! {
    static ref FILE_SIZE_RE: Regex = Regex::new(r"^(\d+) .*$").unwrap();
}

#[derive(Default)]
struct Folder {
    total_size: usize,
    sub_folders: HashMap<String, Rc<RefCell<Folder>>>,
    parent_folder: Option<Rc<RefCell<Folder>>>,
}

impl Folder {
    fn new(parent_folder: Option<Rc<RefCell<Folder>>>) -> Self {
        Self {
            total_size: 0,
            sub_folders: HashMap::new(),
            parent_folder,
        }
    }
}

pub struct Day7Solver {}

impl Day7Solver {
    fn go_through_files_listing(lines: &mut Peekable<Lines>, curr_folder: Rc<RefCell<Folder>>) {
        // If already visited the folder, stop.
        if curr_folder.borrow().total_size > 0 {
            return;
        }

        let mut sub_folders = HashMap::new();
        let mut total_size = 0;
        loop {
            let file_listing = match lines.peek() {
                Some(line) => line,
                // Reached end of file, stop.
                None => return,
            };

            if file_listing.starts_with("dir ") {
                let folder_name = file_listing.strip_prefix("dir ").unwrap().to_owned();
                let sub_folder = Folder::new(Some(curr_folder.clone()));
                sub_folders.insert(folder_name, Rc::new(RefCell::new(sub_folder)));
            } else if let Some(capture) = FILE_SIZE_RE.captures(file_listing) {
                total_size += capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            } else {
                break;
            }

            // We only peeked at the line, time to actually consume it.
            lines.next();
        }

        let mut curr_folder = curr_folder.borrow_mut();
        curr_folder.total_size = total_size;
        curr_folder.sub_folders = sub_folders;
    }

    fn go_through_commands(mut lines: Peekable<Lines>, root_folder: Rc<RefCell<Folder>>) {
        let mut curr_folder = root_folder.clone();
        while let Some(command) = lines.next() {
            if command == "$ cd .." {
                let parent_folder = curr_folder.borrow().parent_folder.as_ref().unwrap().clone();
                curr_folder = parent_folder;
            } else if command == "$ cd /" {
                curr_folder = root_folder.clone();
            } else if command.starts_with("$ cd ") {
                let folder_name = command.strip_prefix("$ cd ").unwrap();
                let sub_folder = curr_folder
                    .borrow()
                    .sub_folders
                    .get(folder_name)
                    .unwrap()
                    .clone();
                curr_folder = sub_folder;
            } else if command == "$ ls" {
                Self::go_through_files_listing(&mut lines, curr_folder.clone());
            } else {
                unreachable!()
            }
        }
    }

    fn update_total_directory_sizes(folder: Rc<RefCell<Folder>>) -> usize {
        let mut sub_total_size = 0;
        let mut folder = folder.borrow_mut();
        for sub_folder in folder.sub_folders.values() {
            sub_total_size += Self::update_total_directory_sizes(sub_folder.clone());
        }

        folder.total_size += sub_total_size;
        folder.total_size
    }

    fn compute_total_directory_sizes_at_most_100k(folder: Rc<RefCell<Folder>>) -> usize {
        let mut total_size_at_most_100k = 0;
        let folder = folder.borrow();
        for sub_folder in folder.sub_folders.values() {
            total_size_at_most_100k +=
                Self::compute_total_directory_sizes_at_most_100k(sub_folder.clone());
        }

        if folder.total_size <= 100_000 {
            total_size_at_most_100k += folder.total_size;
        }
        total_size_at_most_100k
    }

    fn find_smallest_directory_bigger_than(
        threshold: usize,
        folder: Rc<RefCell<Folder>>,
    ) -> Option<usize> {
        let folder = folder.borrow();
        let smallest_directory_bigger_than = if folder.total_size >= threshold {
            Some(folder.total_size)
        } else {
            None
        };

        folder
            .sub_folders
            .values()
            .map(|sub_folder| {
                Self::find_smallest_directory_bigger_than(threshold, sub_folder.clone())
            })
            .filter_map(|sub_smallest_directory_bigger_than| sub_smallest_directory_bigger_than)
            .chain(smallest_directory_bigger_than.into_iter())
            .min()
    }
}

impl Solver for Day7Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day7/input.txt").unwrap();
        let lines = file.lines().peekable();

        let root_folder = Folder::default();
        let root_folder = Rc::new(RefCell::new(root_folder));
        Self::go_through_commands(lines, root_folder.clone());
        Self::update_total_directory_sizes(root_folder.clone());

        println!(
            "Total size of all directories being at most 100k is {}",
            Self::compute_total_directory_sizes_at_most_100k(root_folder)
        );
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day7/input.txt").unwrap();
        let lines = file.lines().peekable();

        let root_folder = Folder::default();
        let root_folder = Rc::new(RefCell::new(root_folder));
        Self::go_through_commands(lines, root_folder.clone());
        let root_folder_total_size = Self::update_total_directory_sizes(root_folder.clone());
        let space_to_delete = root_folder_total_size - FILE_SYSTEM_ALLOWED_SPACE;
        println!(
            "Smallest directory: {}.",
            Self::find_smallest_directory_bigger_than(space_to_delete, root_folder).unwrap()
        )
    }
}
