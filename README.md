# Godaddy cli tools

CLI tools for the DNS provider GoDaddy, useful tools to manage subdomains.

## Features

- Watching for changes in public IP and updating DNS records
- Registering new subdomains
- Listing subdomains

## Installation

### Homebrew

```sh
brew tap hugo-persson/dns-cli-tools
brew install hugo-persson/dns-cli-tools/dns-cli-tools
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

This will create a config file with default config. By default the config path is places at `~/.config/godaddy-cli-tools.json`, this can be overidden with the `-c` flag.

A couple of things needs to be changed in this file to get started

- domain: Change the domain from `example.com` to the main domain you want to create subdomains from
- api_key: Your api key, see [[#getting-api-key]]
- secret: Your secret

## Watching IP for changes

One of the main use cases of this program is watching the public IP of the device it runs on and updating DNS records if the public IP changes.

To register a domain run:

```sh
godaddy register mysubdomain
```

and replcae `mysubdomain` with your subdomain.

This will create/update the record for this subdomain to point to the IP adress of the device command ran at and add the record to watch list.

To check for changes run

```sh
godaddy check
```

This will check if the domain has changed since the command was last run. If it is the first time the command is run then domains will be refreshed to point to current IP.

The program keeps track of last IP in a file located at `~/.lastip`

## Getting api key

1. Navigate to: https://developer.godaddy.com/keys
2. Create new `Production` key

## Cloudflare
