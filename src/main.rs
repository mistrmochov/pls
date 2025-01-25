use clap::Parser;
use colored::*;
use dirs::home_dir;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::fs::File;
use std::fs::{self, Permissions};
use std::io::copy;
use std::io::BufReader;
use std::io::{self};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};
#[cfg(unix)]
use tar::Archive;
use url::Url;
#[cfg(unix)]
use xz2::read::XzDecoder;
#[cfg(windows)]
use zip::read::ZipArchive;

#[derive(Parser)]
#[command(disable_help_flag = true)]
#[command(allow_hyphen_values = true)]
struct Cli {
    #[arg(default_value_t = String::new())]
    command: String,
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

#[cfg(windows)]
fn detect_os() -> String {
    let system = "win".to_string();
    system
}

#[cfg(unix)]
fn detect_os() -> String {
    let system = "unix".to_string();
    system
}

fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP client and send the GET request
    let client = Client::new();
    let mut response = client.get(url).send()?;

    // Get the total size of the file (if available)
    let total_size = response.content_length().unwrap_or(0);

    // Create and configure the progress bar
    let pb = ProgressBar::new(total_size);
    let pb_style_first = "{spinner:.blue} {msg}\n|{elapsed_precise}|";
    let pb_style_line = format!("{}", "|".blue());
    let pb_style_mid = "{bar:40.blue}";
    let pb_style_end = "{percent}% | {bytes}/{total_bytes} | ETA: {eta}";
    let pb_style = format!(
        "{} {}{}{} {}",
        pb_style_first, pb_style_line, pb_style_mid, pb_style_line, pb_style_end
    );
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&pb_style)
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    let dw_file_name = format!("{} {}", "Downloading:".white(), url.blue().bold());
    pb.set_message(dw_file_name);

    // Open the output file
    let mut file = File::create(output_path)?;

    // Wrap the file writer to update the progress bar
    let mut writer = pb.wrap_write(&mut file);

    // Copy the response to the file in chunks
    copy(&mut response, &mut writer)?;

    let finish_mes = format!("{}", "Download complete!".white());
    pb.finish_with_message(finish_mes);
    println!(
        "{} {}",
        "\nFile saved to:".white(),
        output_path.blue().bold()
    );

    Ok(())
}

fn go(url: String, out: String) -> io::Result<()> {
    match download_file(&url, &out) {
        Ok(_) => {}
        Err(e) => {
            println!("{} {}", "Error".red().bold(), e.to_string().white());
            return Ok(());
        }
    }
    Ok(())
}

fn double_force() -> bool {
    let illegal = true;
    println!(
        "{} {}",
        "Error:".red().bold(),
        "-f/--force used twice!".white()
    );
    illegal
}

fn double_media() -> bool {
    let illegal = true;
    println!(
        "{} {}",
        "Error:".red().bold(),
        "-m/--media used twice!".white()
    );
    illegal
}

fn get_file_name_from_url(url_str: &str) -> Option<String> {
    let url = Url::parse(url_str).ok()?;
    url.path_segments()?.last().map(|s| s.to_string())
}

fn remove_slash(out: &str) -> String {
    out.trim_end_matches('/').to_string()
}

fn remove_slash_start(out: &str) -> String {
    out.trim_start_matches('/').to_string()
}

fn remove_backslash(out: &str) -> String {
    out.trim_end_matches('\\').to_string()
}

fn remove_backslash_start(out: &str) -> String {
    out.trim_start_matches('\\').to_string()
}

fn remove_tilde(out: &str) -> String {
    out.trim_start_matches('~').to_string()
}

fn get_dir_from_path(path: &str) -> String {
    let path = Path::new(path);
    path.parent()
        .map(|parent| parent.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string()) // Default to current directory
}

