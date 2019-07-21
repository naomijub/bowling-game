pub struct Frame {
  pub result: Vec<String>,
}

impl Frame {
  pub fn start_game() -> Self {
    Frame {result: Vec::new()}
  }

  pub fn add_pins(&mut self, pin_1: i32, pin_2: i32, pin_3: Option<i32>)  {
    let pins = match (pin_1, pin_2) {
      (p1, _) if p1 == 10 => String::from("X"),
      (p1, p2) if p1 + p2 == 10 => format!("{}/",p1),
      (p1, p2) => format!("{} {}",p1, p2),
    };
    self.result.push(pins);
  }
}

#[cfg(test)]
mod tests {
  use super::{Frame};

  #[test]
  fn startes_game_with_empty_vec() {
    let game = Frame::start_game();
    assert_eq!(game.result.len(), 0);
  }

  #[test]
  fn add_first_pins_3_4_none() {
    let mut game = Frame::start_game();
    game.add_pins(3, 4, None);
    assert_eq!(game.result.clone().len(), 1);
    assert_eq!(game.result, vec![String::from("3 4")]);
  }

  #[test]
  fn add_strike() {
    let mut game = Frame::start_game();
    game.add_pins(10, 0, None);
    assert_eq!(game.result, vec![String::from("X")]);
  }

  #[test]
  fn add_spare() {
    let mut game = Frame::start_game();
    game.add_pins(7, 3, None);
    assert_eq!(game.result, vec![String::from("7/")]);
  }
}
