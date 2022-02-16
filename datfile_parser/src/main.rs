fn main() {
    let paths = std::fs::read_dir("datfile_parser/examples").unwrap();

    for path in paths {
        let file = std::fs::read(path.unwrap().path()).expect("unable to read the file");

        let parser = wot_datfile_parser::DatFileParser::new();
        let result = parser.parse(&file).unwrap();

        let s= serde_json::to_string_pretty(&result).unwrap();

        println!("{}", s);
    }
    // std::fs::write("result.json", s);
}
