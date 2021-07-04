extern crate sdl2;

// Modules
mod battle;
mod overworld;
mod player;

use player::Player;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use std::collections::HashSet;
use std::path::Path;

const TITLE: &str = "Monster Town Week 3";
const TILE_SIZE: u32 = 16;

// Camera
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const VSYNC: bool = true;

const MAX_SPEED: i32 = 5;
const ACCEL_RATE: i32 = 1;

const _SCALE_UP: i16 = 3;

const BUFFER_FRAMES: u32 = 0;

enum Map {
  Overworld,
  Battle,
}

fn resist(vel: i32, deltav: i32) -> i32 {
  if deltav == 0 {
    if vel > 0 {
      -1
    } else if vel < 0 {
      1
    } else {
      deltav
    }
  } else {
    deltav
  }
}

fn check_collision(a: &Rect, b: &Rect) -> bool {
  if a.bottom() < b.top() || a.top() > b.bottom() || a.right() < b.left() || a.left() > b.right() {
    false
  } else {
    true
  }
}

pub fn init(
  title: &str,
  vsync: bool,
  width: u32,
  height: u32,
) -> Result<(sdl2::render::WindowCanvas, sdl2::EventPump), String> {
  let sdl_cxt = sdl2::init()?;
  let video_subsys = sdl_cxt.video()?;

  let window = video_subsys
    .window(title, width, height)
    .build()
    .map_err(|e| e.to_string())?;

  let wincan = window.into_canvas().accelerated();

  // Check if we should lock to vsync
  let wincan = if vsync {
    wincan.present_vsync()
  } else {
    wincan
  };

  let wincan = wincan.build().map_err(|e| e.to_string())?;

  let event_pump = sdl_cxt.event_pump()?;

  let _cam = Rect::new(0, 0, CAM_W, CAM_H);

  Ok((wincan, event_pump))
}

