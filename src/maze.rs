use rand::{self, Rng};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Container {
    pub left_wall: bool,
    pub right_wall: bool,
    pub bottom_wall: bool,
    pub top_wall: bool,
    pub set: usize,
    pub let_spawn: bool,
}

impl Container {
    pub fn determine_corner(&self) -> bool {
        let v = vec!(self.left_wall, self.right_wall, self.top_wall, self.bottom_wall);
        let v : Vec<bool> = v.into_iter().filter(|d| *d).collect();
        return v.len() == 3;
    }

    pub fn assign_spawn(&mut self) -> bool {
        let ran = rand::thread_rng().gen_range(0..7);
        if ran == 2 {
            self.let_spawn = true;
        } else {
            self.let_spawn = false;
        }
        return self.let_spawn;
    }
}

#[derive(Debug, Clone)]
pub struct Maze {
    pub maze: Vec<Vec<Container>>,
    pub maze_width: usize,
    pub maze_height: usize,
}

impl Maze {
    // This function create the layout of all the walls in the maze, called before gameloop
    pub fn create_random_maze(height: usize, width: usize) -> Maze {
        // Create the maze
        let mut maze: Vec<Vec<Container>> = Vec::new();
        //let mut spawn_board = Hashmap::new();
        // Create the first row. No containers will be members of any set
        let mut current_row: Vec<Container> = Vec::new();
        for _n in 0..width {
            current_row.push(Container {
                bottom_wall: false,
                right_wall: false,
                top_wall: false,
                left_wall: false,
                set: 0,
                let_spawn: false,
            });
        }
        // Create the hashmap for the sets
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        // Iterate for each row of the specified height
        let mut current_height = 0;
        let last_height = height - 1;
        let last_width = width - 1;
        while current_height < height {
            // Ellers algorithm only views the current row and has no memory of previously processed rows.
            if map.is_empty() == false {
                map.drain(); //clears map for each row
            }
            // Iterate through all set members and add to hashmap
            let mut current_index = 0;
            for current_container in current_row.iter() {
                // If set number is not present in hashmap, insert.
                if map.is_empty() || !map.contains_key(&current_container.set) {
                    let mut index_vector = Vec::new();
                    index_vector.insert(0, current_index);
                    map.insert(current_container.set, index_vector);
                } else {
                    //Otherwise, add to values of specified key.
                    let mut current_vector_at_key =
                        map.get(&current_container.set).unwrap().clone();
                    map.remove(&current_container.set);
                    current_vector_at_key.insert(current_vector_at_key.len(), current_index);
                    map.insert(current_container.set, current_vector_at_key.to_vec());
                }
                current_index += 1;
            }
            //Join any containers not members of a set to their own unique set
            // If hashmap of sets contains non-members, remove from hashmap and insert into another vector.
            if map.contains_key(&0) {
                let mut non_members = map.get(&0).unwrap().clone();
                map.remove(&0).unwrap();
                // While the non-members vector has entries in it, iterate through and assign an index to each entry and
                // insert in next available mapping in hashmap
                let current_container = 0;
                while non_members.is_empty() == false {
                    //Assign new mapping
                    let mut grouping = 1;
                    while current_row[non_members[current_container]].set == 0 {
                        if map.contains_key(&grouping) == false {
                            current_row[non_members[current_container]].set = grouping;
                            let mut vec_to_insert = Vec::new();
                            vec_to_insert.insert(0, non_members[current_container]);
                            map.insert(grouping, vec_to_insert);
                        } else {
                            grouping += 1;
                        }
                    }
                    // removes the entry from the non-members vector
                    non_members.remove(current_container);
                }
            }

            //Create right-walls, moving from left to right:
            let mut current_container = 0;
            while current_container < width - 1 {
                // If in same set, then create a wall betweem them
                if current_row[current_container].set != current_row[current_container + 1].set {
                    let mut rng = rand::thread_rng();
                    let n1: u8 = rng.gen_range(0..10);
                    if n1 == 3 || n1 == 8 || n1 == 5 {
                        current_row[current_container].right_wall = true;
                    } else {
                        //otherwise, union the current container and container to the right
                        let mut union_component_one = map
                            .get(&current_row[current_container].set)
                            .unwrap()
                            .clone();
                        let union_component_two = map
                            .get(&current_row[current_container + 1].set)
                            .unwrap()
                            .clone();
                        map.remove(&current_row[current_container].set);
                        map.remove(&current_row[current_container + 1].set);
                        let union_component_set_value: usize = current_row[current_container].set;
                        union_component_one.extend(union_component_two);
                        for n in 0..union_component_one.len() {
                            current_row[union_component_one[n]].set = union_component_set_value;
                        }
                        map.insert(union_component_set_value, union_component_one.to_vec());
                    }
                } else {
                    current_row[current_container].right_wall = true;
                    current_container += 1;
                    continue;
                }
                current_container += 1;
            }

            // Create bottom-walls, moving from left to right: Randomly decide to add a wall or not.
            // Make sure that each set has at least one container without a bottom-wall
            let mut mapping_clone = map.clone();
            let mut index_of_container = 0;
            let _v = current_row.clone();
            for container in &mut current_row {
                let mut rng = rand::thread_rng();
                let n1: u8 = rng.gen_range(0..10);
                if n1 == 3 || n1 == 8 || n1 == 5 {
                    let set = mapping_clone.get_mut(&container.set).unwrap();
                    //If a container is the only member of its set, do not create a bottom-wall
                    if set.len() == 1 {
                        container.bottom_wall = false;
                    } else {
                        let mut element = 0;
                        while element < set.len() {
                            if set[element] == index_of_container {
                                set.remove(element);
                                continue;
                            }
                            element += 1;
                        }
                        container.bottom_wall = true;
                    }
                }
                index_of_container += 1;
            }

            // Add top wall to every element in first row
            if current_height == 0 {
                let mut iterate = 0;
                while iterate < current_row.len() {
                    current_row[iterate].top_wall = true;
                    iterate += 1;
                }
            }
            current_row[0].left_wall = true;
            // Keep adding rows until desired height reached
            if current_height < last_height {
                //output row
                let row_clone = current_row.clone();
                maze.insert(maze.len(), row_clone);
                //remove right walls, containers with bottom wall from set, remove all bottom walls
                for current_container in current_row.iter_mut() {
                    current_container.right_wall = false;
                    if current_container.bottom_wall {
                        current_container.set = 0;
                        current_container.bottom_wall = false;
                    }
                }
                current_row[last_width].right_wall = true;
            }
            current_height += 1;
        }

        // Add bottom wall to every container in the row
        let mut current_container = 0;
        while current_container < current_row.len() - 1 {
            current_row[current_container].bottom_wall = true;
            // if adjacent cell have members in different cell , remove right wall and union sets
            if current_row[current_container].set == current_row[current_container + 1].set {
                current_container += 1;
                continue;
            } else {
                current_row[current_container].right_wall = false;
                let mut union_component_one = map
                    .get(&current_row[current_container].set)
                    .unwrap()
                    .clone();
                let union_component_two = map
                    .get(&current_row[current_container + 1].set)
                    .unwrap()
                    .clone();
                map.remove(&current_row[current_container].set);
                map.remove(&current_row[current_container + 1].set);
                let union_component_set_value: usize = current_row[current_container].set;
                union_component_one.extend(union_component_two);
                for n in 0..union_component_one.len() {
                    current_row[union_component_one[n]].set = union_component_set_value;
                }
                map.insert(union_component_set_value, union_component_one.to_vec());
            }
            current_container += 1;
        }

        current_row[last_width].bottom_wall = true;
        // output final row
        maze.insert(maze.len(), current_row);

        // start to initialize all the containers randomly for if they can spawn NPCs
        for row in 0..height {
            for column in 0..width {
                maze[row][column].assign_spawn();
            }
        }
        return Maze {
            maze: maze,
            maze_height: height,
            maze_width: width,
        };
    }
}

pub fn reload_maze(gym_mazes: &mut Vec<Maze>, gym_no: usize) {
    match gym_no {
        0 => { gym_mazes[0] = Maze::create_random_maze(16, 9); }
        1 => { gym_mazes[1] = Maze::create_random_maze(9, 6); }
        2 => { gym_mazes[2] = Maze::create_random_maze(20, 16); }
        _ => { gym_mazes[3] = Maze::create_random_maze(15, 15); }
    }
}