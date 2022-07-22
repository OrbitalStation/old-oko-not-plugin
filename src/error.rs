use crate::span::Span;
use std::process::{ExitCode, Termination};
use core::fmt::Display;
use core::ops::{Try, FromResidual, ControlFlow};
use core::convert::Infallible;
use owo_colors::*;

pub struct Result <T> (pub core::result::Result <T, Error>);

impl <T> FromResidual for Result <T> {
    fn from_residual(residual: Result <Infallible>) -> Self {
        Self(Err(match residual.0 {
            Err(err) => err,
            Ok(_) => unsafe { core::hint::unreachable_unchecked() }
        }))
    }
}

impl <T> Try for Result <T> {
    type Output = T;

    type Residual = Result <Infallible>;

    #[inline(always)]
    fn from_output(output: Self::Output) -> Self {
        Self(Ok(output))
    }

    #[inline]
    fn branch(self) -> ControlFlow <Self::Residual, Self::Output> {
        match self.0 {
            Ok(ok) => ControlFlow::Continue(ok),
            Err(err) => ControlFlow::Break(Result::<Infallible>(Err(err)))
        }
    }
}

impl Termination for Result <()> {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(()) => ().report(),
            Err(err) => err.report()
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub message: String,
    pub lines: String,
    pub clarifying: String,
    pub help: Vec <String>,
    pub filename: String,
    pub code: String
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        let ladjust = " ".repeat(self.span.start.line.max(self.span.end.line).to_string().len() + 1);

        print!("{}{} ", "error".bright_red().bold(), ":".bold());
        print_with_style_and_green_if_asterisks(&self.message, |v| print!("{}", v.bold()));
        println!("{}", ":".bold());

        println!("{}{} {}:{:?}", &ladjust[1..], "-->".blue().bold(), self.filename, self.span.start);
        println!("{ladjust}{}", "|".blue().bold());

        for (linenum, line) in self.lines.lines().enumerate() {
            let idx = linenum + self.span.start.line;
            let idx_stringified = idx.to_string();
            let full_line = self.code.lines().nth(idx - 1).unwrap();

            let ladjust2 = " ".repeat(ladjust.len() - idx_stringified.len() - 1);

            let circumflex_ladjsust = if linenum == 0 {
                " ".repeat(self.span.start.column)
            } else {
                String::new()
            };

            print!("{ladjust2}{idx} {stick} {line}\n{ladjust}{stick}{circumflex_ladjsust}{underscoring} ",
                idx = idx_stringified.blue().bold(),
                stick = "|".blue().bold(),
                line = full_line.red(),
                underscoring = "^".repeat(line.len()).bright_red().bold())
        }

        print_with_style_and_green_if_asterisks(&self.clarifying, |v| print!("{}", v.bright_red().bold()));
        println!();

        ExitCode::FAILURE
    }
}

type Printer = fn(&dyn Display);

fn print_with_style_and_green_if_asterisks(message: &str, default: Printer) {
    if let Some(start) = message.find('`') {
        let extra = start + '`'.len_utf8();

        let end = message[extra..].find('`').unwrap_or(message.len() - extra) + extra;

        print_and_use_another_printer_on_span(message, start, end, default, |v| print!("{}", v.green().bold()));
    } else {
        default(&message)
    }
}

fn print_and_use_another_printer_on_span(message: &str, start: usize, end: usize, default: Printer, special: Printer) {
    default(&&message[..start]);
    special(&&message[start..=end]);
    default(&&message[end + message[end..].chars().next().map(char::len_utf8).unwrap_or(0)..])
}
