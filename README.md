![Build](https://github.com/untoldwind/acari/workflows/Build/badge.svg)

# Command-line client for mite time-tracking

## Requirements

Rust >=1.41.0

## Installation

Simple way
```
cargo install acari-cli
```

Alternative: Checkout the project and do
```
cargo install --path cli --force --locked
```

## Shell completions

### Fish

Copy or symlink `cli/shell_completions/acari.fish` to `~/.config/fish/completions`.

### Bash/Zsh

Right now there are two options:
* Migrate to the more colorful side of `fish`
* Contribute your own completions. I will gladly accept pull requests.

## Basic usage

### Initialization

You need to get an API token from mite. This can be found on your account page. Then you just have to do
```
acari init
```
which will ask for your mite domain and token.

Alternatively you simply create a `~/.config/acari/config.toml`
```
domain = '<your-company>.mite.yo.lk'
token = '<your-token>'
cache_ttl_minutes = 1440
```

After this you may check your connection with
```
acari check
```
with will print your account and user information.

### Query customers/projects/services

List customers
```
acari customers
```

List projects
```
acari projects
```
or if you are interested in the projects of a specific customer
```
acari projects "<customer-name>"
```

List services
```
acari services
```

All of these information will be cached. You can modify the cache duration in your `~/.config/acari/config.toml` (default: 1 day).

If you think that something is missing you can try running the above commands with the `--no-cache` option, or run
```
acari clear-cache
```
or simple erase the `~/.cache/acari` directory.

### Query time entries

```
acari entries <timespan>
```

Whereas `timespan` can be
* `today` or `now`: Just today
* `yeasterday`: Just yesterday
* `this-week` or `week`: All entries of this week
* `last-week`: all entries of last week
* `this-month` or `month`: all entries of this month
* `last-month`: all entries of last month
* `YYYY-MM-DD`: entries of a specific day
* `YYYY-MM-DD/YYYY-MM-DD`: all entries from a specific day up to another day

### Tracking time

Start time-tracking
```
acari start <customer-name> <project-name> <service-name>
```
optionally you can also add a starting offset
```
acari start <customer-name> <project-name> <service-name> <minutes>
```
whereas minutes are either actual minutes or in the form of `hh:mm`.

Stop time-tracking
```
acari stop
```

Show current time tracking
```
acari tracking
```
(Note: This might change, have not found a good naming yet)

### Modify time entries

Chance an entry for today
```
acari set <customer-name> <project-name> <service-name> <minutes>
```
whereas minutes are either actual minutes or in the form of `hh:mm`.

Chany an entry for a specific day
```
acari set <customer-name> <project-name> <service-name> <minutes> <date>
```

Whereas `date` can be
* `today` or `now`: Change today
* `yeasterday`: Change yeasterday
* `YYYY-MM-DD`: Change a specific day

**Note**: If the specified day contains multiple entries for the same cuatomer, project and service these will be squisched down to a single entry. The idea of the set command is to just set the time spend on a task for the day entirely.

### Modify the output

The output of all commandy can be modified via the `--output` or `-o` option. E.g.
```
acari --output=json customers
```

Supported output formats are:
* `pretty`: Show pretty tables (this is the default)
* `json`: Dump all available information as json
* `flat`: Very condensed form of `pretty` that may be helpful processing information in shell-scripts or `awk`
