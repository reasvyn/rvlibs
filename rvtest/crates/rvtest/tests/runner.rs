use rvtest::core::RunnerConfig;
use rvtest::runner::TestRunner;
use rvtest::spec::describe;

#[test]
fn runner_executes_specs_with_custom_config() {
    let config = RunnerConfig {
        parallel: false,
        verbose: true,
        ..RunnerConfig::default()
    };

    let run = TestRunner::new(config)
        .add_spec(describe("Runner test").it("works", || {}))
        .run();

    assert!(run.success());
    assert_eq!(run.total(), 1);
}
