# nimiq-address-miner
A vanity address miner for Nimiq using key derivation.
It supports the new Nimiq addresses using key derivation.
By default, multithreading is enabled to speed up finding a match.

## Installation

Make sure that you have [Rust](https://www.rust-lang.org/learn/get-started#installing-rust) installed.
Compiling the project is then achieved through [`cargo`](https://doc.rust-lang.org/cargo/):

```bash
git clone https://github.com/paberr/nimiq-address-miner
cd nimiq-address-miner
cargo build --release
```
I recommend building in release mode for speed reasons.

## Usage

To run the Nimiq Address Miner run:

```bash
cargo run --release
```

This will also display the help instructions.
In the simplest case, you can specify the prefix you would like your address to have as such:

```bash
cargo run --release <PREFIX>
```

If you want to run the Nimiq Address Miner in loop mode and save the output to a file use:

```bash
cargo run --release -- --loop <PREFIX> >> output.txt
```
## License

This project is under the [MIT](./LICENSE).
