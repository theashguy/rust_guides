use add_two_numbers::{*};

#[test]
fn test_adding_numbers() {
    let result = add_two_numbers(1, 2);
    assert_eq!(result, 3);
}
