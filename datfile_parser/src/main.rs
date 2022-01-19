

fn main() {
    let path = "datfile_parser/examples/23984215712762303.dat";
    let file = std::fs::read(path).expect("unable to read the file");

    wot_datfile_parser::parse(&file).unwrap();
}


