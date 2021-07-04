//use std::time::Duration;
//use std::thread;
//use std::collections::HashSet;

//use std::path::Path;

use sdl2::pixels::Color;
//use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;

use std::time::Duration;
use std::thread;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use sdl2::rect::Rect;

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

pub fn create_attack_tuples<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    attack_names: &'a Vec<String>,
    attack_effects: &'a Vec<String>,
) -> Result<(Vec<(sdl2::render::Texture<'a>, Rect)>, Vec<(sdl2::render::Texture<'a>, Rect)>), String> {
    let rs: Vec<_> = (0..4)
        .map(|i| 180 + i * (200 + 40))
        .map(|i| Rect::new(i, 560 as i32, 200, 100))
        .collect();

    let mut attacks_tup = Vec::new();
    let mut effects_tup = Vec::new();

    for (index, r) in rs.into_iter().enumerate() {
        // Create a texture for the name of each attack
        let surface = font
            .render(&attack_names[index])
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // Figure out how to resize the text to fit within the provided space
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(r.x() + 10, r.y() + 5, 180, 50);
        let text_rect = center(fit(text_rect, width, height), 180, 50);

        // Return the tuple for later use
        attacks_tup.push( (texture, text_rect) );

        // Create a texture for the effect of each attack
        let surface = font
            .render(&attack_effects[index])
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // Figure out how to resize the text to fit within the provided space
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(r.x() + 10, r.y() + 65, 180, 30);
        let text_rect = center(fit(text_rect, width, height), 180, 30);

        // Return the tuple for later use
        effects_tup.push( (texture, text_rect) );
    }
    Ok((attacks_tup, effects_tup))
}

pub fn create_name_tuples<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    player_monster: &'a String,
    enemy_monster: &'a String,
) -> Result<((sdl2::render::Texture<'a>, Rect), (sdl2::render::Texture<'a>, Rect)), String> {
    let surface = font
        .render(&player_monster)
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;
    let player_texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = player_texture.query();

    let text_rect = Rect::new(500, 379, 300, 35);
    let text_rect = fit(text_rect, width, height);
    let player_rect = Rect::new(500 + (290 - text_rect.width() as i32) as i32, text_rect.y(), text_rect.width(), text_rect.height());
    
    let surface = font
        .render(&enemy_monster)
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;
    let enemy_texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = enemy_texture.query();

    let text_rect = Rect::new(490, 88, 300, 35);
    let enemy_rect = fit(text_rect, width, height);

    Ok( ( (player_texture, player_rect), (enemy_texture, enemy_rect) ) )
}

pub struct Battle<'a> {
    pub player_name: &'a (sdl2::render::Texture<'a>, Rect),
    pub enemy_name: &'a (sdl2::render::Texture<'a>, Rect),
    pub background_texture: &'a sdl2::render::Texture<'a>,
    pub player_texture: &'a sdl2::render::Texture<'a>,
    pub enemy_texture: &'a sdl2::render::Texture<'a>,
    pub font: &'a sdl2::ttf::Font<'a, 'a>,
    pub player_attacks: &'a Vec<(sdl2::render::Texture<'a>, Rect)>,
    pub player_attack_effects: &'a Vec<(sdl2::render::Texture<'a>, Rect)>,
    pub player_health: f32,
    pub enemy_health: f32,
}

impl<'a> Battle<'a> {
    pub fn set_player_health(&mut self, new_health: f32) {
        self.player_health = new_health;
    }
    pub fn set_enemy_health(&mut self, new_health: f32) {
        self.enemy_health = new_health;
    }
}

pub fn better_draw_battle(wincan: &mut sdl2::render::WindowCanvas, battle_init: &Battle, choice: usize, message: Option<String>) -> Result<(), String> {
    // Load the battle scene background
    wincan.copy(&battle_init.background_texture, None, Rect::new(0,0,CAM_W,CAM_H))?;
    
    let move_rects: Vec<_> = (0..4)
        .map(|i| 180 + i * (200 + 40))
        .map(|i| Rect::new(i, 560 as i32, 200, 100))
        .collect();

    // Create an outline around the move that is currently selected
    let outline_size = 5;
    let r = move_rects[choice];
    let move_outline_rect = Rect::new(r.x() - outline_size, r.y() - outline_size, (r.width() + (2*outline_size)as u32) as u32, (r.height() + (2*outline_size)as u32) as u32);

    wincan.set_draw_color(Color::RGB(0xf6, 0x52, 0x41));
    wincan.fill_rect(move_outline_rect)?;

    // For all moves
    for (index, item) in move_rects.into_iter().enumerate()  {
        // Create the background for each move
        let r = item;
        wincan.set_draw_color(Color::RGB(0x20, 0x41, 0x6a));
        wincan.fill_rect(r)?;

        // Add the names of each attack
        wincan.copy(&battle_init.player_attacks[index].0, None, battle_init.player_attacks[index].1)?;

        // Add the names of each attack
        wincan.copy(&battle_init.player_attack_effects[index].0, None, battle_init.player_attack_effects[index].1)?;
    }

    // Add the names of both monsters
    wincan.copy(&battle_init.player_name.0, None, battle_init.player_name.1)?;
    wincan.copy(&battle_init.enemy_name.0, None, battle_init.enemy_name.1)?;

    // Add both monsters
    wincan.copy(&battle_init.player_texture, None, Rect::new(800,275,200,200))?;
    wincan.copy_ex(&battle_init.enemy_texture, None, Rect::new(280 as i32,25 as i32,200,200), 0 as f64, None, true, false)?;

    // Calculate and add health bars for each monster
    health_bars(wincan, battle_init.player_health, battle_init.enemy_health)?;

    // FOR DEMO ONLY
    let s = vec!["Demo Instructions:","Use AD/←→ to choose a move","Use Enter to submit your choice", "Use K to kill the other monster", "Use E to exit the battle"];
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
    //let enemy_health: f32 = 12 as f32;

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

    // let r2 = Rect::new(333, 429, 435, 18);

    // let player_health: f32 = 51 as f32;

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

    // wincan.present();

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