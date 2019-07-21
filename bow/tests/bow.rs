use bow::frame::Frame;

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