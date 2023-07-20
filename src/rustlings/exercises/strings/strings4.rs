// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

fn main() {
    println!("{}", trim_me("Hello!     "));
    println!("{}", trim_me("  What's up!"));
    println!("{}", trim_me("   Hola!  "));

    println!("{}", compose_me("Hello"));
    println!("{}", compose_me("Goodbye"));

    println!("{}", replace_me("I think cars are cool"));
    println!("{}", replace_me("I love to look at cars"));
}


    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }

