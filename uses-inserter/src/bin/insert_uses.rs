//! Insert the appropriate uses clauses recursively
use uses_inserter::insert_uses_clauses;
use uses_inserter::ModName;

fn main() {
    let mod_name = ModName::new("../oanda_v2/src").add_part("lib");
    insert_uses_clauses(mod_name);
}
