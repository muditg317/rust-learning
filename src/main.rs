mod aoc;

use std::env;

use crate::aoc::run_aoc;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  let mut which_day: Option<usize> = None;
  let mut which_task: Option<Vec<usize>> = None;
  for (i, arg) in env::args().enumerate() {
    if i == 0 {
      continue;
    }
    match which_day {
      None => {
        let _ = which_day.insert(arg.parse().expect("Pass in single number to indicate which day of AOC to run"));
      },
      Some(_) => {
        let tasks = which_task.get_or_insert(Default::default());
        arg.parse()
          .and_then(|num| Ok(tasks.push(num)))
          .or_else(|e| {
            if arg.contains(",") {
              return Ok(arg.split(",")
                .filter_map(|s| s.parse().ok())
                .for_each(|s| tasks.push(s))
              );
            }
            Err(e)
          })?;
      }
    }
  }
  println!("day: {:?}, tasks: {:?}", which_day, which_task);
  run_aoc(&which_day, &which_task)?;
  Ok(())
}
