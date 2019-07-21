#[derive(Clone)]
pub struct Frame {
  pub result: Vec<String>,
}

impl Frame {
  pub fn start_game() -> Self {
    Frame {result: Vec::new()}
  }

  pub fn add_pins(mut self, pin_1: i32, pin_2: i32, pin_3: Option<i32>) -> Self  {
    let is_last_round = self.result.clone().len();
    if is_last_round < 9 && pin_3.is_some() {
      panic!("Third pin can only be used in the last round")
    } else if is_last_round >= 10 {
      panic!("Only ten rounds are permitted")
    } else if is_last_round == 9 && (pin_1 != 10 && (pin_1 + pin_2) != 10 ){
      panic!("Extra pin can only be played if you strike or spare")
    } else if is_last_round == 9 && (pin_1 == 10 || (pin_1 + pin_2) == 10 ) 
        && pin_3.is_some() {
      return Frame {result: last_round_pins(&mut self.result, pin_1, pin_2, pin_3.unwrap())};
    } else {
      Frame{result: regular_pins(&mut self.result, pin_1, pin_2)}
    }
  }

  pub fn to_string(&self) -> String {
    self.result.clone().join("|")
  }
}

fn last_round_pins(result: &mut Vec<String>,pin_1: i32, pin_2: i32, extra_pin: i32) -> Vec<String> {
  if pin_1 == 10 {
      result.push(format!("X {} {}", pin_2, extra_pin));
    } else {
      result.push(format!("{}/{}", pin_1, extra_pin))
    }
  result.clone()
}

fn regular_pins(result: &mut Vec<String> ,pin_1: i32, pin_2: i32) -> Vec<String> {
  let pins = match (pin_1, pin_2) {
      (p1, _) if p1 == 10 => String::from("X"),
      (p1, p2) if p1 + p2 == 10 => format!("{}/",p1),
      (p1, p2) => format!("{} {}",p1, p2),
    };
  result.push(pins);
  result.clone()
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
    let game = Frame::start_game();
    assert_eq!(game.add_pins(3, 4, None).result, vec![String::from("3 4")]);
  }

  #[test]
  fn add_strike() {
    let game = Frame::start_game();
    assert_eq!( game.add_pins(10, 0, None).result, vec![String::from("X")]);
  }

  #[test]
  fn add_spare() {
    let game = Frame::start_game();
    assert_eq!(game.add_pins(7, 3, None).result, vec![String::from("7/")]);
  }

  #[test]
  #[should_panic]
  fn add_third_pin_before_last_round() {
    let game = Frame::start_game();
    game.add_pins(7, 3, Some(4));
  }

  #[test]
  #[should_panic]
  fn add_more_than_10_round() {
    let game = Frame::start_game();
    game.add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None);
  }

  #[test]
  #[should_panic]
  fn add_last_round_does_not_have_strike_or_spare() {
    let game = Frame::start_game();
    game.add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(5, 3, Some(3));
  }

  #[test]
  fn add_last_round_has_strike() {
    let game = Frame::start_game();
    let actual = game.add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(10, 3, Some(3));

    assert_eq!(actual.result.last().unwrap(), &String::from("X 3 3"))
  }

  #[test]
  fn add_last_round_has_spare() {
    let game = Frame::start_game();
    let actual = game.add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, None)
        .add_pins(7, 3, Some(3));

    assert_eq!(actual.result.last().unwrap(), &String::from("7/3"))
  }

  #[test]
  fn converts_game_to_string() {
    let game = Frame::start_game();
    let expected = String::from("7/|7 2|7/|7 2|7/|5 3|7/|5 3|7/|X 3 3");
    let actual = game.add_pins(7, 3, None)
        .add_pins(7, 2, None)
        .add_pins(7, 3, None)
        .add_pins(7, 2, None)
        .add_pins(7, 3, None)
        .add_pins(5, 3, None)
        .add_pins(7, 3, None)
        .add_pins(5, 3, None)
        .add_pins(7, 3, None)
        .add_pins(10, 3, Some(3));

    assert_eq!(actual.to_string(), expected)
  }
}
