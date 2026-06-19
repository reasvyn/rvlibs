use std::path::Path;

use rvtest::arch::arch_check;

#[test]
fn architecture_enforces_module_rules() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    arch_check()
        .src_dir(src_dir)
        .module("core").may_not_depend_on(&["report", "coverage", "runner"])
        .module("report").may_depend_on(&["core"])
        .module("report").may_not_depend_on(&["coverage", "runner"])
        .module("runner").may_depend_on(&["core", "report"])
        .all_modules().must_not_have_cycles()
        .assert_all_pass();
}
