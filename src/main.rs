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

fn query() -> Result<(), Error> {
    let kvm_vcpu = KvmVcpuWrapper::new()?;
    for reg in kvm_vcpu.query_registers()? {
        println!("{reg}");
    }
    Ok(())
}

fn query_with_values() -> Result<(), Error> {
    let kvm_vcpu = KvmVcpuWrapper::new()?;
    for (reg, val) in kvm_vcpu.query_registers_with_values()? {
        println!("{reg} {val}");
    }
    Ok(())
}

fn query_with_names() -> Result<(), Error> {
    let kvm_vcpu = KvmVcpuWrapper::new()?;
    for reg_id in kvm_vcpu.query_registers()? {
        let regs = AARCH64_KVM_REGISTERS
            .iter()
            .filter(|reg| reg.reg_id == reg_id)
            .collect::<Vec<_>>();
        if !regs.is_empty() {
            for reg in regs {
                println!("{reg_id} {}", reg.register);
            }
        } else {
            println!("{reg_id} none");
        }
    }
    Ok(())
}

fn query_with_values_and_names() -> Result<(), Error> {
    let kvm_vcpu = KvmVcpuWrapper::new()?;
    for (reg_id, val) in kvm_vcpu.query_registers_with_values()? {
        let regs = AARCH64_KVM_REGISTERS
            .iter()
            .filter(|reg| reg.reg_id == reg_id)
            .collect::<Vec<_>>();
        if !regs.is_empty() {
            for reg in regs {
                println!("{reg_id} {val} {}", reg.register);
            }
        } else {
            println!("{reg_id} {val} none");
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
        #[arg(long)]
        with_values: bool,
        #[arg(long)]
        with_names: bool,
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
            with_values,
            with_names,
        } => match (with_values, with_names) {
            (false, false) => query()?,
            (true, false) => query_with_values()?,
            (false, true) => query_with_names()?,
            (true, true) => query_with_values_and_names()?,
        },
    }

    Ok(())
}
