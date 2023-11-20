use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::sync::Arc;
// use regex::Regex;

mod classification;
use classification::{TokenTypes, classify_token};

#[derive(Debug)]
pub struct Token {
    token: String,
    line: u64,
    position: u64,
    token_type: TokenTypes
}

#[derive(Debug)]
pub struct State {
    token: String,
    previous_char: Option<char>,
    pre_previous_char: Option<char>,
    token_stream: Vec<Token>, //TODO: Arc<Token>?
    current_line: u64,
    current_position: u64,
    string_type: Option<char>
}

impl State {
    pub fn new() -> State {
        State {
            token: String::new(),
            token_stream: vec![],
            previous_char: None,
            pre_previous_char: None,
            current_line: 0,
            current_position: 0,
            string_type: None
        }
    }

    pub fn set_previous_char(&mut self, char: char) { self.previous_char = Some(char) }
    pub fn set_pre_previous_char(&mut self, char: char) { self.pre_previous_char = Some(char) }

    pub fn is_currently_in_string(&self) -> bool {
        self.string_type.is_some()
    }

    pub fn clear_token(&mut self) {
        self.token = String::new();
    }
    pub fn add_char_to_token(&mut self, char: char) {
        self.token.push(char);
    }
    pub fn add_char_to_prev_token(&mut self, char: char) {
        let prev_token = self.token_stream.last_mut().unwrap();
        prev_token.token.push(char);
        prev_token.token_type = classify_token(prev_token.token.as_str());
    }
    pub fn add_token_to_stream(&mut self) {
        // Only add the token if it is not empty
        if !self.token.is_empty() {
            let new_token = Token {
                token: self.token.clone(),
                line: self.current_line,
                position: self.current_position,
                token_type: classify_token(self.token.as_str())
            };
            self.token_stream.push(new_token);
            self.clear_token();
        }
    }

    pub fn get_prev_token(&mut self) -> &mut Token {
        self.token_stream.last_mut().unwrap()
    }

    pub fn increment_current_line(&mut self) { self.current_line += 1 }
    pub fn increment_current_position(&mut self) { self.current_position += 1 }
    pub fn clear_current_position(&mut self) { self.current_position = 0 }

    pub fn clear_previous_chars(&mut self) {
        self.previous_char = None;
        self.pre_previous_char = None;
    }
}

fn process_file(file_path: &str, mut state: State) -> State {
    if let Ok(read_lines) = read_lines(file_path) {
        for read_line in read_lines {
            state.increment_current_line();
            state.clear_current_position();
            if let Ok(ip) = read_line {
                for current_char in ip.chars() {
                    state.increment_current_position();
                    if state.is_currently_in_string() {
                        if state.previous_char == Some('\\') && state.pre_previous_char == Some('\\') {
                            state.clear_previous_chars();
                        }

                        match current_char {
                            '\'' | '"' => {
                                if state.string_type.unwrap() == current_char {
                                    if state.string_type.is_some() {
                                        if state.previous_char.unwrap() == '\\' {
                                            state.add_char_to_token(current_char);
                                        } else {
                                            state.string_type = None;
                                            state.add_char_to_token(current_char);
                                            state.add_token_to_stream();
                                        }
                                    } else {
                                        state.string_type = None;
                                        state.add_char_to_token(current_char);
                                        state.add_token_to_stream();
                                    }
                                } else {
                                    state.add_char_to_token(current_char);
                                }
                            }
                            '\\' => {
                                state.add_char_to_token(current_char);
                            }
                            _ => {
                                state.add_char_to_token(current_char);
                            }
                        }
                    } else {
                        if current_char != '=' && current_char != '/' && current_char != ';' {
                            if state.previous_char.is_some() {
                                match state.previous_char.unwrap() {
                                    '/' | '+' | '-' | '*' => {
                                        state.add_char_to_token(state.previous_char.unwrap());
                                        state.add_token_to_stream();
                                    }
                                    _ => {}
                                }
                            }
                        }

                        match current_char {
                            ';' => {
                                state.add_token_to_stream();
                                state.add_char_to_token(current_char);
                                state.add_token_to_stream();
                            }
                            ':' => {
                                if state.previous_char.unwrap() == current_char {
                                    state.add_char_to_prev_token(current_char);
                                } else {
                                    state.add_token_to_stream();
                                    state.add_char_to_token(current_char);
                                    state.add_token_to_stream();
                                }
                            }
                            // Strings / chars:
                            '"' | '\'' => {
                                state.string_type = Some(current_char);
                                state.add_char_to_token(current_char);
                            }
                            //Comments, division:
                            '/' => {
                                // Put this / into 'previous_char'
                                state.add_token_to_stream();
                                if state.previous_char == Some('/') {
                                    break;
                                }
                            }
                            '+' | '-' => {
                                if state.previous_char.unwrap() == current_char {
                                    state.add_char_to_prev_token(current_char);
                                } else {
                                    state.add_token_to_stream();
                                }
                            }
                            '*' => {
                                state.add_token_to_stream();
                            }
                            '=' => {
                                if state.previous_char == Some('=') || state.previous_char == Some('!') || state.previous_char == Some('>') || state.previous_char == Some('<') {
                                    state.add_char_to_prev_token(current_char);
                                } else {
                                    match state.previous_char.unwrap() {
                                        '/' | '*' | '+' | '-' => {
                                            state.add_char_to_token(state.previous_char.unwrap());
                                            state.add_char_to_token(current_char);
                                            state.add_token_to_stream();
                                        }
                                        _ => {
                                            state.add_token_to_stream();
                                            state.add_char_to_token(current_char);
                                            state.add_token_to_stream();
                                        }
                                    }
                                }
                            }
                            '!' => {
                                if state.previous_char == Some('!') {
                                    state.add_char_to_prev_token(current_char);
                                } else {
                                    state.add_token_to_stream();
                                    state.add_char_to_token(current_char);
                                    state.add_token_to_stream();
                                }
                            }
                            '<' | '>' => {
                                if state.previous_char == Some('=') && current_char == '>' {
                                    state.add_char_to_prev_token(current_char);
                                } else if state.get_prev_token().token == current_char.to_string() {
                                    state.add_char_to_prev_token(current_char);
                                } else {
                                    state.add_token_to_stream();
                                    state.add_char_to_token(current_char);
                                    state.add_token_to_stream();
                                }
                            }
                            '{' | '}' | '(' | ')' => {
                                state.add_token_to_stream();
                                state.add_char_to_token(current_char);
                                state.add_token_to_stream();
                            }
                            ' ' | '\r' | '\t' | '\n' => {
                                if !state.token.is_empty() {
                                    state.add_token_to_stream();
                                }
                            }
                            _ => {
                                state.add_char_to_token(current_char);
                            }
                        }
                    }

                    if state.previous_char.is_some() {
                        state.set_pre_previous_char(state.previous_char.unwrap());
                    }
                    state.set_previous_char(current_char);
                }
            }

            state.previous_char = None;
            // End of line, so it'll be a token for sure.
            state.add_token_to_stream();
        }
    }

    state
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let state: State = State::new();
    let file_path = "example.cpp";
    for line in process_file(file_path, state).token_stream {
        println!("{:?}", line);
    }
    // let token_stream = process_file(file_path, state).token_stream;
}

