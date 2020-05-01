mod common;
#[macro_use]
mod x64;
#[macro_use]
mod lir;
mod backend;
#[macro_use]
mod x64s;
mod check_type;
mod checked_grammar;
mod lowering;
mod source_grammar;

#[cfg(test)]
mod test_type_check;

use crate::backend::compile;
use crate::check_type::type_check;
use lowering::lower;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Set up command line argument handling
    let matches = clap_app!(catc =>
        (version: "1.0.0")
        (author: "Chris Phifer <cphifer@galois.com>")
        (about: "A compiler for the Cat programming language.")
        (@arg INFILE: +required "The Cat source file to be compiled")
        (@arg OUTFILE: -o --output +takes_value "Sets a custom output file, defaulting to a.s")
    )
    .get_matches();

    // Get source file from clargs, read into a string to parse
    let mut source_file = File::open(matches.value_of("INFILE").unwrap())?;
    let mut program = String::new();
    source_file.read_to_string(&mut program)?;

    // Begin compiling!
    let parser = parser::ProgramParser::new();

    let program = parser
        .parse(&program)
        .expect("There was an error while parsing.");
    let type_checked_program =
        type_check(program).expect("There was an error while checking types.");
    let (lir_program, label_gen, symbol_gen) = lower(type_checked_program);
    let compiled_program = compile(lir_program, label_gen, symbol_gen);

    // Output file handling
    let mut output_file = File::create(matches.value_of("OUTFILE").unwrap_or("a.s"))?;
    output_file.write_all(format!("{}\n", compiled_program).as_bytes())?;

    Ok(())
}
