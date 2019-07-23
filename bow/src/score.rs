pub fn get_scores(scores: &mut Vec<String>) -> Vec<i32> {
  recur_scores(scores.remove(0), scores, &mut Vec::new())
}

fn recur_scores(head: String, tail:  &mut Vec<String>, scores: &mut Vec<i32>) -> Vec<i32> {
  if head.is_empty() && tail.len() <= 0 { return scores.clone(); }
  match &head {
    head if head.contains("X") => {
      let head_score = count_strike(head.to_owned(), &tail[0..2]);
      scores.push(head_score);
      recur_scores(tail.remove(0), tail, scores)
    }, 
    head if head.contains("/") => {
      let head_score = count_spare(head.to_owned(), &tail.first());
      scores.push(head_score);
      recur_scores(tail.remove(0), tail, scores)
    },
    head if tail.len() <= 0 => {
      let head_score = count_numerals(head.to_owned());
      scores.push(head_score);
      scores.to_vec()
    },
    _ => {
      let head_score = count_numerals(head);
      scores.push(head_score);
      recur_scores(tail.remove(0), tail, scores)
    }
  }
}

pub fn count_spare(_head: String, next: &Option<&String>) -> i32 {
  if next.unwrap().contains("X") {
    return 20;
  } else if next.unwrap().contains("/") {
    let s = next.unwrap().chars().into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
    return 10 + s.first().unwrap().parse::<i32>().unwrap();
  } else {
    let v: Vec<&str> = next.unwrap().split(' ').collect();
    10 + v.first().unwrap().parse::<i32>().unwrap()
  }
}

pub fn count_numerals(head: String) -> i32 {
  let v: Vec<&str> = head.split(' ').collect();
  v.into_iter().map(|n| n.parse::<i32>().unwrap()).fold(0,|acc,n| acc + n)
}

fn count_strike(_head: String, next: &[String]) -> i32 {
  match next.first().unwrap() {
    n if n.contains("/") => 20,
    n => 10 + count_numerals(n.to_owned()),
  }
}

#[cfg(test)]
mod tests {
  use super::{count_numerals, count_spare, count_strike};

  #[test]
  fn counts_numeral_score() {
    let value = count_numerals(String::from("5 3"));
    assert_eq!(value, 8);
  }

  #[test]
  fn counts_spare_score_followed_by_numerals() {
    let next = &String::from("7 2");
    let value = count_spare(String::from("5/"), &Some(next));
    assert_eq!(value, 17);
  }

  #[test]
  fn counts_spare_score_followed_by_spare() {
    let next = &String::from("6/");
    let value = count_spare(String::from("5/"), &Some(next));
    assert_eq!(value, 16);
  }

  #[test]
  fn counts_spare_score_followed_by_strike() {
    let next = &String::from("X");
    let value = count_spare(String::from("5/"), &Some(next));
    assert_eq!(value, 20);
  }

  #[test]
  fn strike_followed_by_numerals() {
      let next = vec![String::from("3 4"), String::from("5 3"), String::from("7 2")];
      let score = count_strike(String::from("X"), &next[0..2]);
      assert_eq!(score, 17);
  }

  #[test]
  fn strike_followed_by_spare() {
      let next = vec![String::from("3/"), String::from("5 3"), String::from("7 2")];
      let score = count_strike(String::from("X"), &next[0..2]);
      assert_eq!(score, 20);
  }
}