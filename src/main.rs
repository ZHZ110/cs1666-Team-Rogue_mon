extern crate sdl2;

// Modules
mod battle;
pub mod monster;
pub mod overworld;
pub mod player;
pub mod gym;
pub mod maze;
pub mod ai;
pub mod intro;

use battle::Map;

use monster::load_mons;
use monster::load_moves;
use player::Player;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use std::time::{Instant};
use std::collections::HashSet;
use std::path::Path;

use std::time::Duration;
use std::thread;

use rand::thread_rng;
use rand::{self, Rng};
use rand::seq::SliceRandom;

const TITLE: &str = "Monster Town";
const TILE_SIZE: u32 = 16;

// Camera
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const VSYNC: bool = true;

const MAX_SPEED: i32 = 5;
const ACCEL_RATE: i32 = 1;

const _SCALE_UP: i16 = 3;

const DELTA_TIME: f64 = 1.0/60.0;
//const BUFFER_FRAMES: u32 = 10;
// supposed keypress duration
const KEYPRESS_DURATION: f64 = 1.0; 

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

fn select_random_team<'a>(keys: &Vec<String>, num: usize, experience: usize) -> Vec<(String, f32, usize)> {
  let mut rng = thread_rng();
  let v : Vec<(String, f32, usize)> = (*keys)
    .choose_multiple(&mut rng, num)
    .map(|s| (s.clone(), 100.0, experience))
    .collect();
  return v
}

fn check_within(small: &Rect, large: &Rect) -> bool {
  if small.left() > large.left()
    && small.right() < large.right()
    && small.top() > large.top()
    && small.top() > large.top()
    && small.bottom() < large.bottom()
  {
    true
  } else {
    false
  }
}

fn random_spawn() -> bool {
  let mut rng = thread_rng();
  let ran = rng.gen_range(0..100);
  if ran == 2 {
    true
  } else {
    false
  }
}

fn next_available_mon(v: &Vec<(String, f32, usize)>) -> String {
  let a = String::new();
  for i in v {
    if i.1 > 0.0 {
      return i.0.clone();
    }
  }
  return a;
}

