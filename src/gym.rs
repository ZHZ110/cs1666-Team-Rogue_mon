extern crate rand;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::maze;
use maze::Maze;

pub fn display_gym_menu(wincan: &mut WindowCanvas) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();
    let display_gym_box = texture_creator
        .load_texture("images/enterbuilding.png")
        .unwrap();

    let display_box = Rect::new(500, 200, 200, 200);
    wincan.copy(&display_gym_box, None, display_box)?;

    Ok(())
}

pub fn display_exit_gym_menu(wincan: &mut WindowCanvas) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();
    let display_gym_box = texture_creator.load_texture("images/exit_gym.png").unwrap();

    let display_box = Rect::new(400, 200, 500, 300);
    wincan.copy(&display_gym_box, None, display_box)?;

    Ok(())
}

pub fn draw_gym(wincan: &mut WindowCanvas, maze: Maze, gym_no: usize) -> Vec<Rect> {
    let x_increment : u32;
    let y_increment : u32;
    if gym_no == 1 { 
        x_increment = 140;
        y_increment = 44;
    } else if gym_no == 2 {
        x_increment = 212;
        y_increment = 79;
    } else if gym_no == 3 {
        x_increment = 79;
        y_increment = 35;
    } else {
        x_increment = 85;
        y_increment = 47;
    }

    let gym_screen = Rect::new((0) as i32, (0) as i32, (1280) as u32, (720) as u32);
    let texture_creator = wincan.texture_creator();
    let maze_sheet = texture_creator.load_texture("images/maze.png").unwrap();
    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.fill_rect(gym_screen).unwrap();

    let mut collision_vec = Vec::new();

    let gym_maze = maze;

    let mut y1 = 0;
    let mut y2 = y_increment as i32;

    for row in 0..gym_maze.maze_height {
        let mut x_tw_lw_bw = 0;
        let mut x_rw = x_increment as i32;
        //let mut row = 0;
        for container in 0..gym_maze.maze[row].len() {
            if row == 0 {
                if gym_maze.maze[row][container].top_wall == true {
                    let container_to_add = Rect::new(x_tw_lw_bw, y1, x_increment, 5);
                    collision_vec.push(container_to_add.clone());
                    wincan.copy(&maze_sheet, None, container_to_add).unwrap();
                }
            }
            if gym_maze.maze[row][container].left_wall == true {
                let container_to_add = Rect::new(x_tw_lw_bw, y1, 5, y_increment + 5);
                collision_vec.push(container_to_add.clone());
                wincan.copy(&maze_sheet, None, container_to_add).unwrap();
            }

            if gym_maze.maze[row][container].right_wall == true {
                let container_to_add = Rect::new(x_rw, y1, 5, y_increment + 5);
                collision_vec.push(container_to_add.clone());
                wincan.copy(&maze_sheet, None, container_to_add).unwrap();
            }
            if gym_maze.maze[row][container].bottom_wall == true {
                let container_to_add = Rect::new(x_tw_lw_bw, y2, x_increment, 5);
                collision_vec.push(container_to_add.clone());
                wincan.copy(&maze_sheet, None, container_to_add).unwrap();
            }

            x_tw_lw_bw += x_increment as i32;
            x_rw += x_increment as i32;
        }
        y1 += y_increment as i32;
        y2 += y_increment as i32;
    }
    let start_sheet = texture_creator.load_texture("images/start.png").unwrap();
    let start_box = Rect::new(1270, 0, 140, 80);
    collision_vec.push(start_box.clone());
    wincan.copy(&start_sheet, None, start_box).unwrap();

    return collision_vec;
}