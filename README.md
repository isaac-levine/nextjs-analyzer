# nextjs-analyzer

A Rust-powered static analyzer that detects client-side React hooks in Next.js server components. Parses your component files using SWC and identifies optimization opportunities where code could be moved from client to server components.

## How It Works

The analyzer walks your Next.js project directory, parses each component file with the SWC parser, and traverses the AST to find:

- Components marked with `"use client"` that could be server components
- Data fetching patterns that belong on the server
- Hook usage that unnecessarily forces client-side rendering

## Usage

```bash
cargo run -- <path-to-nextjs-project>
```

## Example

```
$ cargo run -- ./my-nextjs-app

Analyzing: ./my-nextjs-app/src/components/Dashboard.tsx
  ⚠ Component uses "use client" but only uses useState for a simple toggle
    → Consider extracting the static content into a server component

Analyzing: ./my-nextjs-app/src/components/Profile.tsx
  ⚠ Data fetching detected in client component
    → Move fetch logic to a server component or server action
```

## Built With

- **Rust** with SWC (the parser that powers Next.js itself)
- `swc_ecma_parser` / `swc_ecma_ast` / `swc_ecma_visit` for AST parsing and traversal
- `walkdir` for recursive file discovery
- `colored` for terminal output

## Building from Source

```bash
cargo build --release
```

## License

MIT
