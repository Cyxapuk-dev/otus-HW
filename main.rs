use std::cell::RefCell;
use std::rc::Rc;
struct Logger;

struct Input {
    input: String,
    logger: Rc<Logger>,
    autocomplete_next: bool,
}

impl Input {
    fn new(input: String, logger: Rc<Logger>) -> Self {
        Self {
            input,
            logger,
            autocomplete_next: false,
        }
    }

    fn read(&mut self) -> String {
        let whitespace = self.input.find(' ').unwrap_or(self.input.len());
        let mut word: String = self.input.drain(..whitespace).collect();
        if !self.input.is_empty() {
            self.input.drain(..1); // удалить пробел
        }

        if self.autocomplete_next {
            self.autocomplete_next = false;
            if !word.ends_with('}') {
                word.push('}');
            }
        }

        word
    }

    fn request_autocomplete(&mut self) {
        self.autocomplete_next = true;
    }
}

struct Lexer {
    input: Rc<RefCell<Input>>,
}

impl Lexer {
    fn new(input: Rc<RefCell<Input>>) -> Self {
        Self { input }
    }

    fn call(&mut self) -> String {
        let from_input = self.input.borrow_mut().read();
        if from_input.starts_with('{') {
            return "block_start:".to_owned() + &from_input;
        }
        if from_input.is_empty() {
            return "end".to_owned();
        }
        from_input
    }
}

struct Parser {
    lexer: Lexer,
    input: Rc<RefCell<Input>>,
    logger: Rc<Logger>,
}

impl Parser {
    fn new(input: Rc<RefCell<Input>>, logger: Rc<Logger>) -> Self {
        let lexer = Lexer::new(Rc::clone(&input));
        Self {
            lexer,
            input,
            logger,
        }
    }

    fn parse(&mut self) -> String {
        let mut parsed = vec![];
        let mut value = self.lexer.call();

        while value != "end" {
            let mut v = value;
            if v.starts_with("block_start:") {
                let fixed_v = v.strip_prefix("block_start:").unwrap().to_string();
                self.input.borrow_mut().request_autocomplete();
                v = fixed_v;
            }
            parsed.push(v);
            value = self.lexer.call();
        }

        parsed.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input_string = "{ab aba ba {bb bb} {ab aa".to_owned();
        let expected = "{ab aba} ba {bb bb} {ab aa}".to_owned();

        let logger = Rc::new(Logger);
        let input = Rc::new(RefCell::new(Input::new(input_string, Rc::clone(&logger))));

        let mut parser = Parser::new(Rc::clone(&input), Rc::clone(&logger));

        assert_eq!(parser.parse(), expected);
    }
}

fn main() {}
