use crate::version::version;
use colored::Colorize;

pub(crate) fn help() {
    version();
    println!("A simple and fast command line tool to keep track of your daily events without leaving the terminal.");
    println!();
    println!("{}", "Usage:".bright_green());
    println!("{}", "\tbita [COMMAND]".bold());
    println!();
    println!("{}", "Commands:".bright_green());
    println!("\tn, new \"MESSAGE...\"\t\tCreate new entry");
    println!("\ta, all <--today|-t, --yesterday|-y>\t\t\tShow all entries. Optionally can filter by today or yesterday entries");
    println!("\tg, get <ID, --latest|-l> \tShow entry by ID or pass the latest option to consider the newest entry");
    println!("\td, delete <ID, --latest|-l>\tDelete entry by ID or pass the latest option to consider the newest entry");
    println!();
    println!("\tc, config\t\t\tOpen configuration file");
    println!("\tv, version\t\t\tPrint version");
    println!("\th, help\t\t\t\tPrint help");
}