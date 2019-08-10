use bow::frame::Frame;
use bow::score::get_scores;

#[test]
fn game_has_correct_pins() {
  let game = Frame::start_game();
  let expected = vec![String::from("4 5"),String::from("X"),String::from("5/"),String::from("4 5")];
  
  let actual = game.add_pins(4, 5, None)
      .add_pins(10, 0, None)
      .add_pins(5, 5, None)
      .add_pins(4, 5, None);

  assert_eq!(actual.result, expected)
}

#[test]
fn can_get_scores_for_only_numerals() {
  let game = Frame::start_game();
  
  let mut actual = game.add_pins(4, 5, None)
      .add_pins(6, 0, None)
      .add_pins(5, 4, None)
      .add_pins(4, 3, None);
  let score = get_scores(&mut actual.result);

  assert_eq!(score, vec![9, 6, 9, 7])
}

#[test]
fn can_get_scores_for_one_spare() {
  let game = Frame::start_game();
  
  let mut actual = game.add_pins(4, 5, None)
      .add_pins(6, 4, None)
      .add_pins(5, 4, None)
      .add_pins(4, 3, None);
  let score = get_scores(&mut actual.result);

  assert_eq!(score, vec![9, 15, 9, 7])
}