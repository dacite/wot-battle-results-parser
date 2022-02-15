

fn main() {
    let path = "datfile_parser/examples/32401277874794934.dat";
    let file = std::fs::read(path).expect("unable to read the file");

    let parser = wot_datfile_parser::DatFileParser::new();
    let result = parser.parse(&file).unwrap();

    let s= serde_json::to_string_pretty(&result).unwrap();
    // println!("{}", s);
    // std::fs::write("result.json", s);

}


