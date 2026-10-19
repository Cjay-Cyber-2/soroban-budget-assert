use budget_macros::budget_cpu_lt;

#[budget_cpu_lt(config = "first", config = "second")]
fn test_duplicate_config() {}

fn main() {}
