use crate::check_type::type_check;
use crate::source_grammar::*;
use std::collections::VecDeque;
// In the future we will have a parser (woot, woot)
//use crate::tiger::{ProgramParser};
#[test]
fn test_int_lit() {
    let body = Box::new(Exp::IntLit { value: 2 });
    let args = VecDeque::new();
    let name = "main".to_string();
    let return_type = "int".to_string();

    let dec_list = vec![TopLevelDec::FunDec {
        body,
        args,
        name,
        return_type,
    }]
    .into_iter()
    .collect();

    let program = Program { dec_list };

    type_check(program).unwrap();
}

#[test]
fn test_str_lit() {
    let body = Box::new(Exp::Sequence {
        sequence: VecDeque::new(),
    });
    let args = VecDeque::new();
    let name = "main".to_string();
    let return_type = "void".to_string();

    let body_ = Box::new(Exp::StringLit {
        value: "Hello".to_string(),
    });
    let args_ = VecDeque::new();
    let name_ = "s".to_string();
    let return_type_ = "string".to_string();

    let dec_list = vec![
        TopLevelDec::FunDec {
            body: body_,
            args: args_,
            name: name_,
            return_type: return_type_,
        },
        TopLevelDec::FunDec {
            body,
            args,
            name,
            return_type,
        },
    ]
    .into_iter()
    .collect();

    let program = Program { dec_list };

    type_check(program).unwrap();
}
/***** Tests from SRC code -- Requires Program Parser ************
#[test]
fn test_int_lit_src() {
    let program = "function main () : int { 9 }";

    //print!("{:#?}", ProgramParser::new().parse(program));
    assert!(ProgramParser::new().parse(program).is_ok());
}

#[test]
fn test_string_lit_src() {
    let program = "function main () : int { (\"String\"; 9) }";

    //print!("{:#?}", ProgramParser::new().parse(program));
    assert!(ProgramParser::new().parse(program).is_ok());
}


#[test]
fn test_int_lit_exp_src() {
    let program = "function main () : int { -(9 + 10 * 10 - (9/10)) }";
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_array_exp_src() {
    let program = "
    type intArray = array of int
    function main (): int {
        let var a : intArray := intArray[10] of 0 in
        (a[0])
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_rec_exp_src() {
    let program = "
    type intRec = { i: int }
    function main (): int {
        let var a : intRec := intRec { i = 3 } in
        (a.i)
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_if_then_else_exp_src() {
    let program = "
    type intRec = { i: int }
    function main (): int {
        let var a : intRec := intRec { i = 3 } in
            if a.i < 3 then 1 else 3
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}


#[test]
fn test_if_then_exp_src() {
    let program = "
    type intRec = { i: int }
    function main (): int {
        let var a : intRec := intRec { i = 3 } in
            (if a.i < 3 then a.i := 10; a.i)
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_while() {
    let program = "
    type intRec = { i: int }
    function main (): void {
        let var a : intRec := intRec { i = 3 } in
            while a.i < 1 do (a.i = a.i + 1; ())
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_while_break() {
    let program = "
    type intRec = { i: int }
    function main (): void {
        let var a : intRec := intRec { i = 3 } in
            while a.i < 1 do (
                a.i = a.i + 1;
                if a.i < 10 then break;
                ()
            )
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_for() {
    let program = "
    type intArray = array of int
    function main (): void {
        let var a : intArray := intArray [10] of 20 in
            for i := 0 to 10 do (a[i] := 1; ())
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_for_break() {
    let program = "
    type intArray = array of int
    function main (): void {
        let var a : intArray := intArray [10] of 20 in
            for i := 0 to 10 do (a[i] := 1; break; ())
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_call_rec() {
    let program = "
    type intArray = array of int
    function main (): void {
        main()
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_call_missing_name() {
    let program = "
    function main (): void {
        missingName ()
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_missing_symbol() {
    let program = "
    function main (): void {
        missingName
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_break_for_cond() {
    let program = "
    type intArray = array of int
    function main (): void {
        let var a : intArray := intArray [10] of 20 in
            for i := 0 to (break; 10) do (a[i] := 1; break; ())
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_break_for_cond_1() {
    let program = "
    type intArray = array of int
    function main (): void {
        let var a : intArray := intArray [10] of 20 in
            for i := (break; 0) to 10 do (a[i] := 1; break; ())
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_break_while_cond_1() {
    let program = "
    type intRec = { i: int }
    function main (): void {
        let var a : intRec := intRec { i = 3 } in
            while (break; a.i < 1) do (
                a.i = a.i + 1;
                if a.i < 10 then break;
                ()
            )
        end
    }
    ";
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_assign() {
    let program = r#"
    type intRec = { i: int }
    function main (): void {
        let var a : intRec := intRec { i = 3 } in
            (a.i = "Hello"; () )
        end
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_if_then_else_exp_src() {
    let program = r#"
    type intRec = { i: int }
    function main (): void {
        let var a : intRec := intRec { i = 3 } in
            (if a.i < 3 then "Hello" else 3; ())
        end
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_call_src() {
    let program = r#"
    function main (): void {
        (f("10"); ())
    }
    function f(i: int): int {
        i + 1
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_return_src() {
    let program = r#"
    function main (): void {
        (f(10); ())
    }
    function f(i: int): int {
        "i + 1"
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_return_context_src() {
    let program = r#"
    function main (): void {
        (f(10) + 10; ())
    }
    function f(i: int): str {
        "i + 1"
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_return_context_src_1() {
    let program = r#"
    function main (): int {
        f(10)
    }
    function f(i: int): str {
        i + 1
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_array_len() {
    let program = r#"
    type intArray = array of int
    function main (): void {
        let var a : intArray := intArray ["Hello"] of 20 in
            ()
        end
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_bad_array_bad_init() {
    let program = r#"
    type intArray = array of int
    function main (): void {
        let var a : intArray := intArray [10] of "20" in
            ()
        end
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}

#[test]
fn test_array_array_int() {
    let program = r#"
    type intArray = array of int
    type intArrayArray = array of intArray
    function main (): void {
        let var a : intArrayArray := intArrayArray [10] of (intArray [10] of 1) in
            ()
        end
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
}

#[test]
fn test_bad_array_array_int_init() {
    let program = r#"
    type intArray = array of int
    type intArrayArray = array of intArray
    function main (): void {
        let var a : intArrayArray := intArrayArray [10] of ("Hello") in
            ()
        end
    }
    "#;
//    print!("{:#?}", ProgramParser::new().parse(program).unwrap());
    type_check(ProgramParser::new().parse(program).unwrap()).unwrap_err();
}
*/
