use wot_datfile_parser::{DatFileParser, Intercept};

fn main() {
    let file = std::fs::read("../../datfile_parser/input_files/WOT_1_19_1_0/19011713064132879.dat").unwrap();

    // You must construct the parser first as it needs to some initialization to parse the datfiles
    let parser = DatFileParser::new();

    // We can use the intercept function to change how a serde_pickle::Value is converted to serde_json::Value
    // We can also use it to log any errors in the datfile_parser(by matching the Failed variant)
    let intercept_fn = |intercept, _original_value| {
        use Intercept::*;

        match intercept {
            Success(field, _) | NotPresent(field, _) | ManuallyParsed(field, _) | Failed(field, _, _) => {
                if field.name == "teamHealth" {
                    // Here we can inspect the original_value ourselves provide our own impl for converting
                    // the serde_pickle::Value to serde_json::Value But for this example,
                    // we will just return the following:
                    serde_json::Value::String("My own parser for teamHealth".into())
                } else {
                    intercept.original_result()
                }
            }
        }
    };

    // The parser generates a Battle struct
    let battle = parser.parse_intercept(&file, intercept_fn).unwrap();

    assert_eq!(
        &battle.common["teamHealth"],
        &serde_json::json!("My own parser for teamHealth")
    );
    assert_eq!(&battle.common["duration"], &serde_json::json!(407));

    // Battle implements serde::Serialize and serde::Deserialize. So, you can use other data formats as well.
    // Here we will convert it to json and print it:
    let battle_as_json = serde_json::to_string_pretty(&battle).unwrap();

    println!("{battle_as_json}");
}
