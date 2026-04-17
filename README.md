# Footprint Protocol

The OpenGraph Protocol for carbon footprints — embed `fp:` meta tags on any product page to expose standardized CO₂e data that tools, browsers, and aggregators can read automatically.

```html
<meta property="fp:product" content="Fairphone 5" />
<meta property="fp:co2e" content="23.6" />
<meta property="fp:co2e:unit" content="kg" />
<meta property="fp:scope" content="lifecycle" />
<meta property="fp:methodology" content="ISO 14067" />
```

## Properties

### Required

| Property | Description | Example |
|---|---|---|
| `fp:product` | Product name | `Fairphone 5` |
| `fp:co2e` | CO₂ equivalent value | `23.6` |
| `fp:co2e:unit` | Unit: `kg`, `g`, or `t` | `kg` |

### Recommended

| Property | Description | Example |
|---|---|---|
| `fp:scope` | `lifecycle`, `production`, `use`, or `disposal` | `lifecycle` |
| `fp:per` | Functional unit | `unit`, `year`, `km` |
| `fp:methodology` | Standard used | `ISO 14067`, `GHG Protocol` |
| `fp:certifier` | URL of certifying body | `https://cert.example.org` |
| `fp:verified:date` | ISO 8601 verification date | `2025-01-15` |

### Lifecycle breakdown (GHG Protocol phases)

| Property | Phase |
|---|---|
| `fp:materials` | Raw material extraction and processing |
| `fp:manufacturing` | Manufacturing and assembly |
| `fp:transport` | Transport and distribution |
| `fp:use` | Use phase (energy, consumables) |
| `fp:disposal` | End-of-life treatment |

Breakdown values use the same unit as `fp:co2e:unit`.

## This repository

```
footprint/
├── parser/   Rust crate — parse fp: tags from any HTML string or file
└── site/     Astro site — protocol documentation (Cloudflare Pages)
```

## Parser

```toml
# Cargo.toml
[dependencies]
footprint-parser = "0.1"
```

```rust
let data = footprint_parser::parse(html)?;
println!("{} emits {} {}", data.product, data.co2e, data.co2e_unit);
```

### CLI

```sh
cargo install footprint-parser
footprint product-page.html
```

```
Product:   Fairphone 5
CO2e:      23.6 kg
Scope:     lifecycle
Method:    ISO 14067
Verified:  2025-01-15

Lifecycle breakdown (kg):
  Materials:     8.2
  Manufacturing: 4.1
  Transport:     2.8
  Use:           6.9
  Disposal:      1.6
```

## License

MIT
