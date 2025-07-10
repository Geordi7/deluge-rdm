# deluge-rdm

Deluge Relational Data Management: More than a drizzle!

This crate provides a small interpreter for the **deluge** language. Source
files use the `.del` extension.

```
SOI
a(b,c),
d(e(f,g),hij)
EOI
```

Usage:

```bash
# run and execute a file
cargo run -- path/to/file.del

# check only
cargo run -- --check path/to/file.del

# start language server
cargo run -- --lsp
```
