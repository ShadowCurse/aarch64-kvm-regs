Tool to find info about cpu registers available in KVM on aach64.

## Build

```bash
$ cargo build --release
```

There are 2 binaries: 
- `kvm_regs`
- `parse_reg_info`

## `kvm_regs`
This is used to query KVM registers and print their info.

### Usage
```bash
Usage: kvm_regs [OPTIONS]

Options:
  -v, --value
  -s, --size
  -n, --name-file-path <NAME_FILE_PATH>
  -h, --help                             Print help
```

Example
```bash
$ kvm_regs -v -s -n regs_info.json
```

## `parse_reg_info`
This is used to parse `.xml` files from ARM documentation.
The documentation can be found [here](https://developer.arm.com/downloads/-/exploration-tools)
just download `XML` archive and unpack.

### Usage
```bash
Usage: parse_reg_info --xml-root-path <XML_ROOT_PATH> --output-path <OUTPUT_PATH>

Options:
  -x, --xml-root-path <XML_ROOT_PATH>
  -o, --output-path <OUTPUT_PATH>
  -h, --help                           Print help
```

Example
```bash
$ parse_reg_info -x ./SysReg_xml_A_profile-2023-12/ -o out.json
```