fn file_check_go(url: String, out: String, force: bool) -> io::Result<()> {
    if Path::new(&out).is_file() && Path::new(&out).exists() {
        if force == true {
            let mes = format!(
                "{} {}",
                "Error:".red().bold(),
                "Failed to remove file!".white()
            );
            fs::remove_file(out.clone()).expect(&mes);
            go(url, out)?;
        } else {
            println!(
                "{} {}",
                "Error:".red().bold(),
                "File already exists!".white()
            );
        }
    } else {
        go(url, out)?;
    }
    Ok(())
}

fn help() {
    println!("{}", "NAME".white().bold());
    println!(
        "{}",
        "       pls - Cli downloader written in blazingly fast rust!".white()
    );
    println!("{}", "\nSYNOPSIS".white().bold());
    println!(
        "{} {}",
        "       pls".white().bold(),
        "(options) [URL] [OUTPUT]".white()
    );
    println!(
        "{} {}",
        "       pls".white().bold(),
        "[URL] (options) [OUTPUT]".white()
    );
    println!(
        "{} {}",
        "       pls".white().bold(),
        "[URL] [OUTPUT] (options)".white()
    );
    println!("{}", "\nOPTIONS".white().bold());
    println!("{}", "       The following options are available:".white());
    println!(
        "{} {} {}",
        "\n       -f".white().bold(),
        "or".white(),
        "--force".white().bold()
    );
    println!(
        "{}",
        "       This option allows overwriting files. URL and OUTPUT must be present, while choosing this option.".white()
    );
    println!(
        "{} {} {}",
        "\n       -m".white().bold(),
        "or".white(),
        "--media".white().bold()
    );
    println!(
        "{}",
        "       This option lets you download videos from YouTube or any other site. URL and OUTPUT must be present, while choosing this option. Can be combined with -f/--force".white()
    );
    println!(
        "{} {} {}",
        "\n       -v".white().bold(),
        "or".white(),
        "--version".white().bold()
    );
    println!(
        "{}",
        "       This option prints version of the program. Must be passed alone.".white()
    );
    println!(
        "{} {} {}",
        "\n       -h".white().bold(),
        "or".white(),
        "--help".white().bold()
    );
    println!(
        "{}",
        "       This option prints help page of the program. Must be passed alone.".white()
    );
    println!(
        "{} {} {}",
        "\n       -u".white().bold(),
        "or".white(),
        "--update".white().bold()
    );
    println!(
        "{}",
        "       This option force updates yt-dlp and ffmpeg binaries. Must be passed alone."
            .white()
    );
}

#[cfg(unix)]
fn extract_tar_xz(file_path: &str, output_dir: &str) -> std::io::Result<()> {
    // Open the .tar.xz file
    let file = File::open(file_path)?;
    let decompressor = XzDecoder::new(BufReader::new(file));

    // Create the tar archive from the decompressed data
    let mut archive = Archive::new(decompressor);

    // Extract the archive to the specified output directory
    archive.unpack(output_dir)?;
    Ok(())
}

#[cfg(windows)]
fn extract_zip(zip_path: &str, output_dir: &str) -> io::Result<()> {
    // Open the zip file
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    // Iterate through the files in the zip archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = std::path::Path::new(output_dir).join(file.name());

        // Create directories if necessary
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            // Write the file to the output directory
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

#[cfg(unix)]
fn run_command_interactive(command: &str) -> io::Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command) // Pass the full command as a string
        .stdout(Stdio::inherit()) // Inherit stdout for real-time output
        .stderr(Stdio::inherit()) // Inherit stderr for real-time error output
        .stdin(Stdio::inherit()) // Inherit stdin for interactivity
        .spawn()?; // Spawn the process

    child.wait()?; // Wait for the process to finish
    Ok(())
}

#[cfg(windows)]
fn run_command_interactive(command: &str) -> io::Result<()> {
    // Launch the command in a Windows shell
    let mut child = Command::new("cmd")
        .args(["/C", command]) // Use cmd.exe and pass the command
        .stdout(Stdio::inherit()) // Inherit stdout to show output in real-time
        .stderr(Stdio::inherit()) // Inherit stderr to show error output in real-time
        .stdin(Stdio::inherit()) // Inherit stdin for interactivity
        .spawn()?; // Spawn the process

    // Wait for the process to complete
    child.wait()?;
    Ok(())
}

