# zkSNARK Multiplier 

## Description
We have some public known number C. We want to prove that we know A, B, such that A * B = C, without revealing them.

- [x] CLI
- [x] Smart-contract

## How that works
Implemented in Rust, it works using Groth16 proving system (arkworks-rs/groth16). Verifier implemented as a smart-contract on NEAR Protocol; proofs can be generated using CLI.
___

### Requirements

### Rust Language + Wasm toolchain and NEAR-CLI (blockchain interaction)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown

npm install -g near-cli
```

## Usage

### Clone and compile
```bash
git clone git@github.com:curryrasul/near-zk-demo.git && cd near-zk-demo

cargo build --release -p multiplier-cli

cargo build -p contract --target wasm32-unknown-unknown --release && mkdir res && cp target/wasm32-unknown-unknown/release/contract.wasm res/contract.wasm
```

### Global variables configuration
```bash
mkdir snark # directory for serialized files
VKEY_PATH="snark/vkey"
PKEY_PATH="snark/pkey"
PROOF_PATH="snark/proof.json"
PUBLIC_INPUT_PATH="snark/public.json"
CONSTRUCTOR="snark/constructor.json"
WITNESS="snark/witness.json"
PUBLIC_INPUT="33" # f.e. public variable c will be = 33
```

### Making a proof & Verification in CLI
```bash
echo '{"a":"3", "b":"11"}' >> $WITNESS # private factors a, b, such as a * b = 33

./target/release/multiplier-cli setup --vkey=$VKEY_PATH --pkey=$PKEY_PATH # Trustless setup

./target/release/multiplier-cli prepare-public --input=$PUBLIC_INPUT --path=$PUBLIC_INPUT_PATH # Public input (C) to a right format

./target/release/multiplier-cli prove --pkey=$PKEY_PATH --witness=$WITNESS --proof=$PROOF_PATH # Make a proof

./target/release/multiplier-cli verify --public=$PUBLIC_INPUT_PATH --vkey=$VKEY_PATH --proof=$PROOF_PATH
```

### Smart-contract deploy and verification on a blokchain
```bash
near dev-deploy res/contract.wasm
```
> Need to copy created Account Id, In my case: dev-1653925037826-43957893328277

```bash
GAS_LIMIT=300000000000000

ACCOUNT_ID=dev-1653925037826-43957893328277
near call $ACCOUNT_ID new "$(cat $CONSTRUCTOR)" --accountId $ACCOUNT_ID # Smart-contract initialization

near call $ACCOUNT_ID verify "$(cat $PROOF_PATH)" --accountId $ACCOUNT_ID --gas $GAS_LIMIT # Verification
```
