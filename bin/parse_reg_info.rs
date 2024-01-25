use std::{io::Write, path::PathBuf};

use clap::Parser;
use quick_xml::{events::Event, Reader};

use aarch64_kvm_regs::arm::Aarch64KvmRegister;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Could not read xml root: {0}")]
    ReadXmlRoot(std::io::Error),
    #[error("Could not read xml root entry: {0}")]
    ReadXmlRootEntry(std::io::Error),
    #[error("Could not read xml file: {0}")]
    ReadXmlFile(quick_xml::Error),
    #[error("Could not parse xml file: {0}")]
    ParseXmlFile(quick_xml::Error),
    #[error("Could not read xml attribute: {0}")]
    ReadXmlAttribute(quick_xml::events::attributes::AttrError),
    #[error("Could not create output file: {0}")]
    CreateOutputFile(std::io::Error),
    #[error("Could not write into output file: {0}")]
    WriteOutputFile(std::io::Error),
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    xml_root_path: PathBuf,
    #[arg(short, long)]
    output_path: PathBuf,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Eq)]
struct Aarch64KvmRegisterInfo {
    name: Option<String>,
    op0: Option<u64>,
    op1: Option<u64>,
    CRn: Option<u64>,
    CRm: Option<u64>,
    op2: Option<u64>,
}

impl TryFrom<Aarch64KvmRegisterInfo> for Aarch64KvmRegister {
    type Error = ();
    fn try_from(value: Aarch64KvmRegisterInfo) -> Result<Self, Self::Error> {
        if value.name.is_none()
            || value.op0.is_none()
            || value.op1.is_none()
            || value.CRn.is_none()
            || value.CRm.is_none()
            || value.op2.is_none()
        {
            return Err(());
        }
        Ok(Self::new(
            value.name.unwrap(),
            value.op0.unwrap(),
            value.op1.unwrap(),
            value.CRn.unwrap(),
            value.CRm.unwrap(),
            value.op2.unwrap(),
        ))
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut reg_infos = vec![];
    for entry in std::fs::read_dir(&cli.xml_root_path).map_err(Error::ReadXmlRoot)? {
        let entry = entry.map_err(Error::ReadXmlRootEntry)?;
        let entry_path = entry.path();
        // skip directories
        if entry_path.is_dir() {
            continue;
        }
        // skip not xml files
        if let Some(ext) = entry_path.extension() {
            if ext != "xml" {
                continue;
            }
        }
        let mut reader = Reader::from_file(&entry_path).map_err(Error::ReadXmlFile)?;

        let mut buffer = vec![];
        let mut reg_info = Aarch64KvmRegisterInfo::default();
        // tracking if we are in the name tag to read the
        // register name
        let mut in_reg_short_name = false;
        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(event) => match event {
                    Event::Start(start) => {
                        if let b"reg_short_name" = start.name().as_ref() {
                            in_reg_short_name = true
                        }
                    }
                    Event::Text(text) => {
                        if in_reg_short_name {
                            reg_info.name = Some(text.unescape().unwrap().into_owned());
                        }
                    }
                    Event::Empty(empty) => {
                        if let b"enc" = empty.name().as_ref() {
                            let attributes = empty
                                .attributes()
                                .map(|a| a.map(|a| a.value).map_err(Error::ReadXmlAttribute))
                                .collect::<Result<Vec<_>, _>>()?;

                            macro_rules! parse_attribute {
                                () => {{
                                    let s = unsafe {
                                        std::str::from_utf8_unchecked(attributes[1].as_ref())
                                    };
                                    let v = match u64::from_str_radix(&s[2..], 2) {
                                        Ok(v) => v,
                                        Err(_) => break,
                                    };
                                    Some(v)
                                }};
                            }

                            match attributes[0].as_ref() {
                                b"op0" => reg_info.op0 = parse_attribute!(),
                                b"op1" => reg_info.op1 = parse_attribute!(),
                                b"CRn" => reg_info.CRn = parse_attribute!(),
                                b"CRm" => reg_info.CRm = parse_attribute!(),
                                b"op2" => reg_info.op2 = parse_attribute!(),
                                _ => {
                                    break;
                                }
                            }
                        }
                    }
                    Event::End(end) => {
                        if let b"reg_short_name" = end.name().as_ref() {
                            in_reg_short_name = false
                        }
                    }
                    Event::Eof => break,
                    _ => {}
                },
                Err(e) => Err(Error::ParseXmlFile(e))?,
            }
            buffer.clear();
        }
        reg_infos.push(reg_info);
    }

    let regs: Vec<Aarch64KvmRegister> = reg_infos
        .into_iter()
        .filter_map(|info| Aarch64KvmRegister::try_from(info).ok())
        .collect();
    let regs_json = serde_json::to_string(&regs).expect("Could not convert to json");

    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&cli.output_path)
        .map_err(Error::CreateOutputFile)?;
    output_file
        .write_all(regs_json.as_bytes())
        .map_err(Error::WriteOutputFile)?;

    Ok(())
}
