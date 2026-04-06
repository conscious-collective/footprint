/// The unit of the CO2e value.
#[derive(Debug, Clone, PartialEq)]
pub enum Co2Unit {
    Kg,
    G,
    T,
}

impl Co2Unit {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "kg" => Some(Self::Kg),
            "g" => Some(Self::G),
            "t" => Some(Self::T),
            _ => None,
        }
    }
}

impl std::fmt::Display for Co2Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kg => write!(f, "kg"),
            Self::G => write!(f, "g"),
            Self::T => write!(f, "t"),
        }
    }
}

/// The lifecycle scope reported.
#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Lifecycle,
    Production,
    Use,
    Disposal,
}

impl Scope {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "lifecycle" => Some(Self::Lifecycle),
            "production" => Some(Self::Production),
            "use" => Some(Self::Use),
            "disposal" => Some(Self::Disposal),
            _ => None,
        }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lifecycle => write!(f, "lifecycle"),
            Self::Production => write!(f, "production"),
            Self::Use => write!(f, "use"),
            Self::Disposal => write!(f, "disposal"),
        }
    }
}

/// Optional lifecycle breakdown following GHG Protocol phases.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Breakdown {
    /// Raw material extraction and processing.
    pub materials: Option<f64>,
    /// Manufacturing and assembly.
    pub manufacturing: Option<f64>,
    /// Transport and distribution.
    pub transport: Option<f64>,
    /// Use phase (energy consumption, consumables).
    pub use_phase: Option<f64>,
    /// End-of-life treatment and disposal.
    pub disposal: Option<f64>,
}

/// Parsed footprint data extracted from an HTML document.
#[derive(Debug, Clone, PartialEq)]
pub struct FootprintData {
    // ── Required ──────────────────────────────────────────────────────────
    /// The name of the product.
    pub product: String,
    /// CO2 equivalent value.
    pub co2e: f64,
    /// Unit of the CO2e value.
    pub co2e_unit: Co2Unit,

    // ── Recommended ───────────────────────────────────────────────────────
    /// Lifecycle scope (lifecycle, production, use, disposal).
    pub scope: Option<Scope>,
    /// Functional unit (e.g. "unit", "year", "km").
    pub per: Option<String>,
    /// Methodology standard (e.g. "ISO 14067", "GHG Protocol").
    pub methodology: Option<String>,
    /// URL of the certifying body.
    pub certifier: Option<String>,
    /// ISO 8601 date of verification.
    pub verified_date: Option<String>,

    // ── Optional breakdown ────────────────────────────────────────────────
    pub breakdown: Breakdown,
}
