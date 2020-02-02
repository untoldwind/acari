# Command-line client for mite time-tracking

## Requirements

Rust >=1.40.0

## Installation

Right now this in a too early stage to be published on crates.io, so the best installation method is to check out this project and run
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
