use std::collections::HashMap;

use scraper::{Html, Selector};

use crate::error::ParseError;
use crate::types::{Breakdown, Co2Unit, FootprintData, Scope};

/// Parse an HTML string and extract Footprint Protocol metadata.
///
/// # Errors
///
/// Returns `ParseError::MissingRequired` if any of the three required properties
/// (`fp:product`, `fp:co2e`, `fp:co2e:unit`) are absent.
///
/// Returns `ParseError::InvalidValue` if `fp:co2e` is not a valid decimal number
/// or `fp:co2e:unit` is not one of `kg`, `g`, or `t`.
pub fn parse(html: &str) -> Result<FootprintData, ParseError> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"meta[property]"#).map_err(|_| ParseError::HtmlParse)?;

    // Collect all fp: properties into a map (last one wins for duplicates).
    let mut map: HashMap<String, String> = HashMap::new();
    for el in document.select(&selector) {
        if let Some(property) = el.value().attr("property") {
            if property.starts_with("fp:") {
                if let Some(content) = el.value().attr("content") {
                    map.insert(property.to_string(), content.to_string());
                }
            }
        }
    }

    // ── Required fields ──────────────────────────────────────────────────

    let product = map
        .remove("fp:product")
        .ok_or(ParseError::MissingRequired("fp:product"))?;

    let co2e_raw = map
        .remove("fp:co2e")
        .ok_or(ParseError::MissingRequired("fp:co2e"))?;
    let co2e = co2e_raw.parse::<f64>().map_err(|_| ParseError::InvalidValue {
        field: "fp:co2e",
        expected: "decimal number",
        got: co2e_raw.clone(),
    })?;

    let unit_raw = map
        .remove("fp:co2e:unit")
        .ok_or(ParseError::MissingRequired("fp:co2e:unit"))?;
    let co2e_unit = Co2Unit::from_str(&unit_raw).ok_or_else(|| ParseError::InvalidValue {
        field: "fp:co2e:unit",
        expected: "kg | g | t",
        got: unit_raw.clone(),
    })?;

    // ── Recommended fields ───────────────────────────────────────────────

    // Unknown scope values are silently mapped to None for forward compatibility.
    let scope = map.remove("fp:scope").and_then(|s| Scope::from_str(&s));

    let per = map.remove("fp:per");
    let methodology = map.remove("fp:methodology");
    let certifier = map.remove("fp:certifier");
    let verified_date = map.remove("fp:verified:date");

    // ── Optional breakdown ───────────────────────────────────────────────

    let breakdown = Breakdown {
        materials: parse_optional_f64(&mut map, "fp:materials"),
        manufacturing: parse_optional_f64(&mut map, "fp:manufacturing"),
        transport: parse_optional_f64(&mut map, "fp:transport"),
        use_phase: parse_optional_f64(&mut map, "fp:use"),
        disposal: parse_optional_f64(&mut map, "fp:disposal"),
    };

    Ok(FootprintData {
        product,
        co2e,
        co2e_unit,
        scope,
        per,
        methodology,
        certifier,
        verified_date,
        breakdown,
    })
}

