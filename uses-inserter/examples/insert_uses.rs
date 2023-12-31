//! Insert the appropriate uses clauses recursively
use uses_inserter::insert_uses_clauses;
use uses_inserter::ModName;

fn main() -> uses_inserter::Result<()> {
    pretty_env_logger::init();
    let base_path = "../oanda_v2/src";
    let mod_name = ModName::new(base_path).add_part("lib");
    let files_to_ignore = ["host"]
        .into_iter()
        .map(|s| ModName::new(base_path).add_part(s))
        .collect();
    insert_uses_clauses(mod_name, &files_to_ignore)
}