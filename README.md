# zkSNARK demo on NEAR Protocol using arkworks-rs's Groth16

#### *Remark: Whole program is just PoC, CLI implementation is trash and Smart-Contract is not optimized*

## Description
We have some public known number C. We want to prove that we know A, B, such that A * B = C, without revealing it.

## How that works
It works using Groth16 proving system, implemented by arkworks. Verifier implemented on NEAR Protocol. And proofs can be generated using CLI.

- [x] CLI
- [x] Smart-contract

## CLI Usage
```bash
git clone git@github.com:curryrasul/near-zk-demo.git

Multiplier demo CLI 

USAGE:
    cli [OPTIONS]

OPTIONS:
    -h, --help      Print help information
        --prove     Make a proof
        --setup     Do a trusted setup (Pk/Vk serialization)
        --verify    Verify a proof
        
cargo run --release -p cli -- --setup
cargo run --release -p cli -- --prove
10 15
cargo run --release -p cli -- --verify
150
Verified well
```

> Works because 10 * 15 = 150