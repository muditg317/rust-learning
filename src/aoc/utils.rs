use std::{fs};
use itertools::Itertools;

pub fn run_tasks<Data>(
    input_file: &str,
    loader: fn(String) -> Result<Data, Box<dyn std::error::Error + 'static>>,
    tasks: &[fn(&Data)->String],
    which_tasks: &Option<Vec<usize>>
  ) -> Result<String, Box<dyn std::error::Error + 'static>> {
  println!("Run AOC tasks: {input_file}");
  let file_contents: String = fs::read_to_string(input_file)?;
  let data: Data = loader(file_contents)?;
  return Ok(match which_tasks {
    None => tasks.iter().enumerate().map(|(i,task)| {
      format!("task{}: {}", i+1, task(&data))
    }).join("\n"),
    Some(which_ts) => which_ts.iter().map(|i| {
      format!("task{}: {}", i, tasks[i-1](&data))
    }).join("\n")
  });
}