use clap::Parser;
use colored::*;
use dirs::home_dir;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::io::{self};
use std::path::Path;
use url::Url;

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

fn double_force() -> String {
    let illegal = "yes".to_string();
    println!(
        "{} {}",
        "Error:".red().bold(),
        "-f/--force used twice!".white()
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

fn file_check_go(url: String, out: String, force: String) -> io::Result<()> {
    if Path::new(&out).is_file() && Path::new(&out).exists() {
        if force == "yes" {
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
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.command.is_empty() {
        help();
    } else {
        let cmd = cli.command;
        let mut url = "empty".to_string();
        let mut out = "empty".to_string();
        let mut force = "no".to_string();
        let mut illegal = "no".to_string();
        let system = detect_os();
        let version = "0.1.3".to_string();

        if cmd != "-f" && cmd != "--force" {
            url = cmd;
        } else {
            force = "yes".to_string();
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
        } else {
            for (i, arg) in cli.args.iter().enumerate() {
                if i == 0 {
                    if force == "yes" {
                        if arg == "-f" || arg == "--force" {
                            illegal = double_force();
                        } else {
                            url = arg.clone();
                        }
                    } else if arg == "-f" || arg == "--force" {
                        if force == "yes" {
                            illegal = double_force();
                        } else {
                            force = "yes".to_string();
                        }
                    } else {
                        out = arg.clone();
                    }
                } else if i == 1 {
                    if force == "yes" {
                        if arg == "-f" || arg == "--force" {
                            illegal = double_force();
                        } else {
                            out = arg.clone();
                        }
                    } else if arg == "-f" || arg == "--force" {
                        if force == "yes" {
                            illegal = double_force();
                        } else {
                            force = "yes".to_string();
                        }
                    } else {
                        illegal = "yes".to_string();
                        println!("{} {}", "Error:".red().bold(), "bad arguments!".white());
                    }
                } else {
                    illegal = "yes".to_string();
                    println!(
                        "{} {} {}",
                        "Error:".red().bold(),
                        "More than three arguments".white(),
                        "are not allowed!".red().bold()
                    );
                }
            }

            if url == "empty" {
                illegal = "yes".to_string();
                println!("{} {}", "Error".red().bold(), "No URL specified".white());
            }

            if illegal == "no" {
                if let Some(file_name) = get_file_name_from_url(&url) {
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
                                if Path::new(&out).is_file() {
                                    if force == "yes" {
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
                            } else if Path::new(&out).is_dir() {
                                if out.ends_with('/') {
                                    out = remove_slash(&out);
                                }
                                if out.ends_with('\\') && system == "win" {
                                    out = remove_backslash(&out);
                                }

                                out = Path::new(&out)
                                    .join(file_name)
                                    .to_string_lossy()
                                    .to_string();

                                file_check_go(url, out, force)?;
                            }
                        } else {
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
                        }
                    } else {
                        println!(
                            "{} {}",
                            "Error:".red().bold(),
                            "Unable to determine home directory!".white()
                        );
                    }
                } else {
                    println!(
                        "{} {}",
                        "Error:".red().bold(),
                        "No file name found in the URL!".white()
                    );
                }
            }
        }
    }

    Ok(())
}
