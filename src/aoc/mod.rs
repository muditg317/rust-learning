mod utils;

mod day1;
mod day2;

type DayFunc = fn(&Option<Vec<usize>>)->Result<String, Box<dyn std::error::Error + 'static>>;

const AOC_DAY_FUNCS: &[DayFunc] = &[
  day1::run,
  day2::run
];

pub fn run_aoc(which_day: &Option<usize>, which_tasks: &Option<Vec<usize>>) -> Result<(), Box<dyn std::error::Error + 'static>> {
  match which_day {
      Some(day) => println!("{}", AOC_DAY_FUNCS[day-1](which_tasks)?),
      None => {
        for day_func in AOC_DAY_FUNCS {
          println!("{}", day_func(&None)?);
        }
      }
  }
  Ok(())
}