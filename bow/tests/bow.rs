use bow::frame::Frame;

#[test]
fn game_has_correct_pins() {
  let mut game = Frame::start_game();
  let expected = vec![String::from("4 5"),String::from("X"),String::from("5/"),String::from("4 5")];
  
  game.add_pins(4, 5, None);
  game.add_pins(10, 0, None);
  game.add_pins(5, 5, None);
  game.add_pins(4, 5, None);

  assert_eq!(game.result, expected)
}