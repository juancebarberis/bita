# Bita

A simple, fast and offline command line tool to keep track of your daily events without leaving the terminal.

(My first Rust adventure)

## Installation

Via curl
```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/juancebarberis/bita/master/tools/install.sh)"
```

Via wget
```bash
sh -c "$(wget https://raw.githubusercontent.com/juancebarberis/bita/master/tools/install.sh -O -)"
```

If you got some permission denied error, running the script with `sudo` may help.

**After installation success, write `bita` in your terminal, and it should display the all the help info.**

## Usage

Create new entries with the `new` command.
```bash
$ bita new "Helped Ariel with a database migration issue"

Entry recorded successfully f3e1e42d-197d-46ce-9d67-a62e0ecaa5f6
```

Retrieve them with the `all` command.
```bash
$ bita all

f3e1e42d-197d-46ce-9d67-a62e0ecaa5f6 (Timestamp: 2024-01-14T09:32:00-03:00)
Helped Ariel with a database migration issue

b8493fe4-3959-491f-b9fb-1ddff6a8faf2 (Timestamp: 2024-01-13T10:11:00-03:00)
Meeting with the finance team 💸
```

Read one entry with `get`.
```bash
$ bita get b8493fe4-3959-491f-b9fb-1ddff6a8faf2

b8493fe4-3959-491f-b9fb-1ddff6a8faf2 (Timestamp: 2024-01-13T10:11:00-03:00)
Meeting with the finance team 💸

$ bita get --latest

f3e1e42d-197d-46ce-9d67-a62e0ecaa5f6 (Timestamp: 2024-01-14T09:32:00-03:00)
Helped Ariel with a database migration issue
```

Delete an entry following the same idea.
```bash
$ bita delete f3e1e42d-197d-46ce-9d67-a62e0ecaa5f6

Entry deleted successfully f3e1e42d-197d-46ce-9d67-a62e0ecaa5f6
```

Or if you had a mistake, rapidly delete the latest entry.

```bash
$ bita delete --latest

Considering latest entry f3e1e42d-197d-46ce-9d67-a62e0ecaa5f6
Entry deleted successfully f3e1e42d-197d-46ce-9d67-a62e0ecaa5f6
```

For more info, check the `src/help.rs` file.