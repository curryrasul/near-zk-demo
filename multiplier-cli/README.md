# zkSNARK Multiplier CLI

#### *Works using arkworks-rs/groth16 crate* 

## Usage

```bash
Multiplier zkSNARK CLI 1.0
Magamedrasul I. <curryrasul@gmail.com>
zkSNARK prover/verifier for knowledge of private factors of public C

USAGE:
    multiplier-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    build-constructor    Build a constructor.json for the smart-contract
    help                 Print this message or the help of the given subcommand(s)
    prepare-public       Prepare public inputs to specified path
    prove                Create a proof with specified Pkey and witness
    setup                Do a trustless setup (Pkey / Vkey)
    verify               Verify a proof with specified public parameters, Vkey and proof
```