use poi::*;

fn main() {
    println!("=== Poi Reduce 0.4 ===");
    println!("Type `help` for more information.");
    let ref std = std();

    let mut prev_expr: Option<Expr> = None;
    loop {
        use std::io::{self, Write};

        print!("> ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("ERROR: Could not read input");
                continue;
            }
        };

        let mut inlined = false;
        match input.trim() {
            "" => {
                // Print separator for readability.
                print!("\n------------------------------------<o=o");
                println!("o=o>------------------------------------\n");
                continue;
            }
            "help" => {print_help(); continue}
            "help eqv" => {print_help_eqv(); continue}
            "help asym" => {print_help_asym(); continue}
            "help dom" => {print_help_dom(); continue}
            "help triv" => {print_help_triv(); continue}
            "help ex" => {print_help_ex(); continue}
            "std" => {for k in std {println!("{}", k)}; continue}
            "inline all" => {
                if let Some(expr) = &prev_expr {
                    prev_expr = Some(match expr.inline_all(std) {
                        Ok(x) => {
                            inlined = true;
                            x
                        }
                        Err(err) => {
                            println!("ERROR: {:?}", err);
                            continue;
                        }
                    });
                } else {
                    println!("ERROR: No previous expression");
                    continue;
                }
            }
            "bye" => break,
            x => {
                // Print definitions of symbol.
                if x.starts_with("def ") {
                    match parse_str(x[4..].trim()) {
                        Ok(Expr::Sym(s)) => {
                            let mut found = false;
                            for k in std.iter() {
                                if let Knowledge::Def(a, b) = k {
                                    if a == &s {
                                        found = true;
                                        println!("{}", b);
                                    };
                                }
                            }
                            if !found {println!("(no definition found)")};
                            continue;
                        }
                        Err(err) => {
                            println!("ERROR:\n{}", err);
                            continue;
                        }
                        _ => {
                            println!("ERROR:\nExpected symbol");
                            continue;
                        }
                    }
                }
            }
        }

        let mut expr = if inlined {prev_expr.unwrap()} else {
                match parse_str(&input) {
                    Ok(expr) => expr,
                    Err(err) => {
                        println!("ERROR:\n{}", err);
                        continue;
                    }
                }
            };
        println!("{}", expr);
        loop {
            if let Ok((nexpr, i)) = expr.reduce(std) {
                if nexpr == expr {break};
                expr = nexpr;
                println!("{}\t\t\t( {} )", expr, std[i]);
            } else {
                break;
            }
        }

        let equivalences = expr.equivalences(std);
        for i in 0..equivalences.len() {
            let j = equivalences[i].1;
            println!("<=>  {}\t\t( {} )", equivalences[i].0, std[j]);
        }

        prev_expr = Some(expr);
    }
}

fn print_help() {print!("{}", include_str!("../assets/help.txt"))}
fn print_help_eqv() {print!("{}", include_str!("../assets/help-eqv.txt"))}
fn print_help_asym() {print!("{}", include_str!("../assets/help-asym.txt"))}
fn print_help_dom() {print!("{}", include_str!("../assets/help-dom.txt"))}
fn print_help_triv() {print!("{}", include_str!("../assets/help-triv.txt"))}
fn print_help_ex() {print!("{}", include_str!("../assets/help-ex.txt"))}
