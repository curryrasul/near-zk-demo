# zkSNARK demo on NEAR Protocol using arkworks-rs's Groth16

## Description
We have some number C. We want to prove that we know private inputs a, b, such that a * b = c.

## How that works
It works using Groth16 proving system, implemented by arkworks. Verifier implemented on NEAR Protocol. And proofs can be generated using CLI.

## Usage
```bash
git clone git@github.com:curryrasul/near-zk-demo.git
cargo build --release --bin cli
```
