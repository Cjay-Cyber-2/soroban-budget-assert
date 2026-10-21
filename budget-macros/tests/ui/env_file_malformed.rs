use budget_macros::budget_cpu_lt;


#[path = "support/mock_env.rs"]
mod mock_env;

#[budget_cpu_lt(env_file = "tests/ui/env_file_malformed.env", env = "TARGET_CPU_LIMIT")]
fn test_env_file_malformed_lines_skipped() {}
// Keep the attribute on the documented line used by the compile-fail snapshot.
#[budget_cpu_lt(env_file = "tests/ui/env_file_malformed.env", env = "TARGET_CPU_LIMIT")]
fn test_env_file_malformed() {}

fn main() {}
