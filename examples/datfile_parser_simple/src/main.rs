use wot_datfile_parser::DatFileParser;

fn main() {
    let file = std::fs::read("../../datfile_parser/input_files/WOT_1_19_1_0/19011713064132879.dat").unwrap();

    // You must construct the parser first as it needs to some initialization to parse the datfiles
    let parser = DatFileParser::new();

    // The parser generates a Battle struct
    let battle = parser.parse(&file).unwrap();

    assert_eq!(
        &battle.common["teamHealth"],
        &serde_json::json!({ "1": 13595, "2": 12985 })
    );
    assert_eq!(&battle.common["duration"], &serde_json::json!(407));

    // Battle implements serde::Serialize and serde::Deserialize. So, you can use other data formats as well.
    // Here we will convert it to json and print it:
    let battle_as_json = serde_json::to_string_pretty(&battle).unwrap();

    println!("{battle_as_json}");
}
