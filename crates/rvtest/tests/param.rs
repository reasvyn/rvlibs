use rvtest::param::{parametrize, parametrize_named};

#[test]
fn parametrized_runs_all_cases() {
    let results = parametrize("add", [(1, 1, 2), (0, 0, 0), (-1, 1, 0)], |(a, b, exp)| {
        assert_eq!(a + b, *exp);
    });
    assert!(results.iter().all(|c| c.status.is_passed()));
    assert_eq!(results.len(), 3);
}

#[test]
fn parametrized_supports_named_cases() {
    let results = parametrize_named(
        "parse",
        [("empty", ""), ("valid", "42")],
        |input| {
            if !input.is_empty() {
                assert!(input.parse::<i32>().is_ok());
            }
        },
    );
    assert!(results.iter().all(|c| c.status.is_passed()));
}
