pub struct Frame {
  pub result: Vec<String>,
}

impl Frame {
  pub fn start_game() -> Self {
    Frame {result: Vec::new()}
  }

  pub fn add_pins(&mut self, pin_1: i32, pin_2: i32, pin_3: Option<i32>)  {
    let is_last_round = self.result.clone().len();
    if is_last_round < 9 && pin_3.is_some() {
      panic!("Third pin can only be used in the last round")
    } else if is_last_round >= 10 {
      panic!("Only ten rounds are permitted")
    } else if is_last_round == 9 && (pin_1 == 10 || (pin_1 + pin_2) == 10 ) 
        && pin_3.is_some() {
      self.last_round_pins(pin_1, pin_2, pin_3.unwrap());
    } else {
      self.regular_pins(pin_1, pin_2);
    }
  }

  fn last_round_pins(&mut self,pin_1: i32, pin_2: i32, extra_pin: i32) {
    if pin_1 == 10 {
        self.result.push(format!("X {} {}", pin_2, extra_pin));
      } else {
        self.result.push(format!("{}/{}", pin_1, extra_pin));
      }
  }

  fn regular_pins(&mut self,pin_1: i32, pin_2: i32) {
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

  #[test]
  #[should_panic]
  fn add_third_pin_before_last_round() {
    let mut game = Frame::start_game();
    game.add_pins(7, 3, Some(4));
  }

  #[test]
  #[should_panic]
  fn add_more_than_10_round() {
    let mut game = Frame::start_game();
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
  }

  #[test]
  fn add_last_round_has_strike() {
    let mut game = Frame::start_game();
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(10, 3, Some(3));

    assert_eq!(game.result.last().unwrap(), &String::from("X 3 3"))
  }

  #[test]
  fn add_last_round_has_spare() {
    let mut game = Frame::start_game();
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, None);
    game.add_pins(7, 3, Some(3));

    assert_eq!(game.result.last().unwrap(), &String::from("7/3"))
  }
}
