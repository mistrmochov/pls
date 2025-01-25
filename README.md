**PLS**

**Cli downloader written in blazingly fast rust!** \
This is a incredibly fast downloader with sleek loading bar! Now you can also download videos from YouTube or any other video website. It's using yt-dlp, which is automatically installed to ~/.local/share/pls or ~\AppData\Roaming\pls . It doesn't depend on system binaries for being portable.
__________________________________________________________________________________________________________________________________________________________________________________________

**Install instructions (Linux)**

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

**Install instructions (Windows)**

Just simply download this installer and install it. It automatically adds the binary to path environment variable. \
https://github.com/mistrmochov/pls/releases/latest/download/pls-installer-win.exe

**Usage**

First argument is URL \
Second argument is output path \
You can also pass -f or --force anywhere to allow overwriting. Can be at start in the middle or at the end. \
For downloading videos you have to pass -m or --media. Can be combined with -f as well. \
To trigger update of yt-dlp and ffmpeg binaries, you have to pass --update or -u alone.

Example (file downloading):
 ```shell
pls https://github.com/mistrmochov/pls/raw/refs/heads/main/src/main.rs ~/Downloads
```
```shell
pls -f https://github.com/mistrmochov/pls/raw/refs/heads/main/src/main.rs ~/Downloads
```

Example (video downloading):
 ```shell
pls -m https://www.youtube.com/watch?v=iWDOO1vXmAs ~/Downloads
```
```shell
pls -f -m https://www.youtube.com/watch?v=iWDOO1vXmAs ~/Downloads
```
```shell
pls --update
```
