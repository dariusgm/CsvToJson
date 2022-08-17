# CsvToJson

Allows you to convert your csv files into json. 
This is my first rust project, so feel free to give any feedback.

## Usage
```shell
./csv_to_json --input <path> --output <path>
```

## Examples
In this section we will provide some examples of calls and 
how the application will handle this calls. 

### using input
```shell
./csv_to_json --input document.csv
```
In this example, only an `input` path is provided. By default, a converted file
will be crated based on the input filename, with an added suffix ".json" - 
in the provided example document.csv.json will be created.

### using input with globbing
```shell
./csv_to_json --input *.csv
```

The application will expend the globbing parameter to a list of all files that match the pattern.
The resulting files will be named with an appended ".json" suffix to the original filename. 

### using input and output
```shell
./csv_to_json --input document.csv --output my.json
```

In this example, `input` is provided and an explicit `output` directory.
The converted file will be named `my.json`.

### using input with globbing and output
```shell
./csv_to_json --input *.csv --output converted
```

This example uses a globbing list and provides and output.
In this case, the output will is a new parent directory for all converted files,
respecting the current directory structure of the input path.



# build from source 
## Ubuntu

First you need to install rust (with cargo) if you haven't done that before:
```shell
sudo apt update && sudo apt upgrade
sudo apt install curl build-essential gcc make -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You can than build the tool with optimizations with the following command:

```shell
 cargo build --release
```

Please note that this will build for your current system architecture.
You can find the binary under `target/release/csv_to_json`.

To build it for a different system architecture,
visit the [cargo documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets).

In that case, the binary is placed under target/<platform>/release/csv_to_json 

# contribute
In case you want to add features, please make sure `cargo fmt` and `cargo clippy --no-deps` have no warnings.
see `build.sh` for more details.
You can run a benchmark by using `cargo bench`.

# Parallel Execution
When using globbing, all available cpu are used. 
You can set the amount of parallel execution with `RAYON_NUM_THREADS=4` where `4` is the number of threads that should be used.

# multiple input files
You can pass multiple files in the input argument.
