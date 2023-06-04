use crate::built_in_functions::get_std_ast;
use crate::environment::Environment;
use crate::object::Object;
use crate::parser::Parser;
use crate::{evaluator::Node, lexer::Lexer};
use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

const MONKEY_FACE: &str = r#"         .-"-.
       _/_-.-_\_
      /|( o o )|\
     / //  "  \\ \ 
    / / \'---'/ \ \
    \ \_/`"""`\_/ /
     \           /"#;

pub fn start<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut lines = input.lines();
    let ref env = Environment::new();
    let std = get_std_ast();
    std.eval(env).unwrap();
    {
        let x = env.borrow();
        (*x).debug_print();
    }

    loop {
        write!(output, "{PROMPT}")?;
        output.flush()?;

        let line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        };

        let lexer = Lexer::new(line);

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            writeln!(output, "Woops! We ran into some monkey business here!")?;
            writeln!(output, "{}", MONKEY_FACE)?;
            writeln!(output, "Parser Error:")?;

            for error in parser.errors {
                writeln!(output, "{}", error)?;
            }
        }

        // writeln!(output, "<temp> parser output{:?}", program.statements)?;

        let evaluated = match program.eval(env) {
            Ok(evaluated) => evaluated,
            Err(e) => {
                writeln!(output, "Woops! We ran into some monkey business here!")?;
                writeln!(output, "{}", MONKEY_FACE)?;
                writeln!(output, "Evaluation Error:")?;
                writeln!(output, "{e}")?;
                continue;
            }
        };

        {
            let x = env.borrow();
            (*x).debug_print();
        }

        if let Object::None = *evaluated {
            continue;
        }

        writeln!(output, "{}", evaluated)?
    }
}
