use super::utils;

type ElvesData = Vec<Vec<usize>>;

/**
 * find the heaviest elf amount
 */
fn task1(elves: &ElvesData) -> String {
  let mut heaviest_load = 0;
  for elf in elves {
    let mut sum = 0;
    for item in elf {
      sum += item;
    }
    // println!("elf total: {sum}");
    if sum > heaviest_load {
      heaviest_load = sum;
    }
  }
  format!("{heaviest_load}")
}

/**
 * find the sum of top 3 heaviest elf amounts
 */
fn task2(elves: &ElvesData) -> String {
  let mut top3 = vec![0,0,0];
  for elf in elves {
    let mut sum = 0;
    for item in elf {
      sum += item;
    }
    // println!("elf total: {sum}");
    if sum > top3[0] {
      if sum > top3[1] {
        top3.remove(0);
        if sum > top3[1] {
          top3.push(sum);
        } else {
          top3.insert(1, sum);
        }
      } else {
        top3[0] = sum;
      }
    }
  }
  return format!("{}", top3[0] + top3[1] + top3[2]);
}

pub(super) fn run(which_tasks: &Option<Vec<usize>>) -> Result<String, Box<dyn std::error::Error + 'static>> {
  return utils::run_tasks("inputs/day1.txt", |file_contents| {
    let mut elves: ElvesData = Default::default();
    for elf_str in file_contents.split("\n\n") {
        let mut elf_data = Vec::new();
        for item in elf_str.split("\n") {
          elf_data.push(item.parse()?);
        }
        elves.push(elf_data);
    }
    return Ok(elves);
  }, &[
    task1, task2
  ], which_tasks);
}