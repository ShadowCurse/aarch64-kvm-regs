Tool to find info about cpu registers available in KVM on aach64.

## Build

```bash
$ cargo build --release
```

## Usage

There are 2 modes: 
- `find`
- `query`

### `Find`
Finds the info about the KVM registers.
Has 2 modes:
- `id` - finds register based on its id
- `register` - finds register based on its name
Takes file with register id/name on each row.

### `Query`
Creates basic KVM vm and queries all available registers from it.
Options:
- `values` - adds register value in decimal to the output
- `names` - adds register name to the output 
- `hex` - if `values` is specified then they will be printed in hex 

