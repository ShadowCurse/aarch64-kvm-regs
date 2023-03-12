use std::{env::args, path::PathBuf};

mod arm;
mod registers;

use registers::AARCH64_KVM_REGISTERS;

fn print_usage_and_exit() -> ! {
    println!(
        "Usage: arm_regs <mode> <path>
where:
  mode: id/register
  path: path to file with ids/names"
    );
    std::process::exit(1);
}

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

fn find_by_id(path: PathBuf) -> Result<(), Error> {
    let content = std::fs::read_to_string(path).map_err(Error::Io)?;

    for line in content.lines() {
        let id = line.parse::<u64>().map_err(Error::Parse)?;
        let regs = AARCH64_KVM_REGISTERS
            .iter()
            .filter(|reg| reg.reg_id == id)
            .collect::<Vec<_>>();
        if !regs.is_empty() {
            for reg in regs {
                println!("id: {id} => {reg:?}");
            }
        } else {
            println!("id: {id} => None");
        }
    }
    Ok(())
}

fn find_by_register(path: PathBuf) -> Result<(), Error> {
    let content = std::fs::read_to_string(path).map_err(Error::Io)?;

    for line in content.lines() {
        let regs = AARCH64_KVM_REGISTERS
            .iter()
            .filter(|reg| reg.register == line)
            .collect::<Vec<_>>();
        if !regs.is_empty() {
            for reg in regs {
                println!("register: {line} => {reg:?}");
            }
        } else {
            println!("register: {line} => None");
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let args = args();
    if args.len() != 3 {
        print_usage_and_exit();
    }

    let args = args.into_iter().collect::<Vec<String>>();

    match args[1].as_str() {
        "id" => find_by_id(args[2].clone().into())?,
        "register" => find_by_register(args[2].clone().into())?,
        _ => print_usage_and_exit(),
    }

    Ok(())
}
