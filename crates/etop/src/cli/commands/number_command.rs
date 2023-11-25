use crate::cli::NumberArgs;
use crate::EtopError;
use toolstr::NumberFormat;

/// print formatted number
pub fn number_command(args: NumberArgs) -> Result<(), EtopError> {
    let number: Result<f64, _> = args.number.parse();
    match number {
        Ok(number) => match (args.format, args.all_formats) {
            (Some(format), _) => print_format(number, format),
            (None, true) => print_all_formats(number),
            (None, false) => {
                return Err(EtopError::InvalidFormat("provide --format or --all-formats".to_string()))
            }
        },
        Err(_) => {
            return Err(EtopError::ParseError(format!(
                "could not parse number: {}",
                args.number
            )))
        }
    }
    Ok(())
}

fn print_format(number: f64, format: String) {
    println!("number: {:?}", number);
    println!("format: {}", format);
    println!();
    match toolstr::format(format.as_str(), number) {
        Ok(formatted) => println!("output: {}", formatted),
        Err(e) => println!("could not format number: {:?}", e),
    }
}

fn print_all_formats(number: f64) {
    println!("number: {:?}", number);
    println!("format: all");
    println!();
    for format_type in toolstr::FormatType::all_variants() {
        let format = NumberFormat::new().format_type(&format_type);
        match format.format(number) {
            Ok(formatted) => println!("{:?}: {}", format_type, formatted),
            Err(e) => println!("could not format number: {:?}", e),
        }
    }
}
