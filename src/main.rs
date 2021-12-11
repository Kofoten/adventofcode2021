use clap::{App, Arg, ArgMatches};
use std::io::{Error, ErrorKind};
use core_challange::Challange;

mod core_challange;
mod template;

mod challange01;
mod challange02;
mod challange03;
mod challange04;
mod challange05;
mod challange06;
mod challange07;
mod challange08;

fn exec(day: u32, part: u32, filename: &str) -> Result<String, Error> {
    let challange: Box<dyn Challange> = match day {
        0 => Box::new(template::ChallangeTemplate {}),
        1 => Box::new(challange01::Challange01 {}),
        2 => Box::new(challange02::Challange02 {}),
        3 => Box::new(challange03::Challange03 {}),
        4 => Box::new(challange04::Challange04 {}),
        5 => Box::new(challange05::Challange05 {}),
        6 => Box::new(challange06::Challange06 {}),
        7 => Box::new(challange07::Challange07 {}),
        8 => Box::new(challange08::Challange08 {}),
        _ => return Err(Error::new(ErrorKind::Other, "Invalid day")),
    };

    match part {
        1 => Ok(challange.run_part_1(filename)),
        2 => Ok(challange.run_part_2(filename)),
        _ => return Err(Error::new(ErrorKind::Other, "Invalid day")),
    }
}

fn main() -> std::io::Result<()> {
    let matches: ArgMatches = App::new("Rasmus AOC 2021 solutions")
        .arg(Arg::new("day").required(true).index(1))
        .arg(Arg::new("part").required(true).index(2))
        .arg(Arg::new("filename").default_value("").index(3))
        .get_matches();

    let day = matches.value_of_t::<u32>("day").unwrap();
    let part = matches.value_of_t::<u32>("part").unwrap();
    let filename = matches.value_of("filename").unwrap();

    match exec(day, part, filename) {
        Ok(result) => {
            println!("{}", &result);
            Ok(())
        },
        Err(error) => Err(error),
    }
}

