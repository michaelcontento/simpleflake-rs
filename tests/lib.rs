extern crate simpleflake;

#[test]
fn test_new() {
    let id = simpleflake::new();
    assert!(id > 0);
}

#[test]
fn test_new_returns_different_values() {
    let id_a = simpleflake::new();
    let id_b = simpleflake::new();
    assert!(id_a != id_b);
}

#[test]
fn test_length_of_generated_ids() {
    let id = simpleflake::new();
    assert!(id > 1000000000000000000);
}

#[test]
fn test_parse() {
    let id = 3878068333444056242;
    let parts = simpleflake::parse(id);

    assert_eq!(parts.timestamp, 1409004570.859);
    assert_eq!(parts.random_bits, 2081970);
}

#[test]
fn test_id_can_be_printed() {
    let id = 3878068333444056242;
    let parts = simpleflake::parse(id);

    println!("{:?}", parts);
}
