pub static TYPE_CORRECT_SOURCE_PROGRAMS: &'static [&str] = &[
    "function main () -> int { -(9 + 10 * 10 - (9/10)) }",
    "function main () -> void { () }",
    "function main () -> void { for i:= 13 to 15 do break }",
    "function main () -> void { while 1 do break }",
    "function main () -> void { while 0 do () }",
    "function main () -> void { for i:= 0 to 10 do () }",
    "function main () -> void { for i:= 11 to 10 do () }",
    "function main () -> void { if 2 < 10 then () }",
    "function main () -> int { if 2 < 10 then 2 else 10 }",
    "function main () -> int { if 2 > 10 then 2 else 10 }",
    "function main () -> int { 10 < 11 }",
    "function main () -> int { 10 <= 10 }",
    "function main () -> int { 11 > 10 }",
    "function main () -> int { 10 >= 10 }",
    "function main () -> int { 10 = 10 }",
    "function main () -> int { 10 <> 10 }",
    "function main () -> int { 2147483647 }",
    "function main () -> int { let var a:int := 0 in a end }",
    r#"
    type intArray = array of int
    function main () -> int {
        let var a : intArray := intArray [10] of 20 in
            a[9]
        end
    }
    "#,
    r#"
    type intArray = array of int
    function main () -> int {
        let var a : intArray := intArray [10] of 2 in (
            for i:= 1 to 9 do (a[i] := a[i-1] + a[i]);
            a[9]
        ) end
    }
    "#,
    r#"
    type r = {i: int, j: int}
    function main () -> int {
        let var a : r := r {i = 15, j = 5} in
            (a.j := a.i + a.j;
            a.j)
        end
    }
    "#,
    r#"
    function main () -> int {
        let var a : int := 40 in
            (a < 50 and (a:= 10; 0);
            a)
        end
    }
    "#,
    r#"
    function main () -> int {
        let var a : int := 40 in
            (a > 50 and (a:= 10; 0);
            a)
        end
    }
    "#,
    r#"
    function main () -> int {
        let var a : int := 40 in
            (a < 50 or (a:= 10; 0);
            a)
        end
    }
    "#,
    r#"
    function main () -> int {
        let var a : int := 40 in
            (a > 50 or (a:= 10; 0);
            a)
        end
    }
    "#
];
