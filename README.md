# CsvToJson

Allows you to convert your csv files into json. 
This is my first rust project, so feel free to give any feedback.

# Usage
```shell
./csv_to_json --input <path> --output <path>
```

## input
This argument references to a path on your filesystem to the file you want to convert.
I assume that you have headers present. These headers are the keys of the exported json.
The file have to be encoded with utf8.
Globbing is supported to convert a bunch of files.

## verbose
When using the globbing feature, verbose mode is enabled by default. 
You can disable this behaviour by passing `--quiet true`

## output
Path to where to write the json output. This can be left out. 
In this case, the application will return the converted content on stdout.
This will return a json object in each line.

# build from source 
## Ubuntu

First you need to install rust (with cargo) if you haven't done that before:
```shell
sudo apt update && sudo apt upgrade
sudo apt install curl build-essential gcc make -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You can build the tool with the following command:

```shell
 cargo build --target x86_64-unknown-linux-gnu --release
```

Please note that this will build for linux gnu architecture. 
See the [cargo documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets)
for building it on a different environment than your current one.

You can find the binary under `target/x86_64-unknown-linux-gnu/release/csv_to_json`. 
In case you build it for a different environment, 
it's placed under target/<platform>/release/csv_to_json 

# contribute
In case you want to add features, please make sure `cargo fmt` and `cargo clippy --no-deps` have no warnings.

# Parallel Execution
When using globbing, all available cpu are used. In case your files are huge, you may get out of memory errors. 
You can set the amount of parallel execution with `RAYON_NUM_THREADS=4` where `4` is the number of threads that should be used.
