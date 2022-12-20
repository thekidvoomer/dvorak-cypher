use std::io;

fn main() -> io::Result<()> {
    let mut phrase_to_cypther = String::new();
    let times_to_cypther: u32;
    let shift_value: u32;
    let start_shift: u32;

    let stdin = io::stdin();

    //  TODO: random inputs (appart from the phrase)
    //  TODO: decipher
    //  TODO: input file

    println!("Enter a phrase to cypher:");
    stdin.read_line(&mut phrase_to_cypther).unwrap();

    println!("Enter the number of times to cypher (uint):");
    times_to_cypther = input_int();

    println!("Enter the shift value (uint):");
    shift_value = input_int();

    println!("Enter the start shift value (uint):");
    start_shift = input_int();

    let dvorak_cypher_parameters = DvorakCypherParameters {
        phrase_list: phrase_to_cypther,
        times_to_cypher: times_to_cypther,
        shift_value: shift_value,
        start_shift: start_shift,
    };

    let dvorak_cypher_result = dvorak_cypher(dvorak_cypher_parameters);

    println!("Cyphered phrase:");
    print!("{}", dvorak_cypher_result.phrase_list);
    println!("cypher depth: {}", dvorak_cypher_result.times_to_cypher);
    println!("shift value: {}", dvorak_cypher_result.shift_value);
    println!("start shift value: {}", dvorak_cypher_result.start_shift);

    Ok(())
}

fn input_int() -> u32 {
    let mut input;
    let uint: u32 = loop {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please type a number!"),
        }
    };
    uint
}

fn dvorak_cypher(mut dvorak_cypher_parameters: DvorakCypherParameters) -> DvorakCypherParameters {
    //create and fill vec
    let dvorak_list: Vec<(char, char)> = vec![
        ('a', '\''),
        ('z', ','),
        ('e', '.'),
        ('r', 'p'),
        ('t', 'y'),
        ('y', 'f'),
        ('u', 'g'),
        ('i', 'c'),
        ('o', 'r'),
        ('p', 'l'),
        ('q', 'a'),
        ('s', 'o'),
        ('d', 'e'),
        ('f', 'u'),
        ('g', 'i'),
        ('h', 'd'),
        ('j', 'h'),
        ('k', 't'),
        ('l', 'n'),
        ('m', 's'),
        ('w', ';'),
        ('x', 'q'),
        ('c', 'j'),
        ('v', 'k'),
        ('b', 'x'),
        ('n', 'b'),
        (',', 'm'),
        (';', 'w'),
        ('.', 'v'),
        ('\'', 'z'),
    ];

    let mut shift: u32 = dvorak_cypher_parameters.start_shift;

    (0..dvorak_cypher_parameters.times_to_cypher).for_each(|_| {
        //cypher phrase_list
        dvorak_cypher_parameters.phrase_list = one_pass(
            &dvorak_list,
            &dvorak_cypher_parameters.phrase_list.to_lowercase(),
            shift,
        );

        shift += dvorak_cypher_parameters.shift_value;
    });

    dvorak_cypher_parameters
}

struct DvorakCypherParameters {
    phrase_list: String,
    times_to_cypher: u32,
    shift_value: u32,
    start_shift: u32,
}

fn one_pass(dvorak_list: &Vec<(char, char)>, phrase_list: &String, shift: u32) -> String {
    let mut char_list = phrase_list.chars().collect::<Vec<char>>();

    char_list = char_list
        .iter()
        .map(|char| get_new_letter(dvorak_list, *char, shift))
        .collect();

    char_list.iter().collect::<String>()
}

fn get_new_letter(dvorak_list: &Vec<(char, char)>, char: char, shift: u32) -> char {
    let position = find_letter(dvorak_list, char);
    if position == None {
        char
    } else {
        let new_position = (position.unwrap() + (shift as usize)) % dvorak_list.len();
        dvorak_list[new_position].1
    }
}

fn find_letter(dvorak_list: &Vec<(char, char)>, char: char) -> Option<usize> {
    dvorak_list
        .iter()
        .position(|(dvorak_char, _)| *dvorak_char == char)
}
