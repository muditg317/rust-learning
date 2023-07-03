use std::{str::FromStr};

use itertools::Itertools;

use super::utils;

enum RpsOption {
  Rock,
  Paper,
  Scissors
}

enum RpsMatch {
  Loss,
  Tie,
  Win,
}

impl RpsOption {
  fn choice_score(&self) -> u32 {
    match *self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3
    }
  }
  fn score_against(&self, other: &Self) -> u32 {
    self.choice_score() + RpsMatch::from_plays(other, self).match_score()
  }

  fn from_opp_result(opp: &Self, res: &RpsMatch) -> Self {
    match *opp {
      Self::Rock => match *res {
        RpsMatch::Tie => Self::Rock,
        RpsMatch::Loss => Self::Scissors,
        RpsMatch::Win => Self::Paper
      },
      Self::Paper => match *res {
        RpsMatch::Win => Self::Scissors,
        RpsMatch::Tie => Self::Paper,
        RpsMatch::Loss => Self::Rock
      },
      Self::Scissors => match *res {
        RpsMatch::Loss => Self::Paper,
        RpsMatch::Win => Self::Rock,
        RpsMatch::Tie => Self::Scissors
      }
    }
  }
}

impl RpsMatch {
  fn match_score(&self) -> u32 {
    match *self {
      Self::Loss => 0,
      Self::Tie => 3,
      Self::Win => 6,
    }
  }

  fn from_plays(opp: &RpsOption, you: &RpsOption) -> Self {
    match *you {
      RpsOption::Rock => match *opp {
        RpsOption::Rock => Self::Tie,
        RpsOption::Paper => Self::Loss,
        RpsOption::Scissors => Self::Win
      },
      RpsOption::Paper => match *opp {
        RpsOption::Rock => Self::Win,
        RpsOption::Paper => Self::Tie,
        RpsOption::Scissors => Self::Loss
      },
      RpsOption::Scissors => match *opp {
        RpsOption::Rock => Self::Loss,
        RpsOption::Paper => Self::Win,
        RpsOption::Scissors => Self::Tie
      }
    }
  }
}

#[derive(Debug)]
struct ParseRpsError;

impl FromStr for RpsOption {
  type Err = ParseRpsError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    RpsOption::try_from(&s.chars().next().ok_or(ParseRpsError {})?)
  }
}

type RpsGuide = Vec<(char, char)>;

impl TryFrom<&char> for RpsOption {
  type Error = ParseRpsError;
  fn try_from(value: &char) -> Result<Self, Self::Error> {
    match value {
      'A' | 'X' => Ok(Self::Rock),
      'B' | 'Y' => Ok(Self::Paper),
      'C' | 'Z' => Ok(Self::Scissors),
      _ => Err(ParseRpsError)
    }
  }
}

impl TryFrom<&char> for RpsMatch {
  type Error = ParseRpsError;
  fn try_from(value: &char) -> Result<Self, Self::Error> {
    match value {
      'X' => Ok(Self::Loss),
      'Y' => Ok(Self::Tie),
      'Z' => Ok(Self::Win),
      _ => Err(ParseRpsError)
    }
  }
}

fn task1(guide: &RpsGuide) -> String {
  format!("{}", guide.iter()
    .map(|(opp,you)| RpsOption::try_from(you).and_then(|you| Ok(you.score_against(&RpsOption::try_from(opp)?))))
    .filter(Result::is_ok)
    .map(Result::unwrap)
    .sum::<u32>()
  )
}

fn task2(guide: &RpsGuide) -> String {
  format!("{:?}", guide.iter()
    .map(|(opp,res)| Ok::<(RpsOption,RpsMatch),ParseRpsError>((RpsOption::try_from(opp)?,RpsMatch::try_from(res)?)))
    .filter(Result::is_ok)
    .map(Result::unwrap)
    .map(|(opp,res)| RpsOption::from_opp_result(&opp, &res).choice_score() + res.match_score())
    .sum::<u32>()
    // .collect_vec()
  )
}

pub(super) fn run(which_tasks: &Option<Vec<usize>>) -> Result<String, Box<dyn std::error::Error + 'static>> {
  return utils::run_tasks("inputs/day2.txt", |file_contents| {
    let mut guide: RpsGuide = Default::default();
    for round in file_contents.split('\n') {
      guide.push(round.split(' ')
        // .flat_map(RpsOption::from_str)
        .filter_map(|s| s.chars().next())
        .collect_tuple()
        .expect("Each line should be [ABC] [XYZ]")
      );
    }
    return Ok(guide);
  }, &[
    task1, task2
  ], which_tasks);
}