#[cfg(unix)]
fn ytdlp_check(update: bool) -> io::Result<()> {
    if let Some(home) = home_dir() {
        let architecture = std::env::consts::ARCH;
        let libs = home.join(".local/share/pls/libs");
        let ytdlp_bin = libs.join("yt-dlp");
        let ytdlp_url;
        let mut ytdlp_zip = String::new();
        if PathBuf::from(home.join(".termux")).exists()
            && PathBuf::from(home.join(".termux")).is_dir()
        {
            if architecture == "aarch64" {
                ytdlp_url = "https://storage.googleapis.com/mochov-public/pls/aarch64/yt-dlp-aarch64.tar.xz".to_string();
                ytdlp_zip = "/data/data/com.termux/files/usr/tmp/yt-dlp-aarch64.tar.xz".to_string();
            } else {
                ytdlp_url =
                    "https://storage.googleapis.com/mochov-public/pls/amd64/yt-dlp-amd64.tar.xz"
                        .to_string();
                ytdlp_zip = "/data/data/com.termux/files/usr/tmp/yt-dlp-amd64.tar.xz".to_string();
            }
        } else {
            if architecture == "aarch64" {
                ytdlp_url =
                            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux_aarch64"
                                .to_string();
            } else {
                ytdlp_url =
                    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux"
                        .to_string();
            }
        }

        let ffmpeg_bin = libs.join("ffmpeg");
        let ffmpeg_zip;
        if PathBuf::from(home.join(".termux")).exists()
            && PathBuf::from(home.join(".termux")).is_dir()
        {
            if architecture == "aarch64" {
                ffmpeg_zip = PathBuf::from("/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linuxarm64-gpl.tar.xz")
            } else {
                ffmpeg_zip = PathBuf::from(
                    "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linux64-gpl.tar.xz",
                );
            }
        } else {
            if architecture == "aarch64" {
                ffmpeg_zip = PathBuf::from("/tmp/ffmpeg-master-latest-linuxarm64-gpl.tar.xz");
            } else {
                ffmpeg_zip = PathBuf::from("/tmp/ffmpeg-master-latest-linux64-gpl.tar.xz");
            }
        }

        let ffmpeg_url;
        if architecture == "aarch64" {
            ffmpeg_url = "https://storage.googleapis.com/mochov-public/pls/aarch64/ffmpeg-master-latest-linuxarm64-gpl.tar.xz".to_string();
        } else {
            ffmpeg_url =
        "https://storage.googleapis.com/mochov-public/pls/amd64/ffmpeg-master-latest-linux64-gpl.tar.xz".to_string();
        }

        if !libs.exists() {
            std::fs::create_dir_all(&libs)?;
        }
        if !ytdlp_bin.exists() {
            println!("{} {}", "Installing".white(), "yt-dlp".blue().bold());
            if PathBuf::from(home.join(".termux")).exists()
                && PathBuf::from(home.join(".termux")).is_dir()
            {
                go(ytdlp_url, ytdlp_zip.clone())?;
                extract_tar_xz(&ytdlp_zip, &libs.to_string_lossy())?;
            } else {
                go(ytdlp_url, ytdlp_bin.to_string_lossy().to_string())?;
            }
            fs::set_permissions(&ytdlp_bin, Permissions::from_mode(0o755))?;

            if !ytdlp_bin.exists() {
                println!(
                    "{} {} {} {}",
                    "Error:".red().bold(),
                    "Failed".white().bold(),
                    "to install".white(),
                    "yt-dlp".blue().bold()
                );
                return Ok(());
            } else {
                println!(
                    "{} {} {}",
                    "yt-dlp".blue().bold(),
                    "has been successfully".white(),
                    "installed.".white().bold()
                );
            }
        } else {
            if update == true {
                fs::remove_file(&ytdlp_bin)?;
                println!("{} {}", "Updating".white(), "yt-dlp".blue().bold());
                if PathBuf::from(home.join(".termux")).exists()
                    && PathBuf::from(home.join(".termux")).is_dir()
                {
                    go(ytdlp_url, ytdlp_zip.clone())?;
                    extract_tar_xz(&ytdlp_zip, &libs.to_string_lossy())?;
                } else {
                    go(ytdlp_url, ytdlp_bin.to_string_lossy().to_string())?;
                    fs::set_permissions(&ytdlp_bin, Permissions::from_mode(0o755))?;
                }

                if !ytdlp_bin.exists() {
                    println!(
                        "{} {} {} {}",
                        "Error:".red().bold(),
                        "Failed".white().bold(),
                        "to install".white(),
                        "yt-dlp".blue().bold()
                    );
                    return Ok(());
                } else {
                    println!(
                        "{} {} {}",
                        "yt-dlp".blue().bold(),
                        "has been successfully".white(),
                        "installed.".white().bold()
                    );
                }
            }
        }
        if !ffmpeg_bin.exists() {
            println!("{} {}", "Installing".white(), "ffmpeg".blue().bold());
            go(ffmpeg_url, ffmpeg_zip.to_string_lossy().to_string())?;
            if PathBuf::from(home.join(".termux")).exists()
                && PathBuf::from(home.join(".termux")).is_dir()
            {
                extract_tar_xz(
                    &ffmpeg_zip.to_string_lossy(),
                    "/data/data/com.termux/files/usr/tmp",
                )?;
            } else {
                extract_tar_xz(&ffmpeg_zip.to_string_lossy(), "/tmp")?;
            }
            fs::remove_file(ffmpeg_zip)?;
            let source;
            if PathBuf::from(home.join(".termux")).exists()
                && PathBuf::from(home.join(".termux")).is_dir()
            {
                if architecture == "aarch64" {
                    source = "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linuxarm64-gpl/bin/ffmpeg";
                } else {
                    source = "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linux64-gpl/bin/ffmpeg";
                }
            } else {
                if architecture == "aarch64" {
                    source = "/tmp/ffmpeg-master-latest-linuxarm64-gpl/bin/ffmpeg"
                } else {
                    source = "/tmp/ffmpeg-master-latest-linux64-gpl/bin/ffmpeg";
                }
            }
            fs::copy(source, &ffmpeg_bin)?;
            if PathBuf::from(home.join(".termux")).exists()
                && PathBuf::from(home.join(".termux")).is_dir()
            {
                if architecture == "aarch64" {
                    fs::remove_dir_all(
                        "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linuxarm64-gpl",
                    )?;
                } else {
                    fs::remove_dir_all(
                        "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linux64-gpl",
                    )?;
                }
            } else {
                if architecture == "aarch64" {
                    fs::remove_dir_all("/tmp/ffmpeg-master-latest-linuxarm64-gpl")?;
                } else {
                    fs::remove_dir_all("/tmp/ffmpeg-master-latest-linux64-gpl")?;
                }
            }
            fs::set_permissions(&ffmpeg_bin, Permissions::from_mode(0o755))?;

            if !ffmpeg_bin.exists() {
                println!(
                    "{} {} {} {}",
                    "Error:".red().bold(),
                    "Failed".white().bold(),
                    "to install".white(),
                    "ffmpeg".blue().bold()
                );
                return Ok(());
            } else {
                println!(
                    "{} {} {}",
                    "ffmpeg".blue().bold(),
                    "has been successfully".white(),
                    "installed.".white().bold()
                );
            }
        } else {
            if update == true {
                fs::remove_file(&ffmpeg_bin)?;
                println!("{} {}", "Updating".white(), "ffmpeg".blue().bold());
                go(ffmpeg_url, ffmpeg_zip.to_string_lossy().to_string())?;
                if PathBuf::from(home.join(".termux")).exists()
                    && PathBuf::from(home.join(".termux")).is_dir()
                {
                    extract_tar_xz(
                        &ffmpeg_zip.to_string_lossy(),
                        "/data/data/com.termux/files/usr/tmp",
                    )?;
                } else {
                    extract_tar_xz(&ffmpeg_zip.to_string_lossy(), "/tmp")?;
                }
                fs::remove_file(ffmpeg_zip)?;
                let source;
                if PathBuf::from(home.join(".termux")).exists()
                    && PathBuf::from(home.join(".termux")).is_dir()
                {
                    if architecture == "aarch64" {
                        source = "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linuxarm64-gpl/bin/ffmpeg";
                    } else {
                        source = "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linux64-gpl/bin/ffmpeg";
                    }
                } else {
                    if architecture == "aarch64" {
                        source = "/tmp/ffmpeg-master-latest-linuxarm64-gpl/bin/ffmpeg"
                    } else {
                        source = "/tmp/ffmpeg-master-latest-linux64-gpl/bin/ffmpeg";
                    }
                }
                fs::copy(source, &ffmpeg_bin)?;
                if PathBuf::from(home.join(".termux")).exists()
                    && PathBuf::from(home.join(".termux")).is_dir()
                {
                    if architecture == "aarch64" {
                        fs::remove_dir_all(
                        "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linuxarm64-gpl",
                    )?;
                    } else {
                        fs::remove_dir_all(
                            "/data/data/com.termux/files/usr/tmp/ffmpeg-master-latest-linux64-gpl",
                        )?;
                    }
                } else {
                    if architecture == "aarch64" {
                        fs::remove_dir_all("/tmp/ffmpeg-master-latest-linuxarm64-gpl")?;
                    } else {
                        fs::remove_dir_all("/tmp/ffmpeg-master-latest-linux64-gpl")?;
                    }
                }
                fs::set_permissions(&ffmpeg_bin, Permissions::from_mode(0o755))?;

                if !ffmpeg_bin.exists() {
                    println!(
                        "{} {} {} {}",
                        "Error:".red().bold(),
                        "Failed".white().bold(),
                        "to install".white(),
                        "ffmpeg".blue().bold()
                    );
                    return Ok(());
                } else {
                    println!(
                        "{} {} {}",
                        "ffmpeg".blue().bold(),
                        "has been successfully".white(),
                        "installed.".white().bold()
                    );
                }
            }
        }
    }

    Ok(())
}

