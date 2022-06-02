use std::path::Path;

use wot_datfile_parser::{utils, DatFileParser};


fn main() {
    env_logger::init();
    let parser = DatFileParser::new();

    let mut datfiles = utils::parse_dir(Path::new("datfile_parser/input_files/WOT_1_17_0_0")).unwrap();
    datfiles.append(&mut utils::parse_dir(Path::new("datfile_parser/input_files/WOT_1_16_1_0")).unwrap());
    datfiles.append(&mut utils::parse_dir(Path::new("datfile_parser/input_files/other")).unwrap());
    datfiles.append(&mut utils::parse_dir(Path::new("datfile_parser/input_files/test")).unwrap());
    datfiles.append(&mut utils::parse_dir(Path::new("datfile_parser/input_files")).unwrap());

    let battles = datfiles
        .into_iter()
        .map(|datfile| {
            let input = std::fs::read(datfile.path()).unwrap();

            let battle = parser.parse(&input);

            battle.unwrap()
        })
        .collect::<Vec<_>>();

    std::fs::write("test.json", serde_json::to_string_pretty(&battles).unwrap()).unwrap();
}
