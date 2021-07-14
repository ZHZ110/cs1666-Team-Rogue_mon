use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use rand::{self, Rng};

use crate::monster;

pub enum Map {
	Overworld,
	Battle,
}

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const MESSAGE_TIME: u64 = 500;

fn center(r1: Rect, w: u32, h: u32) -> Rect {
    let mut x = r1.x();
    let mut y = r1.y();
    if r1.width() < w {
        x += ((w - r1.width()) / 2) as i32
    }
    if r1.height() < h {
        y += ((h - r1.height()) / 2) as i32
    }
    Rect::new(x, y, r1.width(), r1.height())

}

fn fit(r1: Rect, w: u32, h: u32) -> Rect {
    let c = if w as f32/r1.width() as f32 > h as f32/r1.height() as f32 { // wider
        r1.width() as f32 / w as f32
    } else {
        r1.height() as f32 / h as f32
    };
    Rect::new(r1.x(), r1.y(), (w as f32 *c) as u32, (h as f32 *c) as u32)
}

pub fn create_all_attack_textures<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    attack_names: &'a Vec<String>,
) -> Result<HashMap<String, sdl2::render::Texture<'a>>, String> {

    let mut attacks_map = HashMap::new();

    for item in attack_names.into_iter() {
        let surface = font
            .render(item)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        attacks_map.insert(item.clone(), texture);
    }

    Ok( attacks_map )
}



pub fn create_all_effect_textures<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    attack_effects: &'a Vec<String>,
) -> Result<HashMap<String, sdl2::render::Texture<'a>>, String> {

    let mut attacks_map = HashMap::new();

    for item in attack_effects.into_iter() {
        let surface = font
            .render(item)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        attacks_map.insert(item.clone(), texture);
    }

    Ok( attacks_map )
}

pub fn create_all_name_tuples<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    monster_names: &'a Vec<String>,
) -> Result<HashMap<String, (sdl2::render::Texture<'a>, Rect, Rect)>, String> {
    
    let mut monster_name_map = HashMap::new();

    for item in monster_names.into_iter() {
        let surface = font
            .render(item)
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let player_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = player_texture.query();

        let text_rect = Rect::new(500, 379, 300, 35);
        let text_rect = fit(text_rect, width, height);
        let player_rect = Rect::new(500 + (290 - text_rect.width() as i32) as i32, text_rect.y(), text_rect.width(), text_rect.height());
    
        let text_rect = Rect::new(490, 88, 300, 35);
        let enemy_rect = fit(text_rect, width, height);

        monster_name_map.insert(item.clone(), (player_texture, player_rect, enemy_rect));
    }
    Ok( monster_name_map )
}

pub fn create_all_monster_textures<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    monster_names: &'a Vec<String>,
) -> Result<HashMap<String, sdl2::render::Texture<'a>>, String> {
    let mut monster_text_map = HashMap::new();
    
    for item in monster_names.into_iter() {
        let im_path = format!("images/{}.png", item);
        let temp_text = texture_creator.load_texture(im_path)?;
        monster_text_map.insert(item.clone(), temp_text);
    }

    Ok( monster_text_map )
}

pub struct Battle<'a> {
    pub background_texture: &'a sdl2::render::Texture<'a>,
    pub player_name: String,
    pub enemy_name: String,
    pub font: &'a sdl2::ttf::Font<'a, 'a>,
    pub player_health: f32,
    pub enemy_health: f32,
    pub name_text_map: &'a HashMap<String, (sdl2::render::Texture<'a>, Rect, Rect)>,
    pub attack_map: &'a HashMap<String, sdl2::render::Texture<'a>>,
    pub effect_map: &'a HashMap<String, sdl2::render::Texture<'a>>,
    pub monster_text_map: &'a HashMap<String, sdl2::render::Texture<'a>>,
    pub moves: &'a HashMap<String, monster::Move>,
    pub monsters: &'a HashMap<String, monster::Monster<'a>>,
}

impl<'a> Battle<'a> {
    pub fn apply_player_damage(&mut self, damage: f32) {
        self.player_health -= damage;
        self.player_health = self.player_health.clamp(0.0, 100.0);
    }
    pub fn apply_enemy_damage(&mut self, damage: f32) {
        self.enemy_health -= damage;
        self.enemy_health = self.enemy_health.clamp(0.0, 100.0);
    }
}

