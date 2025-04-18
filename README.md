# DNS Cli Tools

CLI tools for the DNS provider Cloudflare, useful tools to manage subdomains. In the future more DNS providers can be added.

## Features

- DDNS
- Registering new subdomains
- Listing subdomains
- Importing domains

## Installation

### Homebrew

```sh
brew tap hugo-persson/dns-cli-tools
brew install hugo-persson/dns-cli-tools/dns-cli-tools
```

#### Updating

```sh
brew upgrade dns-cli-tools

```

### Cargo

```sh
cargo install godaddy-cli-tools
```

### Build from source

TODO:

## Usage

Run

```sh
dns-cli help
```

to get a list of commands.

To get started first run

```sh
dns-cli init
```

This will create a config file with default config. By default the config path is places at `~/.config/dns-cli-config.json`, this can be overidden with the `-c` flag.

## Watching IP for changes

One of the main use cases of this program is watching the public IP of the device it runs on and updating DNS records if the public IP changes.

To register a domain run:

```sh
dns-cli cloudflare register mysubdomain.domain.com
```

and replcae `mysubdomain` with your subdomain.

Or you can import all domains currently pointing to this IP with:

```sh

dns-cli cloudflare import
```

This will create/update the record for this subdomain to point to the IP adress of the device command ran at and add the record to watch list.

To check for changes run

```sh
dns-cli check
```

This will check if the domain has changed since the command was last run. If it is the first time the command is run then domains will be refreshed to point to current IP.

The program keeps track of last IP in a file located at `~/.lastip.txt`

## Installing completions

To install completions for your shell run:

### Fish

```sh
mkdir -p ~/.local/share/fish/vendor_completions.d && dns-cli completion fish > ~/.local/share/fish/vendor_completions.d/dns-cli.fish && exec fish
```

## Troubleshooting

If any errors occur or similiar you can edit the config file manually, default location is `~/.config/dns-cli-config.json`

## TODO:

- [ ] Fix auto update brew tap
- [ ] Auto update cargo.toml version
- [ ] Smooth registering and deletion with inquirer
- [ ] Auto completions with homebrew