fn parse_optional_f64(map: &mut HashMap<String, String>, key: &str) -> Option<f64> {
    map.remove(key).and_then(|v| v.parse::<f64>().ok())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Co2Unit;

    const MINIMAL: &str = r#"<html><head>
        <meta property="fp:product" content="Test Widget" />
        <meta property="fp:co2e" content="1.5" />
        <meta property="fp:co2e:unit" content="kg" />
    </head></html>"#;

    #[test]
    fn parses_minimal_required_fields() {
        let data = parse(MINIMAL).unwrap();
        assert_eq!(data.product, "Test Widget");
        assert!((data.co2e - 1.5).abs() < f64::EPSILON);
        assert_eq!(data.co2e_unit, Co2Unit::Kg);
        assert!(data.scope.is_none());
        assert!(data.breakdown.materials.is_none());
    }

    #[test]
    fn parses_full_example() {
        let html = r#"<html><head>
            <meta property="fp:product" content="Fairphone 5" />
            <meta property="fp:co2e" content="23.6" />
            <meta property="fp:co2e:unit" content="kg" />
            <meta property="fp:scope" content="lifecycle" />
            <meta property="fp:per" content="unit" />
            <meta property="fp:methodology" content="ISO 14067" />
            <meta property="fp:certifier" content="https://cert.example.org" />
            <meta property="fp:verified:date" content="2025-01-15" />
            <meta property="fp:materials" content="8.2" />
            <meta property="fp:manufacturing" content="4.1" />
            <meta property="fp:transport" content="2.8" />
            <meta property="fp:use" content="6.9" />
            <meta property="fp:disposal" content="1.6" />
        </head></html>"#;
        let data = parse(html).unwrap();
        assert_eq!(data.product, "Fairphone 5");
        assert!((data.co2e - 23.6).abs() < 0.001);
        assert_eq!(data.scope, Some(Scope::Lifecycle));
        assert_eq!(data.per.as_deref(), Some("unit"));
        assert_eq!(data.methodology.as_deref(), Some("ISO 14067"));
        assert!((data.breakdown.materials.unwrap() - 8.2).abs() < 0.001);
        assert!((data.breakdown.transport.unwrap() - 2.8).abs() < 0.001);
    }

    #[test]
    fn errors_on_missing_product() {
        let html = r#"<html><head>
            <meta property="fp:co2e" content="1.5" />
            <meta property="fp:co2e:unit" content="kg" />
        </head></html>"#;
        assert_eq!(parse(html), Err(ParseError::MissingRequired("fp:product")));
    }

    #[test]
    fn errors_on_missing_co2e() {
        let html = r#"<html><head>
            <meta property="fp:product" content="Widget" />
            <meta property="fp:co2e:unit" content="kg" />
        </head></html>"#;
        assert_eq!(parse(html), Err(ParseError::MissingRequired("fp:co2e")));
    }

    #[test]
    fn errors_on_invalid_co2e_value() {
        let html = r#"<html><head>
            <meta property="fp:product" content="Widget" />
            <meta property="fp:co2e" content="not-a-number" />
            <meta property="fp:co2e:unit" content="kg" />
        </head></html>"#;
        let err = parse(html).unwrap_err();
        assert!(matches!(err, ParseError::InvalidValue { field: "fp:co2e", .. }));
    }

    #[test]
    fn errors_on_invalid_unit() {
        let html = r#"<html><head>
            <meta property="fp:product" content="Widget" />
            <meta property="fp:co2e" content="1.5" />
            <meta property="fp:co2e:unit" content="lbs" />
        </head></html>"#;
        let err = parse(html).unwrap_err();
        assert!(matches!(err, ParseError::InvalidValue { field: "fp:co2e:unit", .. }));
    }

    #[test]
    fn unknown_scope_becomes_none() {
        let html = r#"<html><head>
            <meta property="fp:product" content="Widget" />
            <meta property="fp:co2e" content="1.5" />
            <meta property="fp:co2e:unit" content="kg" />
            <meta property="fp:scope" content="cradle-to-grave" />
        </head></html>"#;
        let data = parse(html).unwrap();
        assert!(data.scope.is_none());
    }

    #[test]
    fn ignores_unknown_fp_properties() {
        let html = r#"<html><head>
            <meta property="fp:product" content="Widget" />
            <meta property="fp:co2e" content="1.5" />
            <meta property="fp:co2e:unit" content="kg" />
            <meta property="fp:custom:field" content="some value" />
        </head></html>"#;
        assert!(parse(html).is_ok());
    }

    #[test]
    fn parses_grams_unit() {
        let html = r#"<html><head>
            <meta property="fp:product" content="Straw" />
            <meta property="fp:co2e" content="450" />
            <meta property="fp:co2e:unit" content="g" />
        </head></html>"#;
        let data = parse(html).unwrap();
        assert_eq!(data.co2e_unit, Co2Unit::G);
    }
}
