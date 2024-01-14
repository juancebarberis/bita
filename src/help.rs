use crate::version::version;
use colored::Colorize;

pub(crate) fn help() {
    version();
    println!("A simple and fast command line tool to keep track of your daily events without leaving the terminal.");
    println!();
    println!("{}", "Usage:".green());
    println!("{}", "\tbita [COMMAND]".bold());
    println!();
    println!("{}", "Commands:".green());
    println!("{}", "\tn, new \"MESSAGE...\"\t\tCreate new entry");
    println!("{}", "\ts, show, all \t\t\tShow all entries");
    println!("{}", "\ta, all \t\t\t\tShow all entries");
    println!("{}", "\tg, get <ID, --latest|-l> \tShow entry by ID or pass the latest option to consider the newest entry");
    println!("{}", "\td, delete <ID, --latest|-l>\tDelete entry by ID or pass the latest option to consider the newest entry");
    println!();
    println!("{}", "\tc, config\t\t\tOpen configuration file");
    println!("{}", "\tv, version\t\t\tPrint version");
    println!("{}", "\th, help\t\t\t\tPrint help");
}