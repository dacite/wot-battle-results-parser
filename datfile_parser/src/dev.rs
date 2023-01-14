use wot_datfile_parser::DatFileParser;
fn main() {
    let parser = DatFileParser::new();

    let intercept = |intercept_value, original_input| match intercept_value {
        wot_datfile_parser::Intercept::Failed(field, default_val, err) => {
            println!("Mannualy parsed : {}", original_input);
            default_val
        }
        other @ _ => other.original_result(),
    };
    let battle = parser
        .parse_intercept(
            &std::fs::read("datfile_parser/input_files/test/12570413167838342.dat").unwrap(),
            intercept,
        )
        .unwrap();

    std::fs::write("test.json", serde_json::to_string_pretty(&battle).unwrap()).unwrap();
}
