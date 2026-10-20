use budget_macros::budget_lt;
use soroban_sdk::Env;

struct Dummy;

#[budget_lt(cpu = 1000000, mem = 1000000)]
impl Dummy {
    #[budget_macros::budget_cpu_lt(100000)]
    pub fn method_cpu(&self, _env: &soroban_sdk::Env) {}

    #[budget_macros::budget_mem_lt(100000)]
    pub fn method_mem(&self, _env: &soroban_sdk::Env) {}

    #[budget_macros::budget_write_bytes_lt(1000)]
    pub fn method_write(&self, _env: &soroban_sdk::Env) {}

    #[budget_macros::budget_read_bytes_lt(1000)]
    pub fn method_read(&self, _env: &soroban_sdk::Env) {}

    #[budget_macros::budget_events_lt(10)]
    pub fn method_events(&self, _env: &soroban_sdk::Env) {}

    #[budget_macros::budget_ledger_entries_lt(10)]
    pub fn method_ledger(&self, _env: &soroban_sdk::Env) {}

    #[budget_macros::budget_scaling(sizes = [10, 100], model = linear, tolerance = 0.5)]
    pub fn method_scaling(_env: Env, _size: u32) { let _ = _env; }

    #[budget_macros::budget_lt(cpu = 100000, mem = 100000)]
    pub fn method_both(&self, _env: &soroban_sdk::Env) {}
}

fn main() {}