fn experience(difficulty: usize, badges: usize) -> usize {
  match difficulty {
    0 => {
      return 10*(1+badges);
    }
    1 => {
      return 30*(1+badges);
    }
    _ => {
      return 50*(1+badges);
    }
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
  let diff_texture = texture_creator.load_texture("images/difficulty_select.png")?;
  let welcome = texture_creator.load_texture("images/welcome.png")?;

  wincan.set_blend_mode(BlendMode::Blend);

  let mut loaded_map = Map::Intro;

  // Load the monsters and their moves from data files, storing them in maps from String to their Object versions
  let moves_map = load_moves();
  let monsters_map = load_mons(&moves_map);

  // Load the font used for battle text
  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
  let font_path = Path::new(r"./fonts/framd.ttf");
  let font = ttf_context.load_font(font_path, 256)?;

  // Gather the Strings of moves, effects, and monsters to turn into textures
  let all_moves = moves_map
    .keys()
    .map(|d| String::from(d))
    .collect::<Vec<String>>();
  let all_effects = moves_map
    .values()
    .map(|d| String::from(d.effect.clone()))
    .collect::<Vec<String>>();
  let all_monsters = monsters_map
    .keys()
    .map(|d| String::from(d))
    .collect::<Vec<String>>();

  // Create any text texture only once for efficiency 
  let move_textures = battle::create_all_attack_textures(&texture_creator, &font, &all_moves)?;
  let effect_textures = battle::create_all_effect_textures(&texture_creator, &font, &all_effects)?;
  let monster_textures = battle::create_all_monster_textures(&texture_creator, &all_monsters)?;
  let names_tup = battle::create_all_name_tuples(&texture_creator, &font, &all_monsters)?;

  let mut player_team: Vec<(String, f32, usize)> = Vec::new();
  player_team.push((String::from("Chromacat"), 100.0, 0));
  player_team.push((String::from("deer pokemon"), 100.0, 0));
  player_team.push((String::from("tokoro"), 100.0, 0));
  player_team.push((String::from("Shockshroom"), 100.0, 0));
  player_team.push((String::from("Gurmail"), 100.0, 0));
  player_team.push((String::from("Burhan"), 100.0, 0));

  let mut enemy_team: Vec<(String, f32, usize)> = Vec::new();
  enemy_team.push((String::from("melon-mon"), 100.0, 0));
  enemy_team.push((String::from("taterface"), 100.0, 0));

  let mut battle_draw = battle::Battle {
    background_texture: &battle_bg,
    player_name: next_available_mon(&player_team),
    enemy_name: next_available_mon(&enemy_team),
    font: &font,
    player_health: 100.0,
    enemy_health: 100.0,
    name_text_map: &names_tup,
    attack_map: &move_textures,
    effect_map: &effect_textures,
    monster_text_map: &monster_textures,
    monsters: &monsters_map,
    moves: &moves_map,
    player_level: 0,
    opp_level: 0,
  };

  let mut player_badges = HashSet::<u32>::new();

  let mut battle_state = monster::BattleState {
    player_turn: true,
    player_team: player_team.clone(),
    enemy_team: enemy_team.clone(),
    self_attack_stages: 0,
    self_defense_stages: 0,
    opp_attack_stages: 0,
    opp_defense_stages: 0,
    player_badges: 0,
    battle_type: &monster::BattleType::Wild,
  };

  let mut current_choice: i32 = 0;

  // Variables used for the monster switching menu
  let mut menu_active = false;
  let mut menu_choice: usize = 0;
  let mut menu_selected_choice: Option<usize> = None;

  // Variables used for the introduction screen and difficulty selection
  let mut intro_played = false;
  let mut difficulty_choice = 1;

  let mut x_vel = 0;
  let mut y_vel = 0;

  let mut delta_x_npc1 = 0;
  let mut delta_x_npc2 = 0;
  let mut delta_x_npc3 = 0;

  let mut flip_1 = false;
  let mut flip_2 = false;
  let mut flip_3 = false;

  let mut gym_no : usize = 1;

  // Tracking time
  let mut time_count = Instant::now();
  let mut keypress_timer: f64 = 0.0;
  let mut timer = Instant::now();

  // Player Creation from mod player with a start position
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

  let npc_player2 = Player::create(
    Rect::new(510,430,TILE_SIZE * 2 as u32,TILE_SIZE * 2 as u32),
    texture_creator.load_texture("images/single_npc.png")?,
  );

  let npc_player3 = Player::create(
    Rect::new(992,240,TILE_SIZE * 2 as u32,TILE_SIZE * 2 as u32),
    texture_creator.load_texture("images/single_npc.png")?,
  );

  let mut gym_mazes = vec!(maze::Maze::create_random_maze(16, 9), maze::Maze::create_random_maze(9, 6), maze::Maze::create_random_maze(20, 16), maze::Maze::create_random_maze(15, 15));

  'gameloop: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'gameloop,
        Event::KeyUp{keycode: Some(k), repeat: false, ..} => {
          match k {
            Keycode::W => keypress_timer = 0.0,
            Keycode::A => keypress_timer = 0.0,
            Keycode::S => keypress_timer = 0.0,
            Keycode::D => keypress_timer = 0.0,
            Keycode::Up => keypress_timer = 0.0,
            Keycode::Down => keypress_timer = 0.0,
            Keycode::Left => keypress_timer = 0.0,
            Keycode::Right => keypress_timer = 0.0,
            Keycode::Return => keypress_timer = 0.0,
            Keycode::M => keypress_timer = 0.0,
            _ => {},
          }
        }
        _ => {}
      }
    }

    // Implement Keystate
    let keystate: HashSet<Keycode> = event_pump
      .keyboard_state()
      .pressed_scancodes()
      .filter_map(Keycode::from_scancode)
      .collect();

    let elapsed = time_count.elapsed().as_secs_f64();
    let single_elapsed = timer.elapsed().as_secs_f64();
    timer = Instant::now();

    player_box.set_width(32);
    player_box.set_height(32);
    
    match loaded_map {
      Map::Intro => {
        let screen = Rect::new(0, 0, CAM_W, CAM_H);

        // Show the title/credits only on the first iteration
        if !intro_played {
          wincan.copy(&welcome, None, screen)?;
          wincan.present();
          wincan.set_draw_color(Color::RGBA(0, 0, 0, 20));

          thread::sleep(Duration::from_millis(3500));
          for _i in 0..100 {
            wincan.fill_rect(screen)?;
            wincan.present();
          }
          intro_played = true;
        }

        wincan.copy(&diff_texture, None, screen)?;
        intro::draw_intro(wincan, difficulty_choice)?;

        if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) {
          if keypress_timer == 0.0 {
            difficulty_choice = if difficulty_choice == 0 {
              2
            } else {
              difficulty_choice - 1
            }
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::S) || keystate.contains(&Keycode::Down) {
          if keypress_timer == 0.0 {
            difficulty_choice = if difficulty_choice == 2 {
              0
            } else {
              difficulty_choice + 1
            }
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::Return) {
          if keypress_timer == 0.0 {
            wincan.set_draw_color(Color::RGBA(0, 0, 0, 20));
            for _i in 0..100 {
              wincan.fill_rect(screen)?;
              wincan.present();
            }
            loaded_map = Map::Overworld;
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
      }

      Map::Overworld => {
        wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        overworld::draw_overworld(wincan)?;
        let spawnable_areas = overworld::mark_rectangles();

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

        // Create front of gym box for each gym
        // LETS GET THESE TO BE TIGHTER
        let front_of_gym_1_box = Rect::new(400,250,20,5);
        let front_of_gym_2_box = Rect::new(1180, 600, 20, 5);
        let front_of_gym_3_box = Rect::new(872, 400, 20, 5);
        let front_of_gym_4_box = Rect::new(370, 600, 20, 5);

        //Create front of building box for buildings
        let front_of_hospital_box = Rect::new(110, 600, 35, 3);

        // Create several static npcs
        let npc_static_box1 = Rect::new(490,230,32,32);
        wincan.copy(&npc_static, None, npc_static_box1)?;
        let npc_static_box2 = Rect::new(890,430,32,32);
        wincan.copy(&npc_static, None, npc_static_box2)?;
        let npc_static_box3 = Rect::new(560,65,32,32);
        wincan.copy(&npc_static, None, npc_static_box3)?;
        let npc_static_box4 = Rect::new(322, 330,32,32);
        wincan.copy(&npc_static, None, npc_static_box4)?;
        let npc_static_box5 = Rect::new(240,480,32,32);
        wincan.copy(&npc_static, None, npc_static_box5)?;
        let npc_static_box6 = Rect::new(880,180,32,32);
        wincan.copy(&npc_static, None, npc_static_box6)?;

        if keystate.contains(&Keycode::M) {
          menu_active = true;
          continue;
        }

        if menu_active {
          battle::draw_monster_menu(
            wincan,
            &battle_draw,
            &battle_state,
            menu_choice,
            menu_selected_choice,
          )?;
          if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => 6,
                1 => 6,
                2 => 0,
                3 => 1,
                4 => 2,
                5 => 3,
                _ => 2 * (player_team.len() / 2 + player_team.len() % 2 - 1),
              };
            } else {
              continue;
            }; 
            // need to calculate how much time each loop takes regarding the machine it runs on 
            // so that we know how much to increment for the keypress timer
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => {
                  if player_team.len() > 1 {
                    1
                  } else {
                    0
                  }
                }
                1 => 0,
                2 => {
                  if player_team.len() > 3 {
                    3
                  } else {
                    0
                  }
                }
                3 => 2,
                4 => {
                  if player_team.len() > 5 {
                    5
                  } else {
                    0
                  }
                }
                5 => 4,
                _ => 6,
              };
            } else {
              continue;
            };
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::S) || keystate.contains(&Keycode::Down) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => {
                  if player_team.len() > 2 {
                    2
                  } else {
                    6
                  }
                }
                1 => {
                  if player_team.len() > 3 {
                    3
                  } else {
                    6
                  }
                }
                2 => {
                  if player_team.len() > 4 {
                    4
                  } else {
                    6
                  }
                }
                3 => {
                  if player_team.len() == 6 {
                    5
                  } else {
                    6
                  }
                }
                4 => 6,
                5 => 6,
                _ => 0,
              };
            } else {
              continue;
            };
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => {
                  if player_team.len() > 1 {
                    1
                  } else {
                    0
                  }
                }
                1 => 0,
                2 => {
                  if player_team.len() > 3 {
                    3
                  } else {
                    0
                  }
                }
                3 => 2,
                4 => {
                  if player_team.len() > 5 {
                    5
                  } else {
                    0
                  }
                }
                5 => 4,
                _ => 6,
              };
            } else {
              continue;
            }
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::Return) {
            if keypress_timer == 0.0 {
              if menu_choice == 6 {
                menu_active = false;
                menu_selected_choice = None;
                battle_state.player_team = battle::verify_team(&battle_state.player_team);
                continue;
              }
              match menu_selected_choice {
                Some(choice) => {
                  if choice != menu_choice {
                    battle_state.player_team.swap(choice, menu_choice);
                    menu_selected_choice = None;
                  }
                }
                None => {
                  menu_selected_choice = Some(menu_choice);
                }
              }
            } else {
              continue;
            };
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          continue;
        }

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
        let mut npc1_box = Rect::new(
          npc_player1.x(),
          npc_player1.y(),
          npc_player1.height(),
          npc_player1.width(),
        );
        let mut npc2_box = Rect::new(
          npc_player2.x(),
          npc_player2.y(),
          npc_player2.height(),
          npc_player2.width(),
        );
        let mut npc3_box = Rect::new(
          npc_player3.x(),
          npc_player3.y(),
          npc_player3.height(),
          npc_player3.width(),
        );
        npc1_box.set_x((npc1_box.x() + delta_x_npc1).clamp(480, 600));
        npc2_box.set_x((npc2_box.x() + delta_x_npc2).clamp(510, 640));
        npc3_box.set_x((npc3_box.x() + delta_x_npc3).clamp(992, 1117));

        if npc1_box.x() == 600  { flip_1 = true; }
        if npc1_box.x() == 480 { flip_1 = false; }
        if flip_1 == false && ((elapsed * 100.0).round() % (DELTA_TIME * 100.0).round() == 0.0)
          { delta_x_npc1 += 1; }
        if flip_1 == true && ((elapsed * 100.0).round() % (DELTA_TIME * 100.0).round() == 0.0)
          { delta_x_npc1 -= 1;}

        if npc2_box.x() == 640  { flip_2 = true; }
        if npc2_box.x() == 510 { flip_2 = false; }
        if flip_2 == false && ((elapsed * 100.0).round() % (DELTA_TIME * 100.0).round() == 0.0)
          { delta_x_npc2 += 1; }
        if flip_2 == true && ((elapsed * 100.0).round() % (DELTA_TIME * 100.0).round() == 0.0)
          { delta_x_npc2 -= 1;}
        
        if npc3_box.x() == 1117  { flip_3 = true; }
        if npc3_box.x() == 992 { flip_3 = false; }
        if flip_3 == false && ((elapsed * 100.0).round() % (DELTA_TIME * 100.0).round() == 0.0)
          { delta_x_npc3 += 1; }
        if flip_3 == true && ((elapsed * 100.0).round() % (DELTA_TIME * 100.0).round() == 0.0)
          { delta_x_npc3 -= 1;}
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

        if check_collision(&player_box, &front_of_gym_1_box)
        {
          gym::display_gym_menu(wincan)?;
          if keystate.contains(&Keycode::Y)
          {
            loaded_map = Map::GymOne;
            player_box.set_x(1200);
            player_box.set_y(7);
          }
        }
        if check_collision(&player_box, &front_of_gym_2_box)
        {
          gym::display_gym_menu(wincan)?;
          if keystate.contains(&Keycode::Y)
          {
            loaded_map = Map::GymTwo;
            player_box.set_x(1200);
            player_box.set_y(7);
          }
          
        }
        if check_collision(&player_box, &front_of_gym_3_box)
        {
          gym::display_gym_menu(wincan)?;
          if keystate.contains(&Keycode::Y)
          {
            loaded_map = Map::GymThree;
            player_box.set_x(1200);
            player_box.set_y(7);
          }
          
        }
        if check_collision(&player_box, &front_of_gym_4_box)
        {
          gym::display_gym_menu(wincan)?;
          if keystate.contains(&Keycode::Y)
          {
            loaded_map = Map::GymFour;
            player_box.set_x(1200);
            player_box.set_y(7);
          }
          
        }
      
        if check_collision(&player_box, &front_of_hospital_box)
        {
          let screen = Rect::new(0,0,CAM_W,CAM_H);
          wincan.set_draw_color(Color::RGBA(0, 0, 0, 20));
          for _i in 0..100 {
            wincan.fill_rect(screen)?;
            wincan.present();
          }
          for item in battle_state.player_team.iter_mut() {
            item.1 = 100.0;
          }
          battle_draw.player_health = 100.0;
          player_box.set_x(player_box.x() - x_vel);
          player_box.set_y(player_box.y() - y_vel);
          x_vel = 0;
          y_vel = 0;
          continue;
        }

        for i in &spawnable_areas {
          if check_within(&player_box, i) && random_spawn() 
            && ((elapsed  * 100.0).round()) % ((DELTA_TIME* 100.0).round()) == 0.0 
            && (elapsed > 3.0) {
            let screen = Rect::new(0, 0, CAM_W, CAM_H);
            wincan.copy(player.texture(), None, player_box)?;
            wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
            for _i in 0..50 {
              wincan.fill_rect(screen)?;
              wincan.present();
            }
            loaded_map = Map::Battle;
            battle_draw.enemy_health = 100.0;

            let enemy_team = select_random_team(&all_monsters, 1, experience(difficulty_choice, battle_state.player_badges));

            let enemy_monster = enemy_team[0].0.clone();
            battle_draw.enemy_name = enemy_monster.clone();
            let player_monster = next_available_mon(&player_team);
            battle_draw.player_name = player_monster.clone();

            battle_state = monster::BattleState {
              player_turn: monsters_map[&player_monster].attack_stat
                >= monsters_map[&enemy_monster].attack_stat,
              player_team: battle_state.player_team.clone(),
              enemy_team: enemy_team.clone(),
              self_attack_stages: 0,
              self_defense_stages: 0,
              opp_attack_stages: 0,
              opp_defense_stages: 0,
              player_badges: battle_state.player_badges,
              battle_type: &monster::BattleType::Wild,
            };

            player_box.set_x(player_box.x() - x_vel);
            player_box.set_y(player_box.y() - y_vel);
            break;
          }
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
          wincan.copy(player.texture(), None, player_box)?;
          wincan.copy(npc_player1.texture(), None, npc1_box)?;
          wincan.copy(npc_player2.texture(), None, npc2_box)?;
          wincan.copy(npc_player3.texture(), None, npc3_box)?;

          overworld::display_menu(wincan, player_box.x(), player_box.y())?;

          if keystate.contains(&Keycode::F) {
            let enemy_team = select_random_team(&all_monsters, 2, experience(difficulty_choice, battle_state.player_badges));
            battle_draw.opp_level = enemy_team[0].2 / 10;
            let enemy_team = battle::verify_team(&enemy_team);

            let enemy_monster = enemy_team[0].0.clone();
            battle_draw.enemy_name = enemy_monster.clone();
            let player_monster = next_available_mon(&battle_state.player_team);
            battle_draw.player_name = player_monster.clone();

            battle_state = monster::BattleState {
              player_turn: true,
              player_team: battle_state.player_team.clone(),
              enemy_team: enemy_team.clone(),
              self_attack_stages: 0,
              self_defense_stages: 0,
              opp_attack_stages: 0,
              opp_defense_stages: 0,
              player_badges: battle_state.player_badges,
              battle_type: &monster::BattleType::Trainer,
            };

            loaded_map = Map::Battle;
            battle_draw.enemy_health = 100.0;

            wincan.present();
            wincan.clear();
            battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;

            x_vel = 0;
            y_vel = 0;

            continue;
          }

          //flashing not active when moving away

          if keystate.contains(&Keycode::W)
            || keystate.contains(&Keycode::Up)
            || keystate.contains(&Keycode::A)
            || keystate.contains(&Keycode::Left)
            || keystate.contains(&Keycode::S)
            || keystate.contains(&Keycode::Down)
            || keystate.contains(&Keycode::D)
            || keystate.contains(&Keycode::Right)
          {
            wincan.present();

            x_vel = 0;
            y_vel = 0;

            continue;
          }

          //causes the flashing effect. Every time near npc, screen flashes
          wincan.present();
          wincan.present();

          x_vel = 0;
          y_vel = 0;

          continue;
        }

        wincan.copy(player.texture(), None, player_box)?;
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
      },

      Map::Battle => {
        if menu_active {
          battle::draw_monster_menu(
            wincan,
            &battle_draw,
            &battle_state,
            menu_choice,
            menu_selected_choice,
          )?;
          if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => 6,
                1 => 6,
                2 => 0,
                3 => 1,
                4 => 2,
                5 => 3,
                _ => 2 * (player_team.len() / 2 + player_team.len() % 2 - 1),
              };
            } else {
              continue;
            };
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => {
                  if player_team.len() > 1 {
                    1
                  } else {
                    0
                  }
                }
                1 => 0,
                2 => {
                  if player_team.len() > 3 {
                    3
                  } else {
                    0
                  }
                }
                3 => 2,
                4 => {
                  if player_team.len() > 5 {
                    5
                  } else {
                    0
                  }
                }
                5 => 4,
                _ => 6,
              };
            } else {
              continue;
            }
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::S) || keystate.contains(&Keycode::Down) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => {
                  if player_team.len() > 2 {
                    2
                  } else {
                    6
                  }
                }
                1 => {
                  if player_team.len() > 3 {
                    3
                  } else {
                    6
                  }
                }
                2 => {
                  if player_team.len() > 4 {
                    4
                  } else {
                    6
                  }
                }
                3 => {
                  if player_team.len() == 6 {
                    5
                  } else {
                    6
                  }
                }
                4 => 6,
                5 => 6,
                _ => 0,
              };
            } else {
              continue;
            };
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
            if keypress_timer == 0.0 {
              menu_choice = match menu_choice {
                0 => {
                  if player_team.len() > 1 {
                    1
                  } else {
                    0
                  }
                }
                1 => 0,
                2 => {
                  if player_team.len() > 3 {
                    3
                  } else {
                    0
                  }
                }
                3 => 2,
                4 => {
                  if player_team.len() > 5 {
                    5
                  } else {
                    0
                  }
                }
                5 => 4,
                _ => 6,
              };
            } else {
              continue;
            };
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          if keystate.contains(&Keycode::Return) {
            if keypress_timer == 0.0 {
              if menu_choice == 6 {
                menu_active = false;
                menu_selected_choice = None;
                battle_state.player_team = battle::verify_team(&battle_state.player_team);

                let mut switched_front : &(String, f32, usize) = &(String::from(""), 0.0, 0);
                for i in 0..battle_state.player_team.len() {
                  if battle_state.player_team[i].1 > 0.0 {
                    switched_front = &battle_state.player_team[i];
                    break;
                  }
                }

                if battle_draw.player_name != switched_front.0 {
                  let new_mon = String::from(switched_front.0.clone());
                  let f = format!("You switched in {}!", new_mon);
                  battle_draw.player_name = new_mon.clone();
                  battle_draw.player_health = switched_front.1;
                  battle_state.self_attack_stages = 0;
                  battle_state.self_defense_stages = 0;
                  battle::draw_battle(wincan, &battle_draw, None, Some(f))?;

                  let enemy_choice = ai::ai_agent(difficulty_choice, &monsters_map, &mut battle_state);
                  match battle::enemy_battle_turn(
                    wincan,
                    &mut battle_state,
                    &mut battle_draw,
                    &monsters_map,
                    enemy_choice,
                  )? {
                    Map::Overworld => {
                      loaded_map = Map::Overworld;
    
                      // Have the player spawn at the hospital with full health
                      player_box.set_x(112);
                      player_box.set_y(604);

                      for i in 0..battle_state.player_team.len() {
                        battle_state.player_team[i].1 = 100.0;
                      }
                      battle_draw.player_health = 100.0;
                      continue;
                    }
                    _ => {}
                  }
                }
                keypress_timer += single_elapsed;
                if keypress_timer >= KEYPRESS_DURATION {
                  keypress_timer = 0.0;
                }
                continue;
              }
              match menu_selected_choice {
                Some(choice) => {
                  if choice != menu_choice {
                    battle_state.player_team.swap(choice, menu_choice);
                    menu_selected_choice = None;
                  }
                }
                None => {
                  menu_selected_choice = Some(menu_choice);
                }
              }
            } else {
              continue;
            };
            keypress_timer += single_elapsed;
            if keypress_timer >= KEYPRESS_DURATION {
              keypress_timer = 0.0;
            }
          }
          continue;
        }

        battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;
        if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
          if keypress_timer == 0.0 {
            current_choice -= 1;
            current_choice = if current_choice > 3 {
              0
            } else if current_choice < 0 {
              3
            } else {
              current_choice
            };

            battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;
            wincan.present();
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
          if keypress_timer == 0.0 {
            current_choice += 1;
            current_choice = if current_choice > 3 {
              0
            } else if current_choice < 0 {
              3
            } else {
              current_choice
            };
            battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;
          } else {
            continue;
          }
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::M)
          || keystate.contains(&Keycode::S)
          || keystate.contains(&Keycode::Down)
        {
          menu_active = true;
          continue;
        }
        if keystate.contains(&Keycode::Return) {
          if keypress_timer == 0.0 {
            battle_state.player_turn = true;
            // Battle Logic
            if battle_state.player_turn {
              match battle::player_battle_turn(
                wincan,
                &mut battle_state,
                &mut battle_draw,
                &monsters_map,
                current_choice as usize,
              )? {
                Map::Overworld => {
                  loaded_map = Map::Overworld;
                  if matches!(battle_state.battle_type, monster::BattleType::GymLeader) {
                    player_badges.insert(gym_no as u32);
                    battle_state.player_badges = player_badges.len();

                    // Spawn the player at their house
                    player_box.set_x(675);
                    player_box.set_y(390);
                  }
                  // Set time_count to now so the player isn't immediately sent into another battle
                  time_count = Instant::now();
                  continue;
                },
                Map::Gym => {
                  loaded_map = Map::Gym;
                  continue;
                }
                _ => {}
              }
              
              if !battle_state.player_turn {
                let enemy_choice = ai::ai_agent(difficulty_choice, &monsters_map, &mut battle_state);

                match battle::enemy_battle_turn(
                  wincan,
                  &mut battle_state,
                  &mut battle_draw,
                  &monsters_map,
                  enemy_choice,
                )? {
                  Map::Overworld => {
                    loaded_map = Map::Overworld;

                    // Have the player spawn at the hospital with full health
                    player_box.set_x(112);
                    player_box.set_y(604);

                    for item in battle_state.player_team.iter_mut() {
                      item.1 = 100.0;
                    }
                    battle_draw.player_health = 100.0;
                    continue;
                  }
                  _ => {}
                }
              }
            }
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
      },

      Map::GymOne => {
        gym_no = 0;
      },
      Map::GymTwo => {
        gym_no = 1;
      },
      Map::GymThree => {
        gym_no = 2;
        player_box.set_height(24);
        player_box.set_width(24);
      },
      Map::GymFour => {
        gym_no = 3;
      }, 
      _ => {

      }
    }
    if matches!(loaded_map, Map::Gym | Map::GymOne | Map::GymTwo | Map::GymThree | Map::GymFour) {
      // Determine the walls and draw them
      let mut collision = gym::draw_gym(wincan, gym_mazes[gym_no].clone(), gym_no);

      let mut wall_collision = false;
      // Prevent the player from going thru walls
      for member in collision.iter_mut() {
        if check_collision(&player_box, &member)
        {
          player_box.set_x(player_box.x() - x_vel);
          player_box.set_y(player_box.y() - y_vel);
          wall_collision = true;
        }
      }

      // Determine the placement of gym trainers and boss
      let gym_npcs = gym::draw_npc(wincan, &gym_mazes[gym_no], gym_no);
      let boss = gym_npcs.1;
      
      // Check if the player wants to exit the gym
      let exit_box = Rect::new(1240,0,100,50);
      if check_collision(&player_box, &exit_box)
      {
        gym::display_exit_gym_menu(wincan)?;
        if keystate.contains(&Keycode::E)
        {
          let coors = gym::gym_coordinates(gym_no);
          player_box.set_x(coors.0);
          player_box.set_y(coors.1);
          loaded_map = Map::Overworld;
          maze::reload_maze(&mut gym_mazes, gym_no);
        }
      }

      if keystate.contains(&Keycode::M) {
        menu_active = true;
        continue;
      }

      if menu_active {
        battle::draw_monster_menu(
          wincan,
          &battle_draw,
          &battle_state,
          menu_choice,
          menu_selected_choice,
        )?;
        if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) {
          if keypress_timer == 0.0 {
            menu_choice = match menu_choice {
              0 => 6,
              1 => 6,
              2 => 0,
              3 => 1,
              4 => 2,
              5 => 3,
              _ => 2 * (player_team.len() / 2 + player_team.len() % 2 - 1),
            };
          } else {
            continue;
          }; 
          // need to calculate how much time each loop takes regarding the machine it runs on 
          // so that we know how much to increment for the keypress timer
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
          if keypress_timer == 0.0 {
            menu_choice = match menu_choice {
              0 => {
                if player_team.len() > 1 {
                  1
                } else {
                  0
                }
              }
              1 => 0,
              2 => {
                if player_team.len() > 3 {
                  3
                } else {
                  0
                }
              }
              3 => 2,
              4 => {
                if player_team.len() > 5 {
                  5
                } else {
                  0
                }
              }
              5 => 4,
              _ => 6,
            };
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::S) || keystate.contains(&Keycode::Down) {
          if keypress_timer == 0.0 {
            menu_choice = match menu_choice {
              0 => {
                if player_team.len() > 2 {
                  2
                } else {
                  6
                }
              }
              1 => {
                if player_team.len() > 3 {
                  3
                } else {
                  6
                }
              }
              2 => {
                if player_team.len() > 4 {
                  4
                } else {
                  6
                }
              }
              3 => {
                if player_team.len() == 6 {
                  5
                } else {
                  6
                }
              }
              4 => 6,
              5 => 6,
              _ => 0,
            };
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
          if keypress_timer == 0.0 {
            menu_choice = match menu_choice {
              0 => {
                if player_team.len() > 1 {
                  1
                } else {
                  0
                }
              }
              1 => 0,
              2 => {
                if player_team.len() > 3 {
                  3
                } else {
                  0
                }
              }
              3 => 2,
              4 => {
                if player_team.len() > 5 {
                  5
                } else {
                  0
                }
              }
              5 => 4,
              _ => 6,
            };
          } else {
            continue;
          }
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        if keystate.contains(&Keycode::Return) {
          if keypress_timer == 0.0 {
            if menu_choice == 6 {
              menu_active = false;
              menu_selected_choice = None;
              battle_state.player_team = battle::verify_team(&battle_state.player_team);
              continue;
            }
            match menu_selected_choice {
              Some(choice) => {
                if choice != menu_choice {
                  battle_state.player_team.swap(choice, menu_choice);
                  menu_selected_choice = None;
                }
              }
              None => {
                menu_selected_choice = Some(menu_choice);
              }
            }
          } else {
            continue;
          };
          keypress_timer += single_elapsed;
          if keypress_timer >= KEYPRESS_DURATION {
            keypress_timer = 0.0;
          }
        }
        continue;
      }

      // Determine player movement
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
      

      // Determine to show option of battling NPC
      let mut show_battle_menu = false;
      for npc in gym_npcs.0 {
        if !wall_collision && check_collision(&player_box, &npc) {
          show_battle_menu = true;
          break;
        }
      }
      
      // When near NPC, show menu to enter battle
      if show_battle_menu {
        wincan.copy(player.texture(), None, player_box)?;
        overworld::display_menu(wincan, player_box.x(), player_box.y())?;

        // Set up a battle
        if keystate.contains(&Keycode::F) {
          let enemy_team = select_random_team(&all_monsters, 3, experience(difficulty_choice, battle_state.player_badges));
          battle_draw.opp_level = enemy_team[0].2 / 10;
          let enemy_team = battle::verify_team(&enemy_team);
          
          let enemy_monster = enemy_team[0].0.clone();
          battle_draw.enemy_name = enemy_monster.clone();
          let player_monster = next_available_mon(&battle_state.player_team);
          battle_draw.player_name = player_monster.clone();
          
          battle_state = monster::BattleState {
            player_turn: true,
            player_team: battle_state.player_team.clone(),
            enemy_team: enemy_team.clone(),
            self_attack_stages: 0,
            self_defense_stages: 0,
            opp_attack_stages: 0,
            opp_defense_stages: 0,
            player_badges: battle_state.player_badges,
            battle_type: &monster::BattleType::GymTrainer,
          };
          loaded_map = Map::Battle;
          battle_draw.enemy_health = 100.0;
          wincan.present();
          wincan.clear();
          battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;
          x_vel = 0;
          y_vel = 0;
          continue;
        }

        //flashing not active when moving away
        if keystate.contains(&Keycode::W)
        || keystate.contains(&Keycode::Up)
        || keystate.contains(&Keycode::A)
        || keystate.contains(&Keycode::Left)
        || keystate.contains(&Keycode::S)
        || keystate.contains(&Keycode::Down)
        || keystate.contains(&Keycode::D)
        || keystate.contains(&Keycode::Right)
        {
          wincan.present();
          x_vel = 0;
          y_vel = 0;
          continue;
        }
        //causes the flashing effect. Every time near npc, screen flashes
        wincan.present();
        wincan.present();
        x_vel = 0;
        y_vel = 0;
        continue;
      } else if check_collision(&player_box, &boss) {
        wincan.copy(player.texture(), None, player_box)?;
        overworld::display_menu(wincan, player_box.x(), player_box.y())?;

        // Set up gym leader battle
        if keystate.contains(&Keycode::F) {
          let enemy_team = select_random_team(&all_monsters, 4, experience(difficulty_choice, battle_state.player_badges));
          battle_draw.opp_level = enemy_team[0].2 / 10;
          let enemy_team = battle::verify_team(&enemy_team);
          
          let enemy_monster = enemy_team[0].0.clone();
          battle_draw.enemy_name = enemy_monster.clone();
          let player_monster = next_available_mon(&battle_state.player_team);
          battle_draw.player_name = player_monster.clone();
          
          battle_state = monster::BattleState {
            player_turn: true,
            player_team: battle_state.player_team.clone(),
            enemy_team: enemy_team.clone(),
            self_attack_stages: 0,
            self_defense_stages: 0,
            opp_attack_stages: 0,
            opp_defense_stages: 0,
            player_badges: battle_state.player_badges,
            battle_type: &monster::BattleType::GymLeader,
          };
          loaded_map = Map::Battle;
          battle_draw.enemy_health = 100.0;
          wincan.present();
          wincan.clear();
          battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;
          x_vel = 0;
          y_vel = 0;
          continue;
        }

        //flashing not active when moving away
        if keystate.contains(&Keycode::W)
        || keystate.contains(&Keycode::Up)
        || keystate.contains(&Keycode::A)
        || keystate.contains(&Keycode::Left)
        || keystate.contains(&Keycode::S)
        || keystate.contains(&Keycode::Down)
        || keystate.contains(&Keycode::D)
        || keystate.contains(&Keycode::Right)
        {
          wincan.present();
          x_vel = 0;
          y_vel = 0;
          continue;
        }
        //causes the flashing effect. Every time near npc, screen flashes
        wincan.present();
        wincan.present();
        x_vel = 0;
        y_vel = 0;
        continue;
      }

      // Display the player
      wincan.copy(player.texture(), None, player_box)?;
      wincan.present();
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
