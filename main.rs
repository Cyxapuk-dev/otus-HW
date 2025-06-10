macro_rules! my_macro {
    ( $( $func:ident ),* ) => {
        (
            $(
                $func()
            ),*
        )
    };
}

fn foo() -> i32 {
    println!("Вызов foo()");
    42
}

fn bar() -> i32 {
    println!("Вызов bar()");
    7
}

fn baz() -> i32 {
    println!("Вызов baz()");
    13
}

fn main() {
    let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);

    println!(
        "Результаты: foo = {}, bar = {}, baz = {}",
        foo_result, bar_result, baz_result
    );
}
