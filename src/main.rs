use std::process;

const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";
const BG_MAGENTA: &str = "\x1b[45m";

const MOO: &str = "moooooo";

// This function will
//
// convert a string to a binary string (uppercase : 1; lowercase : 0)
// => "mooOoOo"       variable : input
//        ↓ ↓
// => "0001010"       variable : output
fn convert_string_to_binary_string(input: &String) -> String {
    let mut output = String::new();

    for input_char in input.chars() {
        output.push_str(if input_char.is_uppercase() { "1" } else { "0" });
    }

    output
}

// This function will
//
// convert a binary string to a decimal number
// => "0001010"       variable : input
// => 10              variable : output
fn binary_string_to_decimal(input: &String) -> i32 {
    let output = i32::from_str_radix(&input, 2).unwrap();

    output
}

// This function will :
//
// step 1 - convert a string to a binary string (uppercase : 1; lowercase : 0)
// => "mooOoOo"       variable : input
//        ↓ ↓
// => "0001010"       variable : input_as_binary
//
// step 2 - Then convert the binary string to a decimal number
// => "0001010"       variable : input_as_binary
// => 10              variable : input_as_decimal
//
// step 3 - add the specified number to the decimal number
// => 10 + 5        variable : input_as_decimal + to_add
// => 15            variable : input_as_decimal
//
// step 4 - convert back the decimal number to a binary string
// => 15              variable : input_as_decimal
// => "0001111"       variable : output_as_binary
//
// step 5 - and finally convert the binary string to the origin string but with the specified upper/lowercase
// => "0001111"       variable : output_as_binary
//        ↓↓↓↓
// => "mooOOOO"       variable : output

fn add_to_string_like_binary(input: String, to_add: i32) -> String {
    let mut output = String::new(); // the final result
    let input_as_binary: String; // the input converted to a binary string
    let mut input_as_decimal: i32; // the input converted to a decimal number
    let output_as_binary: String; // the output converted to a binary string

    // step 1

    input_as_binary = convert_string_to_binary_string(&input);

    // step 2

    input_as_decimal = binary_string_to_decimal(&input_as_binary);

    // step 3
    input_as_decimal += to_add;

    // step 4

    output_as_binary = format!("{:7b}", input_as_decimal);

    // step 5

    for (currend_char_index, current_bit) in output_as_binary.chars().enumerate() {
        let current_char = input.chars().nth(currend_char_index).unwrap();

        if current_bit == '1' {
            output.push_str(current_char.to_uppercase().to_string().as_str());
        } else {
            output.push_str(current_char.to_lowercase().to_string().as_str());
        }
    }

    output
}

