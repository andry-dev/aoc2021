use std::fs;

struct State {
    depth: i32,
    aim: i32,
    horizontal: i32,
}

impl State {
    fn new() -> State {
        State {
            depth: 0,
            aim: 0,
            horizontal: 0,
        }
    }
}

fn execute_command1(command: &str, step: i32, state: State) -> State {
    match command {
        "forward" => State {
            aim: state.aim,
            depth: state.depth,
            horizontal: state.horizontal + step,
        },

        "up" => State {
            aim: state.aim,
            depth: state.depth - step,
            horizontal: state.horizontal,
        },

        "down" => State {
            aim: state.aim,
            depth: state.depth + step,
            horizontal: state.horizontal,
        },

        _ => state,
    }
}

fn execute_command2(command: &str, step: i32, state: State) -> State {
    match command {
        "forward" => State {
            aim: state.aim,
            depth: state.depth + state.aim * step,
            horizontal: state.horizontal + step,
        },

        "up" => State {
            aim: state.aim - step,
            depth: state.depth,
            horizontal: state.horizontal,
        },

        "down" => State {
            aim: state.aim + step,
            depth: state.depth,
            horizontal: state.horizontal,
        },

        _ => state,
    }
}

fn main() {
    let input_contents = fs::read_to_string("data/input").expect("Can't find file!");

    let lines: Vec<&str> = input_contents.split('\n').filter(|&s| s != "").collect();

    let part1: State = lines
        .iter()
        .fold(State::new(), |state: State, line: &&str| {
            let commands: Vec<_> = line.split(" ").collect();

            execute_command1(commands[0], commands[1].parse().unwrap(), state)
        });

    println!("{}", part1.depth * part1.horizontal);

    let part2: State = lines
        .iter()
        .fold(State::new(), |state: State, line: &&str| {
            let commands: Vec<_> = line.split(" ").collect();

            execute_command2(commands[0], commands[1].parse().unwrap(), state)
        });

    println!("{}", part2.depth * part2.horizontal);
}
