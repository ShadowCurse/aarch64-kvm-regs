use clap::Parser;
use std::path::PathBuf;

use aarch64_kvm_regs::{
    arm::{reg_size, Aarch64KvmRegister},
    mimi_vm::{self, KvmVcpuWrapper},
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Can not read file with reg names: {0}")]
    NameFileRead(std::io::Error),
    #[error("Can not parse file with reg names: {0}")]
    ParseFile(serde_json::Error),
    #[error("Kvm error: {0}")]
    Kvm(#[from] mimi_vm::Error),
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    value: bool,
    #[arg(short, long)]
    size: bool,
    #[arg(short, long)]
    name_file_path: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let regs_names = if let Some(path) = cli.name_file_path {
        let content = std::fs::read_to_string(path).map_err(Error::NameFileRead)?;
        let regs_names =
            serde_json::from_str::<Vec<Aarch64KvmRegister>>(&content).map_err(Error::ParseFile)?;
        Some(regs_names)
    } else {
        None
    };

    let kvm_vcpu = KvmVcpuWrapper::new()?;
    for (reg_id, reg_value) in kvm_vcpu.query_registers()? {
        print!("{reg_id:#x}");
        if cli.size {
            let reg_size = reg_size(reg_id);
            print!(" {reg_size}");
        }
        if cli.value {
            let v = if let Some(p) = reg_value.iter().rev().position(|v| v != &0) {
                &reg_value[..(reg_value.len() - p)]
            } else {
                // all are zeros so just use slice with
                // one zero
                &reg_value[..1]
            };
            print!(
                " 0x{}",
                v.iter().rev().fold(String::new(), |mut output, b| {
                    use std::fmt::Write;
                    let _ = write!(output, "{b:x}");
                    output
                })
            );
        }
        if let Some(ref rn) = regs_names {
            for reg in rn.iter() {
                if reg.id == reg_id {
                    print!(" {} ", reg.name);
                }
            }
        }
        println!();
    }
    Ok(())
}