fn run(
  wincan: &mut sdl2::render::WindowCanvas,
  event_pump: &mut sdl2::EventPump,
) -> Result<(), String> {
  // Texture
  let texture_creator = wincan.texture_creator();

  let gym_1 = texture_creator.load_texture("images/GymV6.png")?;
  let gym_2 = texture_creator.load_texture("images/GymV7.png")?;
  let gym_3 = texture_creator.load_texture("images/GymV3.png")?;
  let gym_4 = texture_creator.load_texture("images/GymV2.png")?;
  let hospital = texture_creator.load_texture("images/center.png")?;
  let home = texture_creator.load_texture("images/home.png")?;
  let battle_bg = texture_creator.load_texture("images/battle_bg.png")?;
  let npc_static = texture_creator.load_texture("images/NPC_1.png")?;
  let npc_moving = texture_creator.load_texture("images/move_npc.png")?;

  wincan.set_blend_mode(BlendMode::Blend);

  let mut loaded_map = Map::Battle;

  let player_monster = "deer pokemon";
  let enemy_monster = "Chromacat";

  let pi = format!("images/{}.png", player_monster);
  let ei = format!("images/{}.png", enemy_monster);

  let player_texture = texture_creator.load_texture(pi)?;
  let enemy_texture = texture_creator.load_texture(ei)?;
  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
  let font_path = Path::new(r"./fonts/framd.ttf");
  let font = ttf_context.load_font(font_path, 256)?;

  let player_moves = vec!["a", "b", "c", "d"]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

  let player_e = vec!["w", "x", "y", "z"]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

  let (attacks, effects) =
    battle::create_attack_tuples(&texture_creator, &font, &player_moves, &player_e)?;

  let player_mon = &player_monster.to_string();
  let enemy_mon = &enemy_monster.to_string();

  let player_health: f32 = 100.0;
  let enemy_health: f32 = 100.0;

  let (player_name_tup, enemy_name_tup) =
    battle::create_name_tuples(&texture_creator, &font, &player_mon, &enemy_mon)?;

  let mut battle_init = battle::Battle {
    player_name: &player_name_tup,
    enemy_name: &enemy_name_tup,
    background_texture: &battle_bg,
    player_texture: &player_texture,
    enemy_texture: &enemy_texture,
    font: &font,
    player_attacks: &attacks,
    player_attack_effects: &effects,
    player_health: player_health,
    enemy_health: enemy_health,
  };

  let mut current_choice: i32 = 0;
  let mut selection_buffer = 0;

  let mut x_vel = 0;
  let mut y_vel = 0;

  let mut delta_x_npc1 = 0;
  let mut delta_x_npc2 = 0;
  let mut delta_x_npc3 = 0;

  let mut flip_1 = false;
  let mut flip_2 = false;
  let mut flip_3 = false;

  // Player Creation from mod player.rs
  // it has a start position
  let player = Player::create(
    Rect::new(64, 64, TILE_SIZE * 2 as u32, TILE_SIZE * 2 as u32),
    texture_creator.load_texture("images/walk1_32.png")?,
  );

  let mut player_box = Rect::new(player.x(), player.y(), player.height(), player.width());

  // Create roaming npc players

  let npc_player1 = Player::create(
    Rect::new(480,612,TILE_SIZE * 2 as u32,TILE_SIZE * 2 as u32),
    texture_creator.load_texture("images/single_npc.png")?,
  );

  //let mut npc1_box = Rect::new(npc_player1.x(), npc_player1.y(), npc_player1.height(), npc_player1.width());

  let npc_player2 = Player::create(
    Rect::new(510,430,TILE_SIZE * 2 as u32,TILE_SIZE * 2 as u32),
    texture_creator.load_texture("images/single_npc.png")?,
  );

  //let mut npc2_box = Rect::new(npc_player2.x(), npc_player2.y(), npc_player2.height(), npc_player2.width());

  let npc_player3 = Player::create(
    Rect::new(992,240,TILE_SIZE * 2 as u32,TILE_SIZE * 2 as u32),
    texture_creator.load_texture("images/single_npc.png")?,
  );

  //let mut npc3_box = Rect::new(npc_player3.x(), npc_player3.y(), npc_player3.height(), npc_player3.width());

  'gameloop: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'gameloop,
        _ => {}
      }
    }

    // Implement Keystate
    let keystate: HashSet<Keycode> = event_pump
      .keyboard_state()
      .pressed_scancodes()
      .filter_map(Keycode::from_scancode)
      .collect();

    match loaded_map {
      Map::Overworld => {
        wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        //wincan.clear();

        overworld::draw_overworld(wincan)?;

        // Create the Town Gym
        let gym_1_box = Rect::new(340, 100, 150, 150);
        wincan.copy(&gym_1, None, gym_1_box)?;

        // Create Second Town Gym
        let gym_2_box = Rect::new(1110, 450, 150, 150);
        wincan.copy(&gym_2, None, gym_2_box)?;

        // Create Third Town Gym
        let gym_3_box = Rect::new(810, 250, 150, 150);
        wincan.copy(&gym_3, None, gym_3_box)?;

        // Create Fourth Town Gym
        let gym_4_box = Rect::new(300, 450, 150, 150);
        wincan.copy(&gym_4, None, gym_4_box)?;

        //Create Hospital
        let hospital_box = Rect::new(50, 450, 150, 150);
        wincan.copy(&hospital, None, hospital_box)?;

        // Create Home Entity
        let home_box = Rect::new(610, 250, 150, 140);
        wincan.copy(&home, None, home_box)?;

        // Create several static npcs
        let npc_static_box1 = Rect::new(490,230,32,32);
        wincan.copy(&npc_static, None, npc_static_box1);
        let npc_static_box2 = Rect::new(890,430,32,32);
        wincan.copy(&npc_static, None, npc_static_box2);
        let npc_static_box3 = Rect::new(560,65,32,32);
        wincan.copy(&npc_static, None, npc_static_box3);
        let npc_static_box4 = Rect::new(322, 330,32,32);
        wincan.copy(&npc_static, None, npc_static_box4);
        let npc_static_box5 = Rect::new(240,480,32,32);
        wincan.copy(&npc_static, None, npc_static_box5);
        let npc_static_box6 = Rect::new(880,180,32,32);
        wincan.copy(&npc_static, None, npc_static_box6);

        let mut x_deltav = 0;
        let mut y_deltav = 0;
        if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) {
          y_deltav -= ACCEL_RATE;
        }
        if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
          x_deltav -= ACCEL_RATE;
        }
        if keystate.contains(&Keycode::S) || keystate.contains(&Keycode::Down) {
          y_deltav += ACCEL_RATE;
        }
        if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
          x_deltav += ACCEL_RATE;
        }

        //Utilize the resist function: slowing it down
        x_deltav = resist(x_vel, x_deltav);
        y_deltav = resist(y_vel, y_deltav);

        // not exceed speed limit
        x_vel = (x_vel + x_deltav).clamp(-MAX_SPEED, MAX_SPEED);
        y_vel = (y_vel + y_deltav).clamp(-MAX_SPEED, MAX_SPEED);

        // Try to move horizontally
        player_box.set_x(player_box.x() + x_vel);

        // Try to move vertically
        player_box.set_y(player_box.y() + y_vel);

        // Three NPCs are moving horizontally
        let mut npc1_box = Rect::new(npc_player1.x(), npc_player1.y(), npc_player1.height(), npc_player1.width());
        let mut npc2_box = Rect::new(npc_player2.x(), npc_player2.y(), npc_player2.height(), npc_player2.width());
        let mut npc3_box = Rect::new(npc_player3.x(), npc_player3.y(), npc_player3.height(), npc_player3.width());
        npc1_box.set_x((npc1_box.x() + delta_x_npc1).clamp(480,600));
        npc2_box.set_x((npc2_box.x() + delta_x_npc2).clamp(510,640));
        npc3_box.set_x((npc3_box.x() + delta_x_npc3).clamp(992,1117));

        if npc1_box.x() == 600  { flip_1 = true; }
        if npc1_box.x() == 480 { flip_1 = false; }
        if flip_1 == false { delta_x_npc1 += 1; }
        if flip_1 == true{ delta_x_npc1 -= 1;}

        if npc2_box.x() == 640  { flip_2 = true; }
        if npc2_box.x() == 510 { flip_2 = false; }
        if flip_2 == false { delta_x_npc2 += 1; }
        if flip_2 == true{ delta_x_npc2 -= 1;}
        
        if npc3_box.x() == 1117  { flip_3 = true; }
        if npc3_box.x() == 992 { flip_3 = false; }
        if flip_3 == false { delta_x_npc3 += 1; }
        if flip_3 == true{ delta_x_npc3 -= 1;}

        // Check for collision between player and gyms as well as cam bounds(need to consider trees)
        // Use the "go-back" approach to collision resolution
        if check_collision(&player_box, &gym_1_box)
          || check_collision(&player_box, &gym_2_box)
          || check_collision(&player_box, &gym_3_box)
          || check_collision(&player_box, &gym_4_box)
          || check_collision(&player_box, &hospital_box)
          || check_collision(&player_box, &home_box)
          || player_box.left() < 0
          || player_box.right() > CAM_W as i32
          || player_box.top() < 64
          || player_box.bottom() > CAM_H as i32 - 64
        {
          player_box.set_x(player_box.x() - x_vel);
          player_box.set_y(player_box.y() - y_vel);
        }

        // Check for collision between player and gyms as well as cam bounds
        // Use the "go-back" approach to collision resolution
        if check_collision(&player_box, &npc_static_box1)
          || check_collision(&player_box, &npc_static_box2)
          || check_collision(&player_box, &npc_static_box3)
          || check_collision(&player_box, &npc_static_box4)
          || check_collision(&player_box, &npc_static_box5)
          || check_collision(&player_box, &npc_static_box6)
          || check_collision(&player_box, &npc1_box)
          || check_collision(&player_box, &npc2_box)
          || check_collision(&player_box, &npc3_box)
        {
          player_box.set_x(player_box.x() - x_vel);
          player_box.set_y(player_box.y() - y_vel);
          x_vel = 0;
          y_vel = 0;

          let screen = Rect::new(0, 0, CAM_W, CAM_H);
          wincan.copy(player.texture(), None, player_box)?;

          wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
          for _i in 0..50 {
            wincan.fill_rect(screen)?;
            wincan.present();
          }
          loaded_map = Map::Battle;
          continue;
        }

        wincan.copy(player.texture(), None, player_box)?;
        //wincan.copy(npc_player1.texture(),None,npc1_box)?;
        wincan.copy_ex(
          npc_player1.texture(),
          Rect::new(0, 0, 32, 32),
          Rect::new(npc1_box.x(), npc1_box.y(), 32, 32),
          0.0,
          None,
          flip_1,
          false,
        )?;
        wincan.copy_ex(
          npc_player2.texture(),
          Rect::new(0, 0, 32, 32),
          Rect::new(npc2_box.x(), npc2_box.y(), 32, 32),
          0.0,
          None,
          flip_2,
          false,
        )?;
        wincan.copy_ex(
          npc_player3.texture(),
          Rect::new(0, 0, 32, 32),
          Rect::new(npc3_box.x(), npc3_box.y(), 32, 32),
          0.0,
          None,
          flip_3,
          false,
        )?;

        wincan.present();
      }

      Map::Battle => {
        battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;

        if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
          if selection_buffer > 0 {
            continue;
          } else {
            current_choice -= 1;
            current_choice = if current_choice > 3 {
              0
            } else if current_choice < 0 {
              3
            } else {
              current_choice
            };

            battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;
            selection_buffer = BUFFER_FRAMES;
            wincan.present();
          }
        }
        if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
          if selection_buffer > 0 {
            continue;
          } else {
            current_choice += 1;
            current_choice = if current_choice > 3 {
              0
            } else if current_choice < 0 {
              3
            } else {
              current_choice
            };
            selection_buffer = BUFFER_FRAMES;
            battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;
          }
        }
        if keystate.contains(&Keycode::Return) {
          let f = format!("You selected move #{}!", current_choice + 1);
          battle::better_draw_battle(wincan, &battle_init, current_choice as usize, Some(f))?;
        }
        if keystate.contains(&Keycode::E) {
          let screen = Rect::new(0, 0, CAM_W, CAM_H);
          wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
          for _i in 0..50 {
            wincan.fill_rect(screen)?;
            wincan.present();
          }
          loaded_map = Map::Overworld;
        }
        if keystate.contains(&Keycode::K) {
          for i in 0..101 {
            let k: i32 = ((i - 100) as i32).abs();
            battle_init.set_enemy_health(k as f32);
            battle_init.set_player_health(100.0);
            battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;
          }
        }
        if selection_buffer > 0 {
          selection_buffer -= 1;
        }
      }
    }
  }

  Ok(())
}

fn main() {
  println!("\nRunning {}:", TITLE);
  print!("\tInitting...");
  match init(TITLE, VSYNC, CAM_W, CAM_H) {
    Err(e) => println!("\n\t\tFailed to init: {}", e),
    Ok(d) => {
      println!("DONE");

      let (mut wincan, mut event_pump) = d;

      print!("\tRunning...");
      match run(&mut wincan, &mut event_pump) {
        Err(e) => println!("\n\t\tEncountered error while running: {}", e),
        Ok(_) => println!("DONE\nExiting cleanly"),
      };
    }
  };
}
