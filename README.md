# OpenFGA DSL Parser

[![Telegraf crate](https://img.shields.io/crates/v/telegraf.svg)](https://crates.io/crates/openfga-dsl-parser)
[![Telegraf documentation](https://docs.rs/telegraf/badge.svg)](https://docs.rs/openfga-dsl-parser)

The OpenFGA DSL Parser provides utilities for parsing DSL file string inputs into an AST representation, as well as transforming them into a JSON representation.

This library is meant to be a utility for services using [OpenFGA](https://openfga.dev/) for their authorization solution, but need a way to translate from their DSL to the JSON format the HTTP API expects. A [Typescript library](https://github.com/openfga/syntax-transformer) is available that does something similar, however this library aims to target Rust as well as be a base for other potential languages to bind to.

# Usage

```rust
use openfga_dsl_parser::{json, Parser};

let input = "type group
  relations
    define member as self
type resource
  relations
    define writer as self
    define reader as self but not writer";

let mut parser = Parser::new(input);
let doc = parser.parse_document()?;

let json = json::JsonTransformer::new(&doc).serialize();
```