#[cfg(windows)]
fn ytdlp_check(update: bool) -> io::Result<()> {
    if let Some(home) = home_dir() {
        let libs = home.join("AppData\\Roaming\\pls\\libs");
        let ytdlp_bin = libs.join("yt-dlp.exe");
        let ytdlp_url =
            "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe".to_string();
        let ffmpeg_bin = libs.join("ffmpeg.exe");
        let ffmpeg_zip = home.join("AppData\\Local\\Temp\\ffmpeg-release-essentials.zip");
        let ffmpeg_url =
            "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip".to_string();

        if !libs.exists() {
            std::fs::create_dir_all(libs)?;
        }
        if !ytdlp_bin.exists() {
            println!("{} {}", "Installing".white(), "yt-dlp".blue().bold());
            go(ytdlp_url, ytdlp_bin.to_string_lossy().to_string())?;

            if !ytdlp_bin.exists() {
                println!(
                    "{} {} {} {}",
                    "Error:".red().bold(),
                    "Failed".white().bold(),
                    "to install".white(),
                    "yt-dlp".blue().bold()
                );
                return Ok(());
            } else {
                println!(
                    "{} {} {}",
                    "yt-dlp".blue().bold(),
                    "has been successfully".white(),
                    "installed.".white().bold()
                );
            }
        } else {
            if update == true {
                fs::remove_file(&ytdlp_bin)?;
                println!("{} {}", "Updating".white(), "yt-dlp".blue().bold());
                go(ytdlp_url, ytdlp_bin.to_string_lossy().to_string())?;

                if !ytdlp_bin.exists() {
                    println!(
                        "{} {} {} {}",
                        "Error:".red().bold(),
                        "Failed".white().bold(),
                        "to install".white(),
                        "yt-dlp".blue().bold()
                    );
                    return Ok(());
                } else {
                    println!(
                        "{} {} {}",
                        "yt-dlp".blue().bold(),
                        "has been successfully".white(),
                        "installed.".white().bold()
                    );
                }
            }
        }
        if !ffmpeg_bin.exists() {
            println!("{} {}", "Installing".white(), "ffmpeg".blue().bold());
            go(ffmpeg_url, ffmpeg_zip.to_string_lossy().to_string())?;
            extract_zip(
                &ffmpeg_zip.to_string_lossy(),
                &home.join("AppData\\Local\\Temp").to_string_lossy(),
            )?;
            fs::remove_file(ffmpeg_zip)?;
            let source = "AppData\\Local\\Temp\\ffmpeg-7.1-essentials_build\\bin\\ffmpeg.exe";
            fs::copy(source, &ffmpeg_bin)?;
            fs::remove_dir_all(home.join("AppData\\Local\\Temp\\ffmpeg-7.1-essentials_build"))?;

            if !ffmpeg_bin.exists() {
                println!(
                    "{} {} {} {}",
                    "Error:".red().bold(),
                    "Failed".white().bold(),
                    "to install".white(),
                    "ffmpeg".blue().bold()
                );
                return Ok(());
            } else {
                println!(
                    "{} {} {}",
                    "ffmpeg".blue().bold(),
                    "has been successfully".white(),
                    "installed.".white().bold()
                );
            }
        } else {
            if update == true {
                println!("{} {}", "Updating".white(), "ffmpeg".blue().bold());
                fs::remove_file(&ffmpeg_bin)?;
                go(ffmpeg_url, ffmpeg_zip.to_string_lossy().to_string())?;
                extract_zip(
                    &ffmpeg_zip.to_string_lossy(),
                    &home.join("AppData\\Local\\Temp").to_string_lossy(),
                )?;
                fs::remove_file(ffmpeg_zip)?;
                let source = "AppData\\Local\\Temp\\ffmpeg-7.1-essentials_build\\bin\\ffmpeg.exe";
                fs::copy(source, &ffmpeg_bin)?;
                fs::remove_dir_all(home.join("AppData\\Local\\Temp\\ffmpeg-7.1-essentials_build"))?;

                if !ffmpeg_bin.exists() {
                    println!(
                        "{} {} {} {}",
                        "Error:".red().bold(),
                        "Failed".white().bold(),
                        "to install".white(),
                        "ffmpeg".blue().bold()
                    );
                    return Ok(());
                } else {
                    println!(
                        "{} {} {}",
                        "ffmpeg".blue().bold(),
                        "has been successfully".white(),
                        "installed.".white().bold()
                    );
                }
            }
        }
    }

    Ok(())
}

