# SITCON@GDSC – Review Tool

This tool can easily review the content on the SITCON@GDSC review table
in a standard way.

## Usage

### Native (release)

```bash
cargo run --release
```

### Browser (build & serve)

```bash
cargo install trunk
trunk build --release
bun x serve dist
```

## Development

### Native (debug)

```bash
cargo run
```

### Browser (serve)

```bash
cargo install trunk
trunk serve
```

## License

MIT License
