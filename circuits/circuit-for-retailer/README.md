# ZK for Supply Chain in SP1 / Succinct

## Introduction

- SP1 / Succinct would be used for generating/verifying a ZKP, which is attested to the document of supply chain

<br>

## Installation
- 1/ Compile program
```shell
cd program
cargo prove build
```

- 2/ Executing program without generating a proof.
```shell
cd script
RUST_LOG=info cargo run --release -- --execute
```

- 3/ Executing program with generating a proof.
```shell
cd script
RUST_LOG=info cargo run --release -- --prove
```

<br>

## Installation - Script
- Run the program and script at the same time:
```bash
(at the root directory)

sh run_program_and_script.sh
```


<br>

## NOTE
- Currently, this project has been in progress.

<br>

## References
