use clap::{Parser, Subcommand, ValueEnum};
use mimi_vm::KvmVcpuWrapper;
use std::path::PathBuf;

mod arm;
mod mimi_vm;

use arm::AARCH64_KVM_REGISTERS;

use crate::arm::reg_size;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Can not read file: {0}: {1}")]
    FileRead(PathBuf, std::io::Error),
    #[error("Can not parse value: {0}: {1}")]
    ParseNum(String, std::num::ParseIntError),
    #[error("Kvm error: {0}")]
    Kvm(#[from] mimi_vm::Error),
}

fn find_by_id(path: PathBuf) -> Result<(), Error> {
    let content = std::fs::read_to_string(path.as_path()).map_err(|e| Error::FileRead(path, e))?;

    for line in content.lines() {
        let id = line
            .parse::<u64>()
            .map_err(|e| Error::ParseNum(line.into(), e))?;
        let regs = AARCH64_KVM_REGISTERS
            .iter()
            .filter(|reg| reg.reg_id == id)
            .collect::<Vec<_>>();
        if !regs.is_empty() {
            for reg in regs {
                println!("id: {id} => {reg}");
            }
        } else {
            println!("id: {id} => None");
        }
    }
    Ok(())
}

fn find_by_register(path: PathBuf) -> Result<(), Error> {
    let content = std::fs::read_to_string(path.as_path()).map_err(|e| Error::FileRead(path, e))?;

    for line in content.lines() {
        let regs = AARCH64_KVM_REGISTERS
            .iter()
            .filter(|reg| reg.register == line)
            .collect::<Vec<_>>();
        if !regs.is_empty() {
            for reg in regs {
                println!("register: {line} => {reg}");
            }
        } else {
            println!("register: {line} => None");
        }
    }
    Ok(())
}

fn query(values: bool, names: bool, size: bool, hex: bool) -> Result<(), Error> {
    let kvm_vcpu = KvmVcpuWrapper::new()?;
    for (reg_id, val) in kvm_vcpu.query_registers()? {
        let reg_size = reg_size(reg_id);
        let regs = AARCH64_KVM_REGISTERS
            .iter()
            .filter(|reg| reg.reg_id == reg_id)
            .collect::<Vec<_>>();
        let print_info = || {
            print!("{reg_id}");
            if size {
                print!("{reg_size}");
            }
            if values {
                if hex {
                    print!("{val:#018x}");
                } else {
                    print!("{val}");
                }
            }
        };
        if regs.is_empty() {
            print_info();
        } else {
            for reg in regs {
                print_info();
                if names {
                    print!("{}", reg.register);
                }
                println!();
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum FindMode {
    Id,
    Register,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Find {
        #[arg(short, long)]
        mode: FindMode,
        #[arg(short, long)]
        path: PathBuf,
    },
    Query {
        #[arg(short, long)]
        values: bool,
        #[arg(short, long)]
        names: bool,
        #[arg(short, long)]
        size: bool,
        #[arg(long)]
        hex: bool,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Find { mode, path } => match mode {
            FindMode::Id => find_by_id(path)?,
            FindMode::Register => find_by_register(path)?,
        },
        Command::Query {
            values,
            names,
            size,
            hex,
        } => query(values, names, size, hex)?,
    }

    Ok(())
}
