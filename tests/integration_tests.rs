use laydown::parse_arguments;
use laydown::Env;

#[test]
#[should_panic(expected = "Something went horribly wrong.")]
fn test_app_failed_to_run() {
    let arguments = vec![];
    parse_arguments(arguments, Env::Test);
}

#[test]
fn test_no_arguments_passed() {
    let arguments = vec![String::from("name/of/binary/passed/by/default")];
    parse_arguments(arguments, Env::Test);
}