pub fn draw_battle(wincan: &mut sdl2::render::WindowCanvas, battle_init: &Battle, choice: Option<usize>, message: Option<String>) -> Result<(), String> {


    // Load the battle scene background
    wincan.copy(&battle_init.background_texture, None, Rect::new(0,0,CAM_W,CAM_H))?;

    let move_rects: Vec<_> = (0..4)
        .map(|i| 180 + i * (200 + 40))
        .map(|i| Rect::new(i, 560 as i32, 200, 100))
        .collect();

    // Create an outline around the move that is currently selected
    let outline_size = 5;

    match choice {
        Some(option) => {
            let r = move_rects[option];
            let move_outline_rect = Rect::new(r.x() - outline_size, r.y() - outline_size, (r.width() + (2*outline_size)as u32) as u32, (r.height() + (2*outline_size)as u32) as u32);

            wincan.set_draw_color(Color::RGB(0xf6, 0x52, 0x41));
            wincan.fill_rect(move_outline_rect)?;
        }
        None => {}
    };

    // For all moves
    for (index, item) in move_rects.into_iter().enumerate()  {
        // Create the background for each move
        let r = item;
        wincan.set_draw_color(Color::RGB(0x20, 0x41, 0x6a));
        wincan.fill_rect(r)?;

        let attack_name = &battle_init.monsters[&battle_init.player_name].moves[index].name;
        let texture = &battle_init.attack_map[attack_name];
        
        // Add the names of each attack
        // Figure out how to resize the text to fit within the provided space
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(r.x() + 10, r.y() + 5, 180, 50);
        let text_rect = center(fit(text_rect, width, height), 180, 50);
        
        wincan.copy(&texture, None, text_rect)?;
        
        let effect_name = &battle_init.monsters[&battle_init.player_name].moves[index].effect;
        let texture = &battle_init.effect_map[effect_name];
        
        // Add the names of each effect
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(r.x() + 10, r.y() + 65, 180, 30);
        let text_rect = center(fit(text_rect, width, height), 180, 30);


        let stat_name = &battle_init.monsters[&battle_init.player_name].moves[index].name;
        let texture = &battle_init.attack_map[attack_name];
        

        //stats of each monster
        let TextureQuery { width, height, .. } = stats.query();
        let text_rect = Rect::new(r.x() + 10, r.y() + 200, 180, 30);


        
        wincan.copy(&texture, None, text_rect)?;
    }

    // Add the names of both monsters
    wincan.copy(&battle_init.name_text_map[&battle_init.player_name].0, None, battle_init.name_text_map[&battle_init.player_name].1)?;
    wincan.copy(&battle_init.name_text_map[&battle_init.enemy_name].0, None, battle_init.name_text_map[&battle_init.enemy_name].2)?;

    // Add both monsters
    wincan.copy(&battle_init.monster_text_map[&battle_init.player_name], None, Rect::new(800,275,200,200))?;
    wincan.copy_ex(&battle_init.monster_text_map[&battle_init.enemy_name], None, Rect::new(280 as i32,25 as i32,200,200), 0 as f64, None, true, false)?;

    // Calculate and add health bars for each monster
    health_bars(wincan, battle_init.player_health, battle_init.enemy_health)?;

    // FOR DEMO ONLY
    let s = vec!["Demo Instructions:","Use AD/←→ to choose a move","Use Enter to submit your choice"];
    let texture_creator = wincan.texture_creator();

    for (index, item) in s.iter().enumerate() {
        let surface = battle_init.font
            .render(&item)
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let text_rect = Rect::new(25, 300 + (20*index) as i32, width, 20);
        let text_rect = fit(text_rect, width, height);
        wincan.copy(&texture, None, text_rect)?;
    }

    // Print out a message if needed
    match message {
        Some(text) => {
            message_box(wincan, battle_init.font, &text)?;
            thread::sleep(Duration::from_millis(MESSAGE_TIME));
        },
        None => (),
    };

    wincan.present();

    Ok(())
}

pub fn health_bars(wincan: &mut sdl2::render::WindowCanvas, player_health: f32, enemy_health: f32) -> Result<(), String> {
    
    if enemy_health > 50 as f32{
        wincan.set_draw_color(Color::GREEN);
    } else if enemy_health > 20 as f32{
        wincan.set_draw_color(Color::YELLOW);
    } else if enemy_health == 0 as f32 {
        wincan.set_draw_color(Color::RGBA(0,0,0,0));
    } else {
        wincan.set_draw_color(Color::RED);
    }

    let r2 = Rect::new(508, 54, ((enemy_health*435.0/100.0) as f32).ceil() as u32, 18);
    wincan.fill_rect(r2)?;

    if player_health > 50 as f32{
        wincan.set_draw_color(Color::GREEN);
    } else if player_health > 20 as f32{
        wincan.set_draw_color(Color::YELLOW);
    } else if player_health == 0 as f32 {
        wincan.set_draw_color(Color::RGBA(0,0,0,0));
    } else {
        wincan.set_draw_color(Color::RED);
    }

    let r2 = Rect::new(333, 429, ((player_health*435.0/100.0) as f32).ceil() as u32, 18);
    wincan.fill_rect(r2)?;

    Ok(())
}

