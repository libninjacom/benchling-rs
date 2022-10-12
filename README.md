<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/benchling-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/benchling-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/benchling-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/benchling-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/benchling-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/benchling-rs/test?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/benchling">
    <img src="https://img.shields.io/crates/d/benchling?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/benchling">
    <img src="https://img.shields.io/crates/v/benchling?style=flat-square" alt="Crates.io" />
</a>

</p>

Benchling client, generated from the OpenAPI spec.

# Usage

```rust
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_aa_sequences()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .modified_at("your modified at")
        .name("your name")
        .name_includes("your name includes")
        .amino_acids("your amino acids")
        .folder_id("your folder id")
        .mentioned_in("your mentioned in")
        .project_id("your project id")
        .registry_id("your registry id")
        .schema_id("your schema id")
        .schema_fields(SchemaFieldsQueryParam {})
        .archive_reason("your archive reason")
        .mentions("your mentions")
        .ids("your ids")
        .entity_registry_ids_any_of("your entity registry ids.any of")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .creator_ids("your creator ids")
        .author_ids_any_of("your author ids.any of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:

* `BENCHLING_BASIC_API_KEY_AUTH`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
benchling = "0.1.0"
```


# Documentation


* [API Documentation](https://docs.benchling.com)


* [Client Library Documentation](https://docs.rs/benchling)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*