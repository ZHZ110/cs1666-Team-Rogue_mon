extern crate rand;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;


const TILE_SIZE: u32 = 16;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;


// it also serve as tagging the blocks as spawnable
pub fn draw_overworld(wincan: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
  let texture_creator = wincan.texture_creator();

  // Texture
  let tree_sheet = texture_creator.load_texture("images/tree.png")?;
  let grass_sheet = texture_creator.load_texture("images/grass_patch_32.png")?;
  let water_sheet = texture_creator.load_texture("images/water_patch_32.png")?;
  let rock_sheet = texture_creator.load_texture("images/rock_patch.png")?;
  wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
  wincan.clear();

  // Draw bottom trees
  let mut i = 0;
  while i * TILE_SIZE < CAM_W {
    let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);
    let pos = Rect::new(
      (i * TILE_SIZE) as i32,
      (CAM_H - 4 * TILE_SIZE) as i32,
      TILE_SIZE,
      4 * TILE_SIZE,
    );

    wincan.copy(&tree_sheet, src, pos)?;

    i += 1;
  }

  // Draw upper trees
  let mut i = 0;
  while i * TILE_SIZE < CAM_W {
    let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);
    let pos = Rect::new((i * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);

    wincan.copy(&tree_sheet, src, pos)?;

    i += 1;
  }

  // Draw grass patches to the top left corner of map
  let mut i = 6;
  while i * TILE_SIZE < 320 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 96, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 128, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 160, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 192, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&grass_sheet, src, pos_1)?;
    wincan.copy(&grass_sheet, src, pos_2)?;
    wincan.copy(&grass_sheet, src, pos_3)?;
    wincan.copy(&grass_sheet, src, pos_4)?;

    i += 1;
  }

  // Draw a pond to the right bottom corner of map
  let mut i = 48;
  while i * TILE_SIZE < 1060 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 480, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 512, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 544, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 576, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&water_sheet, src, pos_1)?;
    wincan.copy(&water_sheet, src, pos_2)?;
    wincan.copy(&water_sheet, src, pos_3)?;
    wincan.copy(&water_sheet, src, pos_4)?;

    i += 1;
  }

  // Draw rock patch to right upper corner of map
  let mut i = 60;
  while i * TILE_SIZE < 1240 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 66, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 98, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 130, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 162, TILE_SIZE, 2 * TILE_SIZE);
    let pos_5 = Rect::new((i * TILE_SIZE) as i32, 194, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&rock_sheet, src, pos_1)?;
    wincan.copy(&rock_sheet, src, pos_2)?;
    wincan.copy(&rock_sheet, src, pos_3)?;
    wincan.copy(&rock_sheet, src, pos_4)?;
    wincan.copy(&rock_sheet, src, pos_5)?;
    i += 1;
  }

  // Draw large grass patches to north of map
  let mut i = 32;
  while i * TILE_SIZE < 820 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 96, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 128, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 160, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 192, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&grass_sheet, src, pos_1)?;
    wincan.copy(&grass_sheet, src, pos_2)?;
    wincan.copy(&grass_sheet, src, pos_3)?;
    wincan.copy(&grass_sheet, src, pos_4)?;

    i += 1;
  }

  // Draw grass patches to the south of map
  let mut i = 32;
  while i * TILE_SIZE < 730 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 480, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 512, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 544, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 576, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&grass_sheet, src, pos_1)?;
    wincan.copy(&grass_sheet, src, pos_2)?;
    wincan.copy(&grass_sheet, src, pos_3)?;
    wincan.copy(&grass_sheet, src, pos_4)?;

    i += 1;
  }

  // Draw grass patches to the center right of map
  let mut i = 62;
  while i * TILE_SIZE < 1240 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 300, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 332, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 364, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 396, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&grass_sheet, src, pos_1)?;
    wincan.copy(&grass_sheet, src, pos_2)?;
    wincan.copy(&grass_sheet, src, pos_3)?;
    wincan.copy(&grass_sheet, src, pos_4)?;

    i += 1;
  }

  // Draw pond to the middle center left of map
  let mut i = 4;
  while i * TILE_SIZE < 300 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 280, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 312, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 344, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 376, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&water_sheet, src, pos_1)?;
    wincan.copy(&water_sheet, src, pos_2)?;
    wincan.copy(&water_sheet, src, pos_3)?;
    wincan.copy(&water_sheet, src, pos_4)?;

    i += 1;
  }

  // Draw small rock patch in middle of map
  let mut i = 24;
  while i * TILE_SIZE < 570 {
    let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
    let pos_1 = Rect::new((i * TILE_SIZE) as i32, 280, TILE_SIZE, 2 * TILE_SIZE);
    let pos_2 = Rect::new((i * TILE_SIZE) as i32, 312, TILE_SIZE, 2 * TILE_SIZE);
    let pos_3 = Rect::new((i * TILE_SIZE) as i32, 344, TILE_SIZE, 2 * TILE_SIZE);
    let pos_4 = Rect::new((i * TILE_SIZE) as i32, 376, TILE_SIZE, 2 * TILE_SIZE);

    wincan.copy(&grass_sheet, src, pos_1)?;
    wincan.copy(&grass_sheet, src, pos_2)?;
    wincan.copy(&grass_sheet, src, pos_3)?;
    wincan.copy(&grass_sheet, src, pos_4)?;

    i += 1;
  }

  Ok(())
}