fn message_box<'a>(
    wincan: &mut sdl2::render::WindowCanvas, 
    font: &'a sdl2::ttf::Font,
    message: &str,
) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();

    let r2 = Rect::new(600, 150, 400, 125);
    wincan.set_draw_color(Color::WHITE);
    wincan.fill_rect(r2)?;

    let r2 = Rect::new(605, 155, 390, 115);
    wincan.set_draw_color(Color::BLACK);
    wincan.fill_rect(r2)?;

    let r2 = Rect::new(610, 160, 380, 105);
    wincan.set_draw_color(Color::WHITE);
    wincan.fill_rect(r2)?;

    let mut line_message = format!("");

    let mut line_number = 0;

    for c in message.chars() {
        line_message = format!("{}{}", line_message, c);
        
        let mut surface = font
            .render(&line_message)
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let mut texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        if width as f32/height as f32 > 375 as f32/30 as f32 {
            line_number += 1;
            line_message = format!("{}", c);
            surface = font
                .render(&line_message)
                .blended(Color::BLACK)
                .map_err(|e| e.to_string())?;
            texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
        }

        let TextureQuery { width, height, .. } = texture.query();
        
        let text_rect = Rect::new(612, 162 + (line_number * 35), 375, 30);
        let text_rect = fit(text_rect, width, height);
        
        wincan.copy(&texture, None, text_rect)?;
        wincan.present();
    }
    Ok(())
}

pub fn player_battle_turn(
    wincan: &mut sdl2::render::WindowCanvas, 
    battle_state: &mut monster::BattleState,
    battle_draw: &mut Battle,
    monsters_map: &HashMap<String, monster::Monster>,
    current_choice: usize,
) -> Result<Map, String> {

    let enemy_monster = battle_draw.enemy_name.clone();
    let player_monster = battle_draw.player_name.clone();

    // Message for what move was used
    thread::sleep(Duration::from_millis(100));

    let f = format!("{} used {}!", &player_monster, monsters_map[&player_monster].moves[current_choice].name);
    draw_battle(wincan, &battle_draw, None, Some(f))?;

    // Apply the damage internally and to the drawing
    let d = monster::calculate_damage(battle_state, current_choice);
    battle_draw.apply_enemy_damage(d);

    // Check effectiveness, and message based upon it
    let effectiveness = monster::str_effectiveness(
      d,
      &monsters_map[&player_monster].moves[current_choice].attack_type,
      &monsters_map[&enemy_monster].monster_type, 
    );
    match effectiveness {
      Some(s) => {
        thread::sleep(Duration::from_millis(300));
        draw_battle(wincan, &battle_draw, None, Some(s))?;
      },
      None => {
        draw_battle(wincan, &battle_draw, None, None)?;
      }
    }

    thread::sleep(Duration::from_millis(300));
    
    if battle_draw.enemy_health == 0.0 {
      // Write message that enemy is KO'd
      let f = format!("{} KO'd {}!", &player_monster, &enemy_monster);
      draw_battle(wincan, &battle_draw, None, Some(f))?;

      // Fade out back to the overworld
      let screen = Rect::new(0, 0, CAM_W, CAM_H);
      wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
      for _i in 0..50 {
        wincan.fill_rect(screen)?;
        wincan.present();
      }
      return Ok( Map::Overworld );
    }
    Ok( Map::Battle )
}

pub fn enemy_battle_turn(
    wincan: &mut sdl2::render::WindowCanvas, 
    battle_state: &mut monster::BattleState,
    battle_draw: &mut Battle,
    monsters_map: &HashMap<String, monster::Monster>,
) -> Result<Map, String> {

    let enemy_monster = battle_draw.enemy_name.clone();
    let player_monster = battle_draw.player_name.clone();

    let enemy_choice = rand::thread_rng().gen_range(0..4) as usize;

    // Message for what move was used

    thread::sleep(Duration::from_millis(300));


    let f = format!("{} used {}!", &enemy_monster, monsters_map[&enemy_monster].moves[enemy_choice].name);
    draw_battle(wincan, &battle_draw, None, Some(f))?;

    // Apply the damage internally and to the drawing
    let d = monster::calculate_damage(battle_state, enemy_choice);
    battle_draw.apply_player_damage(d);

    // Check effectiveness, and message based upon it
    let effectiveness = monster::str_effectiveness(
      d,
      &monsters_map[&enemy_monster].moves[enemy_choice].attack_type,
      &monsters_map[&player_monster].monster_type, 
    );
    match effectiveness {
      Some(s) => {
        thread::sleep(Duration::from_millis(300));
        draw_battle(wincan, &battle_draw, None, Some(s))?;
      },
      None => {
        draw_battle(wincan, &battle_draw, None, None)?;
      }
    }

    thread::sleep(Duration::from_millis(300));
    
    if battle_draw.player_health == 0.0 {
      // Write message that player is KO'd
          thread::sleep(Duration::from_millis(200));
      let f = format!("{} KO'd {}!", &enemy_monster, &player_monster);
      draw_battle(wincan, &battle_draw, None, Some(f))?;


      thread::sleep(Duration::from_millis(200));
      let f = format!("You blacked out!");
      draw_battle(wincan, &battle_draw, None, Some(f))?;
     

      // Fade out back to the overworld
      let screen = Rect::new(0, 0, CAM_W, CAM_H);
      wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
      for _i in 0..50 {
        wincan.fill_rect(screen)?;
        wincan.present();
      }
      return Ok( Map::Overworld );
    }
    Ok( Map::Battle )
}
