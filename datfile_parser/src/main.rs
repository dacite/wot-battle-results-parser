

fn main() {
    let path = "datfile_parser/examples/23984215712762303.dat";
    let file = std::fs::read(path).expect("unable to read the file");

    let parser = wot_datfile_parser::DatFileParser::new();
    let result = parser.parse(&file).unwrap();

    let _s= serde_json::to_string_pretty(&result).unwrap();
    // std::fs::write("result.json", s);

}


