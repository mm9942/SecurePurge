mod file_remover;

use crate::file_remover::FileRemover;
use clap::{arg, ArgAction, Command};
use std::path::PathBuf;

fn cli() -> Command {
    Command::new("SecurePurge")
        .about("")
        .subcommand_required(true)
        .subcommand(
            Command::new("remove")
                .about("Removing a file by overwriting the data before deletion")
                .arg(arg!(-p --path <PATH> "File path of the file you want to delete").required(true))
                .arg(arg!(-t --times <TIMES> "File data overwriting times").required(true))
                .arg(arg!(-r --recursive <RECURSIVE> "Allow recursive deletion wich means, allowing the deletion of directorys").action(ArgAction::SetTrue))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("format")
                .about("Formatting a disk by overwriting the data before the formatting process itself")
                .arg(arg!(-p --path <PATH> "Device path of the disk you want to format").required(true))
                .arg(arg!(-t --times <TIMES> "File data overwriting times").required(true))
                .arg(arg!(-f --format <FORMAT> "Disk format with wich you want to format the disk (ntfs, exfat, fat32, btrfs, ext4, xfs)").required(true))
                .arg_required_else_help(true),
        )
}

fn command_handler() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("remove", sub_matches)) => {
            let path_str = sub_matches.get_one::<String>("path").expect("required");
            let times_str = sub_matches.get_one::<String>("times").expect("required");
            let recursive = sub_matches.get_flag("recursive");

            let file_path = PathBuf::from(path_str);
            let overwrite_times = times_str.parse::<u32>().expect("Times should be a number");

            match FileRemover::new(overwrite_times, file_path, recursive) {
                Ok(mut file_remover) => {
                    if let Err(e) = file_remover.delete() {
                        eprintln!("Error while deleting file: {}", e);
                    } else {
                        println!("File successfully deleted.");
                    }
                }
                Err(e) => eprintln!("Failed to initialize file remover: {}", e),
            }
        }
        _ => unreachable!("Exhaustive match not required as clap enforces subcommand usage"),
    }
}

fn main() {
    command_handler();
}