fn ytdlp_go(system: String, url: String, out: String, force: bool) -> io::Result<()> {
    if let Some(home) = home_dir() {
        if system == "unix" {
            let libs = home.join(".local/share/pls/libs");
            let ytdlp_bin = libs.join("yt-dlp");
            let ffmpeg_bin = libs.join("ffmpeg");
            if force == true {
                let dw = format!(
                    "{} {} --force-overwrites --ffmpeg-location {} -P {}",
                    ytdlp_bin.to_string_lossy(),
                    url,
                    ffmpeg_bin.to_string_lossy(),
                    out
                );
                run_command_interactive(&dw)?;
            } else {
                let dw = format!(
                    "{} {} --no-overwrites --ffmpeg-location {} -P {}",
                    ytdlp_bin.to_string_lossy(),
                    url,
                    ffmpeg_bin.to_string_lossy(),
                    out
                );
                run_command_interactive(&dw)?;
            }
        } else {
            let libs = home.join("AppData\\Roaming\\pls\\libs");
            let ytdlp_bin = libs.join("yt-dlp.exe");
            let ffmpeg_bin = libs.join("ffmpeg.exe");
            if force == true {
                let dw = format!(
                    "{} {} --force-overwrites --ffmpeg-location {} -P {}",
                    ytdlp_bin.to_string_lossy(),
                    url,
                    ffmpeg_bin.to_string_lossy(),
                    out
                );
                run_command_interactive(&dw)?;
            } else {
                let dw = format!(
                    "{} {} --no-overwrites --ffmpeg-location {} -P {}",
                    ytdlp_bin.to_string_lossy(),
                    url,
                    ffmpeg_bin.to_string_lossy(),
                    out
                );
                run_command_interactive(&dw)?;
            }
        }
    } else {
        println!(
            "{} {} {}",
            "Error:".red().bold(),
            "Failed".white().bold(),
            "to determine home directory.".white()
        );
        return Ok(());
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.command.is_empty() {
        help();
    } else {
        let cmd = cli.command;
        let mut url = "empty".to_string();
        let mut out = "empty".to_string();
        let mut force = false;
        let mut media = false;
        let mut illegal = false;
        let mut update = false;
        let mut file_name = String::new();
        let system = detect_os();
        let version = "0.1.3".to_string();

        if cmd != "-f" && cmd != "--force" {
            if cmd == "-m" || cmd == "--media" {
                media = true;
            } else {
                url = cmd;
            }
        } else {
            force = true;
        }

        if url == "--version" || url == "-v" {
            println!(
                "{} {} {}",
                "pls,".white().bold(),
                "version -".white().bold(),
                version.blue().bold()
            );
        } else if url == "--help" || url == "-h" {
            help();
        } else if url == "--update" || url == "-u" {
            update = true;
            ytdlp_check(update)?;
        } else {
            for (i, arg) in cli.args.iter().enumerate() {
                if i == 0 {
                    if force == true {
                        if arg == "-f" || arg == "--force" {
                            illegal = double_force();
                        } else if arg == "-m" || arg == "--media" {
                            media = true;
                        } else {
                            url = arg.clone();
                        }
                    } else if media == true {
                        if arg == "-m" || arg == "--media" {
                            illegal = double_media();
                        } else if arg == "-f" || arg == "--force" {
                            force = true;
                        } else {
                            url = arg.clone();
                        }
                    } else if arg == "-f" || arg == "--force" {
                        force = true;
                    } else if arg == "-m" || arg == "--media" {
                        media = true;
                    } else {
                        out = arg.clone();
                    }
                } else if i == 1 {
                    if force == true {
                        if arg == "-f" || arg == "--force" {
                            illegal = double_force();
                        } else {
                            if media == true {
                                if arg == "-m" || arg == "--media" {
                                    illegal = double_media();
                                } else {
                                    url = arg.clone();
                                }
                            } else {
                                if arg == "-m" || arg == "--media" {
                                    media = true;
                                } else {
                                    out = arg.clone();
                                }
                            }
                        }
                    } else if media == true {
                        if arg == "-m" || arg == "--media" {
                            illegal = double_force();
                        } else {
                            if arg == "-f" || arg == "--force" {
                                force = true;
                            } else {
                                out = arg.clone();
                            }
                        }
                    } else if arg == "-f" || arg == "--force" {
                        force = true;
                    } else if arg == "-m" || arg == "--media" {
                        media = true;
                    } else {
                        println!("{} {}", "Error:".red().bold(), "bad arguments!".white());
                        illegal = true;
                    }
                } else if i == 2 {
                    if arg == "-f" || arg == "--force" {
                        if force == true {
                            illegal = double_force();
                        } else {
                            force = true;
                        }
                    } else if arg == "-m" || arg == "--media" {
                        if media == true {
                            illegal = double_media();
                        } else {
                            media = true;
                        }
                    } else {
                        if out == "empty" {
                            out = arg.clone();
                        } else {
                            println!("{} {}", "Error:".red().bold(), "bad arguments!".white());
                            illegal = true;
                        }
                    }
                } else {
                    println!(
                        "{} {} {}",
                        "Error:".red().bold(),
                        "More than four arguments".white(),
                        "are not allowed!".red().bold()
                    );
                    illegal = true;
                }
            }

            if url == "empty" {
                println!("{} {}", "Error".red().bold(), "No URL specified".white());
                illegal = true;
            }

            if illegal == false {
                if media == false {
                    if let Some(fl_name) = get_file_name_from_url(&url) {
                        file_name = fl_name;
                    } else {
                        println!(
                            "{} {}",
                            "Error:".red().bold(),
                            "No file name found in the URL!".white()
                        );
                    }
                } else {
                    ytdlp_check(update)?;
                }
                if let Some(home) = home_dir() {
                    if out.starts_with('~') && system == "win" {
                        out = remove_tilde(&out);
                        if out.starts_with('\\') {
                            out = remove_backslash_start(&out);
                            out = home.join(out.clone()).to_string_lossy().to_string();
                        } else if out.starts_with('/') {
                            out = remove_slash_start(&out);
                            out = home.join(out.clone()).to_string_lossy().to_string();
                        } else {
                            println!(
                                "{} {}",
                                "Error".red().bold(),
                                "Home directory written incorrectly!".white()
                            );
                            return Ok(());
                        }
                    }
                    if Path::new(&out).exists() {
                        if !Path::new(&out).is_dir() {
                            if media == true {
                                println!(
                                    "{} {} {} {}",
                                    "Error:".red().bold(),
                                    "You can't use".white(),
                                    "filename,".white().bold(),
                                    "when downloading media.".white()
                                );
                                return Ok(());
                            } else {
                                if Path::new(&out).is_file() {
                                    if force == true {
                                        let mes = format!(
                                            "{} {}",
                                            "Error:".red().bold(),
                                            "Failed to remove file!".white()
                                        );
                                        fs::remove_file(out.clone()).expect(&mes);
                                        go(url, out)?;
                                    } else {
                                        println!(
                                            "{} {}",
                                            "Error:".red().bold(),
                                            "File already exists!".white()
                                        );
                                    }
                                }
                            }
                        } else if Path::new(&out).is_dir() {
                            if out.ends_with('/') {
                                out = remove_slash(&out);
                            }
                            if out.ends_with('\\') && system == "win" {
                                out = remove_backslash(&out);
                            }

                            if media == true {
                                ytdlp_go(system, url, out, force)?;
                            } else {
                                out = Path::new(&out)
                                    .join(file_name)
                                    .to_string_lossy()
                                    .to_string();

                                file_check_go(url, out, force)?;
                            }
                        }
                    } else {
                        if media == false {
                            let out_bare = get_dir_from_path(&out);
                            if Path::new(&out_bare).exists() && Path::new(&out_bare).is_dir() {
                                if out_bare == "." {
                                    out = file_name;
                                    file_check_go(url, out, force)?;
                                } else {
                                    go(url, out)?;
                                }
                            } else if out == "empty" {
                                out = file_name;
                                file_check_go(url, out, force)?;
                            } else {
                                println!(
                                    "{} {}",
                                    "Error".red().bold(),
                                    "Output directory couldn't be found!".white()
                                );
                            }
                        } else {
                            if out == "empty" {
                                out = ".".to_string();
                                ytdlp_go(system, url, out, force)?;
                            } else {
                                println!(
                                    "{} {}",
                                    "Error".red().bold(),
                                    "Output directory couldn't be found!".white()
                                );
                            }
                        }
                    }
                } else {
                    println!(
                        "{} {}",
                        "Error:".red().bold(),
                        "Unable to determine home directory!".white()
                    );
                }
            }
        }
    }

    Ok(())
}
