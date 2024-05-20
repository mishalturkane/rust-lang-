use std::collections::HashMap;

fn main() {
  let mut score = HashMap::new();

    score.insert(34,"Mishal".to_string());
    score.insert(36,"Mohan".to_string());

    println!("{:?}",score);


   let mishal = score.get(&36);
  println!("{:?}",mishal);

  score.remove(&36);
  println!("{:?}",score);ar


}
