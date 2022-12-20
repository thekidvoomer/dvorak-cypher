use std::io;

fn main() -> io::Result<()> {
    let mut phrase_to_cypther = String::new();
    let mut times_to_cypther = String::new();
    let mut shift_value = String::new();
    let mut start_shift = String::new();

    let stdin = io::stdin();

    //  TODO: add error handling
    //  TODO: random inputs (appart from the phrase)
    //  TODO: decipher

    println!("Enter a phrase to cypher:");
    stdin.read_line(&mut phrase_to_cypther).unwrap();

    println!("Enter the number of times to cypher (int):");
    stdin.read_line(&mut times_to_cypther).unwrap();

    println!("Enter the shift value (int):");
    stdin.read_line(&mut shift_value).unwrap();

    println!("Enter the start shift value (int):");
    stdin.read_line(&mut start_shift).unwrap();

    let dvorak_cypher_parameters = DvorakCypherParameters {
        phrase_list: phrase_to_cypther,
        times_to_cypher: times_to_cypther.trim().parse().unwrap(),
        shift_value: shift_value.trim().parse().unwrap(),
        start_shift: start_shift.trim().parse().unwrap(),
    };

    let dvorak_cypher_result = dvorak_cypher(dvorak_cypher_parameters);

    println!("Cyphered phrase:");
    print!("{}", dvorak_cypher_result.phrase_list);
    println!("cypher depth: {}", dvorak_cypher_result.times_to_cypher);
    println!("shift value: {}", dvorak_cypher_result.shift_value);
    println!("start shift value: {}", dvorak_cypher_result.start_shift);

    Ok(())
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
