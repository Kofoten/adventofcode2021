use clap::{App, Arg, ArgMatches};
use core_challange::Challange;
use std::io::{Error, ErrorKind};

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
mod challange09;
mod challange10;
mod challange11;
mod challange12;
mod challange13;
mod challange14;
mod challange15;
mod challange16;
mod challange17;
mod challange18;
mod challange19;
mod challange20;
mod challange21;
mod challange22;
mod challange23;
mod challange24;
mod challange25;

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
        9 => Box::new(challange09::Challange09 {}),
        10 => Box::new(challange10::Challange10 {}),
        11 => Box::new(challange11::Challange11 {}),
        12 => Box::new(challange12::Challange12 {}),
        13 => Box::new(challange13::Challange13 {}),
        14 => Box::new(challange14::Challange14 {}),
        15 => Box::new(challange15::Challange15 {}),
        16 => Box::new(challange16::Challange16 {}),
        17 => Box::new(challange17::Challange17 {}),
        18 => Box::new(challange18::Challange18 {}),
        19 => Box::new(challange19::Challange19 {}),
        20 => Box::new(challange20::Challange20 {}),
        21 => Box::new(challange21::Challange21 {}),
        22 => Box::new(challange22::Challange22 {}),
        23 => Box::new(challange23::Challange23 {}),
        24 => Box::new(challange24::Challange24 {}),
        25 => Box::new(challange25::Challange25 {}),
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
        }
        Err(error) => Err(error),
    }
}
