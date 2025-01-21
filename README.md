**PLS**

Cli downloader written in blazingly fast rust!
__________________________________________________________________________________________________________________________________________________________________________________________

**Install instructions**

Build dependencies:

rust (cargo)

 ```shell
git clone https://github.com/mistrmochov/pls
cd pls
cargo build --release
sudo cp target/release/pls /bin
```

If you have arch linux, you can add my public repo and don't have to build it. Add this to /etc/pacman.conf
 ```shell
[mochov]                                                                                                                               
Server = https://storage.googleapis.com/mochov-public/repo                                                                             
SigLevel = Optional TrustAll
```

And then simply run:
 ```shell
sudo pacman -Sy pls
```

**Usage**

First argument is URL
Second argument is output path
You can also pass -f or --force anywhere to allow overwriting. Can be at start in the middle or at the end.

Example:
 ```shell
pls https://github.com/mistrmochov/pls/raw/refs/heads/main/src/main.rs ~/Downloads
```
```shell
pls -f https://github.com/mistrmochov/pls/raw/refs/heads/main/src/main.rs ~/Downloads
```
