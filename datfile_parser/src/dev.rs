use wot_datfile_parser::DatFileParser;

fn main() {
    let parser = DatFileParser::new();

    let battle = parser
        .parse(&std::fs::read("datfile_parser/input_files/test/12570413167838342.dat").unwrap())
        .unwrap();
    std::fs::write("test.json", serde_json::to_string_pretty(&battle).unwrap()).unwrap()
}
