use std::env;
use std::io::Read;
use std::process::exit;
use std::fs::File;

// AST -- the "abstract syntax tree" for a brainfuck program.
type AST = Vec<Operation>;

// All defined operations for an AST.
#[derive(Clone)]
#[derive(Debug)]
enum Operation {
    Left,      // <
    Right,     // >
    Increment, // +
    Decrement, // -
    Output,    // .
    Accept,    // ,
    Forward,   // [
    Backward,  // ]
}

// Extract all operations from the input program into an AST
fn operations(input: String) -> AST {
    let mut ops: Vec<Operation> = Vec::new();
    for token in input.chars() {
        match operation(token) {
            Some(op) => ops.push(op),
            None => ()
        }
    }
    return ops
}

fn operation(token: char) -> Option<Operation> {
    return match token {
        '<' => Some(Operation::Left),
        '>' => Some(Operation::Right),
        '+' => Some(Operation::Increment),
        '-' => Some(Operation::Decrement),
        '.' => Some(Operation::Output),
        ',' => Some(Operation::Accept),
        '[' => Some(Operation::Forward),
        ']' => Some(Operation::Backward),
        _ => None
    };
}

fn execute(ast: &AST) {
    let mut data: Vec<u8> = vec![0; 300];
    let mut pointer = 0;
    let mut index = 0;
    while index < ast.len() {
        match ast[index] {
            Operation::Left => {
                pointer = pointer - 1;
                index += 1;
            },
            Operation::Right => {
                pointer = pointer + 1;
                index += 1;
            },
            Operation::Increment => {
                data[pointer] = data[pointer] + 1;
                index += 1;
            },
            Operation::Decrement => {
                data[pointer] = data[pointer] - 1;
                index += 1;
            },
            Operation::Output => {
                print!("{}", data[pointer] as char);
                index += 1;
            },
            Operation::Accept => {
                let mut input: [u8; 1] = [0; 1]; // a single byte array to read a single character into
                std::io::stdin().read_exact(&mut input).expect("Failed to read single byte of data");
                data[pointer] = input[0];
            },
            Operation::Forward => match data[pointer] {
                0 => index = find_end_index(index, &ast) + 1,
                _ => index += 1
            },
            Operation::Backward => match data[pointer] {
                0 => index += 1,
                _ => index = find_start_index(index, &ast),
            }
        }
    }
}

fn find_end_index(start: usize, slice: &AST) -> usize {
    let mut need: usize = 1;
    let mut found: usize = 0;
    for position in start..slice.len() {
        match &slice[position] {
            Operation::Forward => need += 1,
            Operation::Backward => found += 1,
            _ => {}
        }
        if need == found {
            return position;
        }
    }
    // TODO: this is an error
    return 0
}

fn find_start_index(start: usize, slice: &AST) -> usize {
    for position in (0..start).rev() {
        match &slice[position] {
            Operation::Forward => return position,
            _ => {}
        }
    }
    return 0
}

fn program_contents(args: Vec<String>) -> String {
    let filename: &String = &args[1];
    let mut file = File::open(filename).expect("Program not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to open program file");
    return contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit(1)
    }
    let contents: String = program_contents(args);
    let ast: AST = operations(contents);
    execute(&ast); //, &mut [0; 300].to_vec());
}
