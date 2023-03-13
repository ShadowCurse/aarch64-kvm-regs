use clap::{Parser, Subcommand, ValueEnum};
use mimi_vm::KvmVcpuWrapper;
use std::path::PathBuf;

mod arm;
mod mimi_vm;

use arm::AARCH64_KVM_REGISTERS;

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
                println!("id: {id} => {reg:?}");
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
                println!("register: {line} => {reg:?}");
            }
        } else {
            println!("register: {line} => None");
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
    Query,
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
        Command::Query => {
            let kvm_vcpu = KvmVcpuWrapper::new()?;
            for reg in kvm_vcpu.query_registers()? {
                println!("{reg}");
            }
        }
    }

    Ok(())
}
