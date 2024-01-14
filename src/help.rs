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
    println!("{}", "\tn, new \"MESSAGE...\"\tCreate new entry");
    println!("{}", "\td, delete <ID>\t\tDelete entry");
    println!("{}", "\tc, config\t\tOpen configuration file");
    println!("{}", "\tv, version\t\tPrint version");
    println!("{}", "\th, help\t\t\tPrint help");
}