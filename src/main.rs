mod load_cell_parser;
use load_cell_parser::LoadCellParser;

fn main() {
    let mut parser = LoadCellParser::new();

    let mut user_input = String::new();

    println!("Hit ENTER to start listening");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Couldn't read line");

    parser.start_listening();
}
