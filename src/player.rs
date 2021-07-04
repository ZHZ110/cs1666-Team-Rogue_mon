use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Player<'a> {
  //delta_x_npc: i32,
  pos: Rect,
  texture: Texture<'a>,
  //flip: bool
}

impl<'a> Player<'a> {
  //Create a new instance of player struct
  pub fn create(pos: Rect, texture: Texture<'a>) -> Player {
    Player { pos, texture }
  }

  pub fn x(&self) -> i32 {
    self.pos.x()
  }
  pub fn y(&self) -> i32 {
    self.pos.y()
  }
  pub fn width(&self) -> u32 {
    self.pos.width()
  }
  pub fn height(&self) -> u32 {
    self.pos.height()
  }
  pub fn _set_x(&mut self, x_cor: i32) {
    self.pos.set_x(x_cor);
  }
  pub fn _set_y(&mut self, y_cor: i32) {
    self.pos.set_y(y_cor);
  }
  /*pub fn update_pos_x(&mut self, vel_x: i32, x_left_bound: i32, x_right_bound: i32) {
    self
      .pos
      .set_x((self.pos.x() + vel_x).clamp(x_left_bound, x_right_bound));
  }

  pub fn update_pos_y(&mut self, vel_y: i32, y_left_bound: i32, y_right_bound: i32){
    self
      .pos
      .set_y((self.pos.y() + vel_y).clamp(y_left_bound, y_right_bound));
  }*/

  pub fn texture(&self) -> &Texture {
    &self.texture
  }
}
