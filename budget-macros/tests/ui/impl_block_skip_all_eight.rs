use budget_macros::*;
use soroban_sdk::Env;

struct Dummy;

#[budget_lt(cpu = 1000000, mem = 1000000)]
impl Dummy {
    #[budget_cpu_lt(100000)]
    pub fn method_cpu(&self, env: &Env) {}

    #[budget_mem_lt(100000)]
    pub fn method_mem(&self, env: &Env) {}

    #[budget_write_bytes_lt(1000)]
    pub fn method_write(&self, env: &Env) {}

    #[budget_read_bytes_lt(1000)]
    pub fn method_read(&self, env: &Env) {}

    #[budget_events_lt(10)]
    pub fn method_events(&self, env: &Env) {}

    #[budget_ledger_entries_lt(10)]
    pub fn method_ledger(&self, env: &Env) {}

    #[budget_scaling(sizes = [10, 100], model = linear, tolerance = 0.5)]
    pub fn method_scaling(env: Env, size: u32) {}

    #[budget_lt(cpu = 100000, mem = 100000)]
    pub fn method_both(&self, env: &Env) {}
}

fn main() {}
