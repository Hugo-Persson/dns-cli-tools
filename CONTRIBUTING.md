# Contributing

## Testing

```sh
# Compile
cargo build
# Run
./target/debug/dns-cli
```

## Releasing

Run the release script:

```sh
./scripts/do-release.sh
```

This will

1. Bump the version in `Cargo.toml`
2. Create a git tag
3. Push the tag to the remote
4. Create a release on GitHub
5. Publish the Homebrew release
