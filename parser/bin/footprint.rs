use std::env;
use std::fs;
use std::process;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: footprint <file.html>");
        eprintln!();
        eprintln!("Parse Footprint Protocol meta tags from an HTML file.");
        eprintln!("See https://github.com/conscious-collective/footprint for the spec.");
        process::exit(1);
    });

    let html = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("Error: could not read '{}': {}", path, e);
        process::exit(1);
    });

    match footprint_parser::parse(&html) {
        Ok(data) => {
            println!("Product:   {}", data.product);
            println!("CO2e:      {} {}", data.co2e, data.co2e_unit);

            if let Some(scope) = &data.scope {
                println!("Scope:     {}", scope);
            }
            if let Some(per) = &data.per {
                println!("Per:       {}", per);
            }
            if let Some(method) = &data.methodology {
                println!("Method:    {}", method);
            }
            if let Some(cert) = &data.certifier {
                println!("Certifier: {}", cert);
            }
            if let Some(date) = &data.verified_date {
                println!("Verified:  {}", date);
            }

            let b = &data.breakdown;
            if b.materials.is_some()
                || b.manufacturing.is_some()
                || b.transport.is_some()
                || b.use_phase.is_some()
                || b.disposal.is_some()
            {
                println!();
                println!("Lifecycle breakdown ({}):", data.co2e_unit);
                if let Some(v) = b.materials     { println!("  Materials:     {}", v); }
                if let Some(v) = b.manufacturing { println!("  Manufacturing: {}", v); }
                if let Some(v) = b.transport     { println!("  Transport:     {}", v); }
                if let Some(v) = b.use_phase     { println!("  Use:           {}", v); }
                if let Some(v) = b.disposal      { println!("  Disposal:      {}", v); }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    }
}
