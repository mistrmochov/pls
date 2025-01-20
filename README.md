**PLS**
Cli downloader written in blazingly fast rust!

**Install instructions**

Build dependencies:

rust (cargo)

 ```shell
git clone https://github.com/mistrmochov/pls
cd pls
cargo build --release
sudo cp target/release/pls /bin
```

**Usage**

First argument is URL
Second argument is output path
You can also pass -f or --force anywhere to allow overwriting. Can be at start in the middle or at the end.

Example:
 ```shell
pls https://github.com/mistrmochov/pls/raw/refs/heads/main/src/main.rs ~/Downloads
pls -f https://github.com/mistrmochov/pls/raw/refs/heads/main/src/main.rs ~/Downloads
```