#[cfg(test)]
mod tests {
    use crate::{process_file, State, Token};

    #[test]
    fn test_strings() {
        let state: State = State::new();
        let strings_stream: Vec<Token> = process_file("./tests/strings.cpp", state).token_stream;
        assert_eq!(strings_stream.len(), 5);
        assert_eq!(strings_stream[0].token, "\"'\"");
        assert_eq!(strings_stream[1].token, "\"alphabets: a b c A B C\"");
        assert_eq!(strings_stream[2].token, "\"numbers: 0 1 2 3 4 5 6 7 8 9\"");
        assert_eq!(strings_stream[3].token, "\"special: ! = _ + - /\"");
        assert_eq!(strings_stream[4].token, "\"\\\"\"");
    }

    #[test]
    fn test_logic() {
        let state: State = State::new();
        let logic_stream: Vec<Token> = process_file("./tests/logic.cpp", state).token_stream;
        assert_eq!(logic_stream.len(), 34);
        assert_eq!(logic_stream[0].token, "a");
        assert_eq!(logic_stream[1].token, "==");
        assert_eq!(logic_stream[2].token, "b");

        assert_eq!(logic_stream[4].token, "!=");

        assert_eq!(logic_stream[7].token, ">=");

        assert_eq!(logic_stream[10].token, "<=");

        assert_eq!(logic_stream[13].token, ">");

        assert_eq!(logic_stream[16].token, "<");

        assert_eq!(logic_stream[19].token, "||");

        assert_eq!(logic_stream[22].token, "&&");
        
        assert_eq!(logic_stream[24].token, "!");

        assert_eq!(logic_stream[26].token, "!!");

        assert_eq!(logic_stream[29].token, "<<");
        assert_eq!(logic_stream[32].token, ">>");
    }

    #[test]
    fn test_chars() {
        let state: State = State::new();
        let chars_stream: Vec<Token> = process_file("./tests/chars.cpp", state).token_stream;
        assert_eq!(chars_stream.len(), 7);
        assert_eq!(chars_stream[0].token, "'\\''");
        assert_eq!(chars_stream[1].token, "'\"'");
        assert_eq!(chars_stream[2].token, "'a'");
        assert_eq!(chars_stream[3].token, "'1'");
        assert_eq!(chars_stream[4].token, "'_'");
        assert_eq!(chars_stream[5].token, "'@'");
        assert_eq!(chars_stream[6].token, "'-'");
    }

    #[test]
    fn test_math() {
        let state: State = State::new();
        let chars_stream: Vec<Token> = process_file("./tests/math.cpp", state).token_stream;
        assert_eq!(chars_stream.len(), (12 * 3) + (2 * 2));

        assert_eq!(chars_stream[0].token, "a");
        assert_eq!(chars_stream[1].token, "=");
        assert_eq!(chars_stream[2].token, "b");

        assert_eq!(chars_stream[4].token, "==");

        assert_eq!(chars_stream[7].token, "/");
        assert_eq!(chars_stream[10].token, "/=");

        assert_eq!(chars_stream[13].token, "+");
        assert_eq!(chars_stream[16].token, "+=");

        assert_eq!(chars_stream[19].token, "-");
        assert_eq!(chars_stream[22].token, "-=");

        assert_eq!(chars_stream[25].token, "*");
        assert_eq!(chars_stream[28].token, "*=");

        assert_eq!(chars_stream[31].token, "++");
        assert_eq!(chars_stream[33].token, "--");

        assert_eq!(chars_stream[34].token, "a");
        assert_eq!(chars_stream[35].token, "++");
        assert_eq!(chars_stream[36].token, ";");

        assert_eq!(chars_stream[37].token, "b");
        assert_eq!(chars_stream[38].token, "--");
        assert_eq!(chars_stream[39].token, ";");
    }
}