extern crate sdl2;

// Modules
mod battle;
mod overworld;
mod player;
pub mod monster;

use monster::load_mons;
use monster::load_moves;
use player::Player;
use battle::Map;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use std::time::{Instant, Duration};

use std::collections::HashSet;
use std::path::Path;

use rand::{self, Rng};
use rand::thread_rng;

const TITLE: &str = "Monster Town Midterm";
const TILE_SIZE: u32 = 16;

// Camera
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const VSYNC: bool = true;

const MAX_SPEED: i32 = 5;
const ACCEL_RATE: i32 = 1;

const _SCALE_UP: i16 = 3;

const BUFFER_FRAMES: u32 = 10;
const DELTA_TIME: f64 = 1.0/60.0;

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

fn select_random_monster<'a>(keys: &Vec<String>) -> String {
  let a = &keys[rand::thread_rng().gen_range(0..keys.len())];
  return a.clone();
}


fn check_within(small: &Rect, large: &Rect) -> bool {
  if small.left() > large.left() && small.right() < large.right() && small.top() > large.top() 
    && small.top() > large.top() && small.bottom() < large.bottom() {
      true
    } else {
      false
    }
}

fn random_spawn() -> bool{
  let mut rng = thread_rng();
  let ran = rng.gen_range(0..300);
  if ran == 2 {
    true
  } else {
    false
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
  
  wincan.set_blend_mode(BlendMode::Blend);

  let mut loaded_map = Map::Overworld;

  let player_monster = String::from("deer pokemon");
  let mut enemy_monster = String::from("melon-mon");

  let moves_map = load_moves();
  let monsters_map = load_mons(&moves_map);

  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
  let font_path = Path::new(r"./fonts/framd.ttf");
  let font = ttf_context.load_font(font_path, 256)?;

  let all_moves = moves_map.keys().map(|d| String::from(d)).collect::<Vec<String>>();
  let all_effects = moves_map.values().map(|d| String::from(d.effect.clone())).collect::<Vec<String>>();
  let all_monsters = monsters_map.keys().map(|d| String::from(d)).collect::<Vec<String>>();

  let move_textures = battle::create_all_attack_textures(&texture_creator, &font, &all_moves)?;
  let effect_textures = battle::create_all_effect_textures(&texture_creator, &font, &all_effects)?;
  let names_tup = battle::create_all_name_tuples(&texture_creator, &font, &all_monsters)?;
  let monster_textures = battle::create_all_monster_textures(&texture_creator, &all_monsters)?;

  let mut battle_draw = battle::Battle {
    background_texture: &battle_bg,
    player_name: player_monster.clone(),
    enemy_name: enemy_monster.clone(),
    font: &font,
    player_health: 100.0,
    enemy_health: 100.0,
    name_text_map: &names_tup,
    attack_map: &move_textures,
    effect_map: &effect_textures,
    monster_text_map: &monster_textures,
    monsters: &monsters_map,
    moves: &moves_map,
  };

  let mut battle_state = monster::BattleState {
    player_turn: monsters_map[&player_monster].attack_stat >= monsters_map[&enemy_monster].attack_stat,
    player_monster: &monsters_map[&player_monster],
    opp_monster: &monsters_map[&enemy_monster],
    self_attack_stages: 0,
    self_defense_stages: 0,
    opp_attack_stages: 0,
    opp_defense_stages: 0,
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

   // Tracking time
   let mut time_count = Instant::now();

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

    //println!("time_count: {:.2}", time_count.elapsed().as_secs_f64());
    let elapsed = time_count.elapsed().as_secs_f64();

    match loaded_map {
      Map::Overworld => {
        wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
        
        overworld::draw_overworld(wincan)?;
        let spawnable_areas = overworld::mark_rectangles();
        //let test = &spawnable_areas[0].x();
        // iterate over the spawnable rectangles 
        /*for i in &spawnable_areas{
          let test_result = check_within(&Rect::new(100,120,1,1),i);
          println!("{:?}",test_result);
          if test_result == true && random_spawn() {
            break;
          }
        }*/

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

        for i in &spawnable_areas{
          let test_result = check_within(&player_box,i);
          //println!("{:?}",test_result);
          //let elapsed = time_count.elapsed();
          if test_result == true && random_spawn() 
            && (((elapsed  * 100.0).round()) % ((DELTA_TIME* 100.0).round()) == 0.0 ){
            println!("check time step: {:.2}", elapsed);
            let screen = Rect::new(0, 0, CAM_W, CAM_H);
            wincan.copy(player.texture(), None, player_box)?;

            wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
            for _i in 0..50 {
              wincan.fill_rect(screen)?;
              wincan.present();
            }
            loaded_map = Map::Battle;

            battle_draw.enemy_health = 100.0;

            enemy_monster = select_random_monster(&all_monsters);

            battle_draw.enemy_name = enemy_monster.clone();

            battle_state = monster::BattleState {
              player_turn: monsters_map[&player_monster].attack_stat >= monsters_map[&enemy_monster].attack_stat,
              player_monster: &monsters_map[&player_monster],
              opp_monster: &monsters_map[&enemy_monster],
              self_attack_stages: 0,
              self_defense_stages: 0,
              opp_attack_stages: 0,
              opp_defense_stages: 0,
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

          overworld::display_menu(wincan,player_box.x(),player_box.y())?;
          if keystate.contains(&Keycode::F) {

            enemy_monster = select_random_monster(&all_monsters);
            battle_draw.enemy_name = enemy_monster.clone();

            battle_state = monster::BattleState {
              player_turn: monsters_map[&player_monster].attack_stat >= monsters_map[&enemy_monster].attack_stat,
              player_monster: &monsters_map[&player_monster],
              opp_monster: &monsters_map[&enemy_monster],
              self_attack_stages: 0,
              self_defense_stages: 0,
              opp_attack_stages: 0,
              opp_defense_stages: 0,
            };

            loaded_map = Map::Battle;
            battle_draw.enemy_health = 100.0;
          }
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
      }

      Map::Battle => {
        battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;

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

            battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;
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
            battle::draw_battle(wincan, &battle_draw, Some(current_choice as usize), None)?;
          }
        }
        if keystate.contains(&Keycode::Return) {
          // Battle Logic
          if battle_state.player_turn {
            match battle::player_battle_turn(wincan, &mut battle_state, &mut battle_draw, &monsters_map, current_choice as usize)? {
              Map::Overworld => { 
                loaded_map = Map::Overworld;
                continue;
              }
              _ => {}
            }
            // Change to AI's turn
            battle_state.player_turn = !battle_state.player_turn;
            
            match battle::enemy_battle_turn(wincan, &mut battle_state, &mut battle_draw, &monsters_map)? {
              Map::Overworld => { 
                loaded_map = Map::Overworld;

                // Have the player spawn at the hospital with full health
                player_box.set_x(112);
                player_box.set_y(604);
                battle_draw.player_health = 100.0;
                continue;
              }
              _ => {}
            }
            
            // Change to player's turn
            battle_state.player_turn = !battle_state.player_turn;
          } else {
            match battle::enemy_battle_turn(wincan, &mut battle_state, &mut battle_draw, &monsters_map)? {
              Map::Overworld => { 
                loaded_map = Map::Overworld;
                // Have the player spawn at the hospital with full health
                player_box.set_x(112);
                player_box.set_y(604);
                battle_draw.player_health = 100.0;
                continue;
              }
              _ => {}
            }

            // Change to player's turn
            battle_state.player_turn = !battle_state.player_turn;
            match battle::player_battle_turn(wincan, &mut battle_state, &mut battle_draw, &monsters_map, current_choice as usize)? {
              Map::Overworld => { 
                loaded_map = Map::Overworld;
                continue;
              }
              _ => {}
            }
            // Change to AI's turn
            battle_state.player_turn = !battle_state.player_turn;

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