fn main() {
    let mut command = "to_cow"; // the command to execute : to_cow, to_text, help, version
    let mut input = String::from(""); // the input string to convert

    let chars = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ', '.', ',', '!', '?', ':', ';', '"', '\'', '`',
        '~', '\\', '/', '|', '=', '-', '_', '+', '(', ')', '[', ']', '{', '}',
    ]; // Supported characters

    let mut args: Vec<String> = std::env::args().collect(); // command line arguments
    args.remove(0); // remove the first argument (the name of the program => "cow-translator")

    while args.len() > 0 {
        match args[0].as_str() {
            "-c" | "--to-cow" => {
                command = "to_cow"; // set the command to : convert text to cow language
                args.remove(0);
            }

            "-t" | "--to-text" => {
                command = "to_text"; // set the command to : convert cow language to text
                args.remove(0);
            }
            "--" => {
                args.remove(0);
                input = args.join(" ").to_string(); // everything after the "--" is the input and isn't parsed

                if input.is_empty() {
                    // error if the input is empty
                    eprintln!(
                        "{}Error : the input is empty\nPlease specify the input after the '--'{}",
                        RED, RESET
                    );
                    process::exit(1);
                }

                break; // stop parsing arguments
            }

            "-v" | "--version" => {
                // print version and exit
                println!(
                    "{}cow-translator => {}{}",
                    GREEN,
                    MAGENTA,
                    env!("CARGO_PKG_VERSION")
                );
                process::exit(0);
            }

            "-h" | "--help" => {
                // print help and exit
                println!("{}{} cow-translator {}", WHITE, BG_MAGENTA, RESET);
                println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
                println!("Author: {}SkwalExe{}", MAGENTA, RESET);
                println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
                println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
                println!("Options : ");
                println!(
                    "\t{}--version, -v: {}Prints the version of the program{}",
                    MAGENTA, YELLOW, RESET
                );
                println!(
                    "\t{}--help, -h: {}Prints this help message{}",
                    MAGENTA, YELLOW, RESET
                );
                println!(
                    "\t{}--to-cow, -c: {}Converts text to cow [Default]{}",
                    MAGENTA, YELLOW, RESET
                );
                println!(
                    "\t{}--to-text, -t: {}Converts cow to text{}",
                    MAGENTA, YELLOW, RESET
                );
                println!(
                    "\t{}-- [text] : {}Specify the text to convert{}",
                    MAGENTA, YELLOW, RESET
                );
                println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
                println!("Example : ");
                println!("{}cow-translator -- Hello World!{}", MAGENTA, RESET);
                println!("{}> mOoOoOO moooOoo mooOoOO mooOoOO mooOOOo mOOOOOo mOOOoOo mooOOOo moOoooO mooOoOO mooooOO MoooooO{}", BLUE, RESET);

                process::exit(0);
            }

            _ => {
                // if the parameter is not recognized, print error and exit
                println!(
                    "{}Invalid argument: {}{} {} {}",
                    RED, BG_RED, WHITE, args[0], RESET
                );
                process::exit(1);
            }
        }
    }

    match command {
        "to_cow" => {
            if input.is_empty() {
                // error if the input is empty
                eprintln!(
                    "{}Error : the input is empty\nPlease specify the input after the '--'{}",
                    RED, RESET
                );
                process::exit(1);
            }
            // convert text to cow language
            let mut input_as_numbers = String::new(); // the input converted to numbers, each character is converted to a 2-digit number that represents its index in the chars vector

            for c in input.chars() {
                let c_as_number = match chars.iter().position(|&x| x == c) {
                    // get the index of the character in the chars vector
                    Some(x) => format!("{}{}", if x >= 10 { "" } else { "0" }, x), // add a 0 if the index is less than 10 to make it a 2-digit number
                    None => {
                        println!(
                            "{}[ x ] : Unsupported character : {}{} {} {}",
                            RED, WHITE, BG_RED, c, RESET
                        );
                        process::exit(1);
                    }
                };
                input_as_numbers.push_str(&format!("{}", c_as_number)); // add the 2-digit number to the input_as_numbers string
            }

            let mut input_as_moos = String::new(); // the input converted to moos, each 2-digit number is converted to a moo

            for c in input_as_numbers
                .chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
            // split the input_as_numbers string into 2-digit numbers and convert them to moos
            {
                let c_as_moo =
                    add_to_string_like_binary(MOO.to_string(), c.parse::<i32>().unwrap());
                // take the original moo (MOO constant) and add the 2-digit number to it like if it was a binary number (uppercase is 1, lowercase is 0)

                input_as_moos.push_str(&format!("{} ", c_as_moo)); // add the moo to the input_as_moos string
            }

            println!("\n{}{}", MAGENTA, input_as_moos); // print the result, the input converted to moos
        }
        "to_text" => {
            if input.is_empty() {
                // error if the input is empty
                eprintln!(
                    "{}Error : the input is empty\nPlease specify the input after the '--'{}",
                    RED, RESET
                );
                process::exit(1);
            }
            // convert cow language to text
            let moos_vec = input.split_whitespace().collect::<Vec<&str>>(); // the vector containing all the MOOs of the input

            let mut input_as_text = String::new(); // the input converted to text, each moo is converted to a character

            for moo in moos_vec {
                // if moo to lowercase doens't correspond to the original moo, print error and exit, invalid moo
                if moo.to_lowercase() != MOO {
                    println!(
                        "{}[ x ] : Invalid moo : {}{} {} {}",
                        RED, WHITE, BG_RED, moo, RESET
                    );
                    process::exit(1);
                }
                let moo_as_binary = convert_string_to_binary_string(&moo.to_string()); // the moo converted to a binary string | mooOoOo => 0001010
                let moo_as_decimal = binary_string_to_decimal(&moo_as_binary); // the moo as a decimal number | 0001010 => 10
                let moo_as_char = match chars.get(moo_as_decimal as usize) {
                    // moo_as_decimal is the index of the character in the chars vector
                    Some(x) => x,
                    None => {
                        // if the index is out of bounds, print an error and exit, invalid moo
                        println!(
                            "{}[ x ] : This moo doesn't exists : {}{} {} {}",
                            RED, WHITE, BG_RED, moo, RESET
                        );
                        process::exit(1);
                    }
                };

                input_as_text.push(*moo_as_char); // add the character to the input_as_text string
            }

            println!("\n{}{}", MAGENTA, input_as_text); // print the result, the moos converted to text
        }
        _ => {}
    }
}
