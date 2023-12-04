use crate::commons::read_lines;
struct Position {
    line: usize,
    pos: usize,
}
struct Schematic {
    lines: Vec<String>,
}

trait GetCoordinates {
    fn get_char_at_position(&self, pos: &Position) -> char;
    fn get_adjacents(&self, pos: &Position) -> Vec<char>;
}

impl GetCoordinates for Schematic {
    fn get_char_at_position(&self, pos: &Position) -> char {
        let line: String = self.lines.get(pos.line).unwrap().to_string();
        let c: char = line.chars().nth(pos.pos).unwrap();

        c
    }

    fn get_adjacents(&self, pos: &Position) -> Vec<char> {
        let mut adj: Vec<char> = Vec::new();
        let max_col: usize = self.lines[0].chars().count() - 1;
        let max_lines: usize = self.lines.len() - 1;

        // At Max there are 8 Adjacent char
        // But the below line might not exist or the left and right might no exist
        let mut adjacent_pos: Vec<Position> = Vec::new();
        if pos.line > 0 {
            // Top
            adjacent_pos.push(Position {
                line: pos.line - 1,
                pos: pos.pos,
            });
        }
        if pos.line < max_lines {
            // Below
            adjacent_pos.push(Position {
                line: pos.line + 1,
                pos: pos.pos,
            });
        }
        if pos.pos > 0 {
            // left
            adjacent_pos.push(Position {
                line: pos.line,
                pos: pos.pos - 1,
            });
        }
        if pos.pos < max_col {
            // right
            adjacent_pos.push(Position {
                line: pos.line,
                pos: pos.pos + 1,
            });
        }
        if (pos.pos > 0) & (pos.line > 0) {
            // Top Left
            adjacent_pos.push(Position {
                line: pos.line - 1,
                pos: pos.pos - 1,
            });
        }
        if (pos.pos < max_col) & (pos.line > 0) {
            //Top right
            adjacent_pos.push(Position {
                line: pos.line - 1,
                pos: pos.pos + 1,
            })
        }
        if (pos.pos > 0) & (pos.line < max_lines) {
            //Below left
            adjacent_pos.push(Position {
                line: pos.line + 1,
                pos: pos.pos - 1,
            })
        }
        if (pos.pos < max_col) & (pos.line < max_lines) {
            //Below right
            adjacent_pos.push(Position {
                line: pos.line + 1,
                pos: pos.pos + 1,
            })
        }

        for p in adjacent_pos {
            adj.push(self.get_char_at_position(&p));
        }

        adj
    }
}

struct Number {
    is_part: bool,
    digits: Vec<usize>,
}

trait Compute {
    fn get_value(&self) -> usize;
}

impl Compute for Number {
    fn get_value(&self) -> usize {
        let mut value: usize = 0;
        let base: usize = 10;
        for (i, d) in self.digits.iter().rev().enumerate() {
            value += base.pow(i.try_into().unwrap()) * d;
        }

        value
    }
}

fn is_a_symbol(c: char) -> bool {
    // A Symbol is not a "." and not a digit

    if c == '.' || c.is_digit(10) {
        return false;
    }

    true
}

fn create_numbers(line_index: usize, line: &String, schema: &Schematic) -> Vec<Number> {
    // For simplicity
    let chars: Vec<char> = line.chars().collect();
    // Get all the numbers on a line
    let mut numbers: Vec<Number> = Vec::new();

    let mut is_part: bool = false;
    let mut digits: Vec<usize> = Vec::new(); // This vec will get reset from time to time
    let mut is_in_a_number: bool = false;

    for (ic, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if !is_in_a_number {
                // Then we just started a new number
                digits.clear();
                is_part = false;
                is_in_a_number = true;
            }
            let dig: u32 = c.to_digit(10).unwrap();
            let d: usize = usize::try_from(dig).unwrap();
            digits.push(d);

            let pos = Position {
                line: line_index,
                pos: ic,
            };

            let adj: Vec<char> = schema.get_adjacents(&pos);
            for c in adj {
                if is_a_symbol(c) {
                    is_part = true;
                    break;
                }
            }
        } else {
            if is_in_a_number {
                // We just ended a number
                numbers.push(Number {
                    is_part: is_part,
                    digits: digits.to_vec(), // we create a copy of the vec
                });
                is_in_a_number = false;
            }
        }

        // If current char is last char
        if (ic == (chars.len() - 1)) && is_in_a_number {
            // End what we were doing and return everything
            numbers.push(Number {
                is_part: is_part,
                digits: digits.to_vec(), // we create a copy of the vec
            })
        }
    }

    numbers
}

pub fn day3() {
    // Add up all the part numbers
    // Number adjacent to a symbol (diagonals included) are parts number
    // periods (.) are not symbols
    // Part1 what is the sum of all part numbers

    let lines = read_lines("inputs/day3.txt");

    let schematic = Schematic { lines: lines };

    let mut numbers: Vec<Number> = Vec::new();

    // Find the numbers
    for (il, line) in schematic.lines.iter().enumerate() {
        numbers.extend(create_numbers(il, line, &schematic));
    }

    let mut sum: usize = 0;
    for num in numbers {
        if num.is_part {
            sum += num.get_value();
            println!("Value: {}", num.get_value());
        }
    }
    println!("Total sum of Parts: {}", sum)
}