pub fn display_menu(wincan: &mut sdl2::render::WindowCanvas, player_x: i32, player_y: i32) -> Result<(), String>{
  let texture_creator = wincan.texture_creator();
  let fight_tab = texture_creator.load_texture("images/pressF.png")?;
  let bail_tab = texture_creator.load_texture("images/bail.png")?;

  // Add the fight tab
  let src_f = Rect::new(0, 0, 128, 64);
  let mut pos_f = Rect::new(player_x - 20, player_y - 140, 128, 64);

  
  // Add the bail tab
  let src_b = Rect::new(0, 0, 128, 64);
  let mut pos_b = Rect::new(player_x - 20, player_y - 140 + 64, 128, 64);
  
  if pos_f.y < 0 || pos_b.y < 0{
    pos_f = Rect::new(player_x - 20, player_y + 50, 128, 64);
    pos_b = Rect::new(player_x - 20, player_y + 50 + 64, 128, 64);
  }
  wincan.copy(&fight_tab, src_f, pos_f)?;
  wincan.copy(&bail_tab, src_b, pos_b)?;
  Ok(())
}


  pub fn display_building_menu(wincan: &mut WindowCanvas) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();
    let display_gym_box = texture_creator.load_texture("images/enterbuilding.png").unwrap();
  
    let display_box = Rect::new(500, 200, 200 ,200);
    wincan.copy(&display_gym_box, None, display_box)?;

    Ok(())
  }

  pub fn draw_hospital(wincan: &mut WindowCanvas)->Result<(), String>{
    let hospital_screen = Rect::new((0) as i32, (0) as i32, (1280) as u32, (720) as u32);
    let _texture_creator = wincan.texture_creator();
    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.fill_rect(hospital_screen).unwrap();

     Ok(())
  }

  pub fn draw_home(wincan: &mut WindowCanvas)->Result<(), String>{
    let home_screen = Rect::new((0) as i32, (0) as i32, (1280) as u32, (720) as u32);
    let _texture_creator = wincan.texture_creator();
    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.fill_rect(home_screen).unwrap();

    Ok(())
  }

pub fn mark_rectangles() -> Vec<Rect>{
  let mut spn_rectangles = Vec::new();
  // Top left corner of the grass patches
  let left_corner_grass = Rect::new((6*TILE_SIZE) as i32, 96, 2*TILE_SIZE*7, 2*TILE_SIZE*4);
  spn_rectangles.push(left_corner_grass);
  // A pond to the right bottom corner of map
  let right_bottom_pond = Rect::new((48*TILE_SIZE) as i32, 480, 2*TILE_SIZE*10, 2*TILE_SIZE*4);
  spn_rectangles.push(right_bottom_pond);
  // Rock patches to right upper corner of map
  let right_upper_rock = Rect::new((60*TILE_SIZE) as i32, 66, 2*TILE_SIZE*9, 2*TILE_SIZE*5);
  spn_rectangles.push(right_upper_rock);
  // Large grass patches to north of map
  let north_grass = Rect::new((32*TILE_SIZE) as i32, 96, 2*TILE_SIZE*10, 2*TILE_SIZE*4);
  spn_rectangles.push(north_grass);
  // Grass patches to the south of map
  let south_grass = Rect::new((32*TILE_SIZE) as i32, 480, 2*TILE_SIZE*7, 2*TILE_SIZE*4);
  spn_rectangles.push(south_grass);
  // Grass patches to the center right of map
  let center_right_grass = Rect::new((62*TILE_SIZE) as i32, 300, 2*TILE_SIZE*8, 2*TILE_SIZE*4);
  spn_rectangles.push(center_right_grass);
  // A pond to the middle center left of map
  let center_left_pond = Rect::new((4*TILE_SIZE) as i32, 280, 2*TILE_SIZE*8, 2*TILE_SIZE*4);
  spn_rectangles.push(center_left_pond);
  // Small grass patches in the center of the map
  let center_grass = Rect::new((24*TILE_SIZE) as i32, 280, 2*TILE_SIZE*6, 2*TILE_SIZE*4);
  spn_rectangles.push(center_grass);

  return spn_rectangles;
}
