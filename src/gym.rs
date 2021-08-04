extern crate rand;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::maze;
use maze::Maze;

const NPC_SIZE: i32 = 32;
const PLAYER_SIZE: i32 = 32;
const CAM_W: u32 = 1280;
const CAM_H: u32 = 960;
const BG_W: u32 = 1380;
const BG_H: u32 = 1980;

pub fn determine_cur_bg(player_x: i32, player_y: i32) -> Rect {

    // This is the position rectangle for displaying the maze
    let cur_bg = Rect::new (
        (player_x + (PLAYER_SIZE / 2) - ((CAM_W / 2) as i32)).clamp(0, (BG_W - CAM_W) as i32),
        (player_y + (PLAYER_SIZE / 2) - ((CAM_H / 2) as i32) + 450).clamp(0, (BG_H - CAM_H)  as i32),
        CAM_W,
        CAM_H,
    );
    return cur_bg;
}

pub fn draw_gym(wincan: &mut WindowCanvas, maze: Maze, gym_no: usize, player_x: i32, player_y: i32 ) -> Vec<Rect> {
    let x_increment: u32;
    let y_increment: u32;
    let bg_color : Color;
    let wall_color : Color; 
    if gym_no == 0 {
        bg_color = Color::RGB(0x57, 0x89, 0xA2);
        wall_color = Color::RGB(0x60, 0x29, 0x8A);
        x_increment = 140;
        y_increment = 70;
    } else if gym_no == 1 {
        bg_color = Color::RGB(0x52, 0x83, 0x4A);
        wall_color = Color::RGB(0xA8, 0x2D, 0x16);
        x_increment = 212;
        y_increment = 79;
    } else if gym_no == 2 {
        bg_color = Color::RGB(0x29, 0x39, 0x6A);
        wall_color = Color::RGB(0xEE, 0xB0, 0x45);
        x_increment = 79;
        y_increment = 70;
    } else {
        bg_color = Color::RGB(0x5A, 0x4A, 0x41);
        wall_color = Color::RGB(0xBD, 0x41, 0x41);
        x_increment = 85;
        y_increment = 70;
    }

    let gym_screen = Rect::new((0) as i32, (0) as i32, (1280) as u32, (720) as u32);
    let texture_creator = wincan.texture_creator();
    wincan.set_draw_color(bg_color);
    wincan.fill_rect(gym_screen).unwrap();
    wincan.set_draw_color(wall_color);

    let mut collision_vec = Vec::new();

    let gym_maze = maze;

    let mut y1 = 0;
    let mut y2 = y_increment as i32;

    let cur_bg = determine_cur_bg(player_x, player_y);

    // The walls are making up the "background", we need to push their position in regards to the camera to the collision vec
    for row in 0..gym_maze.maze_height {
        let mut x_tw_lw_bw = 0;
        let mut x_rw = x_increment as i32;
        //let mut row = 0;
        for container in 0..gym_maze.maze[row].len() {
            if row == 0 {
                if gym_maze.maze[row][container].top_wall {
                    let container_to_add = Rect::new(x_tw_lw_bw - cur_bg.x(), y1 - cur_bg.y(), x_increment, 5);
                    collision_vec.push(container_to_add.clone());
                    wincan.fill_rect(container_to_add).unwrap();
                }
            }
            if gym_maze.maze[row][container].left_wall {
                let container_to_add = Rect::new(x_tw_lw_bw - cur_bg.x(), y1 - cur_bg.y(), 5, y_increment + 5);
                collision_vec.push(container_to_add.clone());
                wincan.fill_rect(container_to_add).unwrap();
            }

            if gym_maze.maze[row][container].right_wall {
                let container_to_add = Rect::new(x_rw - cur_bg.x(), y1 - cur_bg.y(), 5, y_increment + 5);
                collision_vec.push(container_to_add.clone());
                wincan.fill_rect(container_to_add).unwrap();
            }
            if gym_maze.maze[row][container].bottom_wall {
                let container_to_add = Rect::new(x_tw_lw_bw - cur_bg.x(), y2 - cur_bg.y(), x_increment, 5);
                collision_vec.push(container_to_add.clone());
                wincan.fill_rect(container_to_add).unwrap();
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

    let top_entrance = Rect::new(1150,0, x_increment, 5);
    collision_vec.push(top_entrance.clone());

    
    let bottom_entrance = Rect::new(1150, y_increment as i32, x_increment, 5);
    collision_vec.push(bottom_entrance.clone());

    return collision_vec;
}

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

pub fn draw_npc(wincan: &mut WindowCanvas, maze: &Maze, gym_no: usize, player_x: i32, player_y: i32) -> (Vec<Rect>, Rect) {
    let x_increment: u32;
    let y_increment: u32;
    let y_adjust: i32;
    if gym_no == 0 {
        x_increment = 140;
        y_increment = 70;
        y_adjust = 2;
    } else if gym_no == 1 {
        x_increment = 212;
        y_increment = 79;
        y_adjust = 2;
    } else if gym_no == 2 {
        x_increment = 79;
        y_increment = 70;
        y_adjust = 2;
    } else {
        x_increment = 85;
        y_increment = 70;
        y_adjust = 2;
    }

    let texture_creator = wincan.texture_creator();
    let npc_sheet = texture_creator.load_texture("images/NPC_1.png").unwrap();

    let mut npc_collection = Vec::new();
    let gym_maze = maze;

    let mut top_y = 0;

    let x_adjust: i32 = (1280 / gym_maze.maze_width / 2) as i32 - 20 as i32;

    let mut left_x;
    let cur_bg = determine_cur_bg(player_x, player_y);

    for row in 0..gym_maze.maze_height - 1 {
        left_x = 0;
        for container in 0..gym_maze.maze[row].len() {
            if gym_maze.maze[row][container].determine_corner()
                && gym_maze.maze[row][container].let_spawn
            {
                let npc_container = Rect::new(
                    left_x + 6 + x_adjust - cur_bg.x(),
                    top_y + 6 + y_adjust - cur_bg.y(),
                    NPC_SIZE as u32,
                    NPC_SIZE as u32,
                );
                npc_collection.push(npc_container.clone());
                wincan.copy(&npc_sheet, None, npc_container).unwrap();
            }
            left_x += x_increment as i32;
        }
        top_y += y_increment as i32;
    }

    let npc_container = Rect::new(
        6 + x_adjust - cur_bg.x(),
        top_y + 6 + y_adjust - cur_bg.y(),
        NPC_SIZE as u32,
        NPC_SIZE as u32,
    );
    let npc_sheet = texture_creator.load_texture("images/boss.png").unwrap();
    wincan.copy(&npc_sheet, None, npc_container).unwrap();

    return (npc_collection, npc_container);
}

pub fn gym_coordinates(gym_index: usize) -> (i32, i32) {
    return match gym_index {
        0 => {(410, 260)}
        1 => {(1190, 600)}
        2 => {(880, 400)}
        _ => {(380, 600)}
    }
}
