use uses_inserter::{mod_info_recursive, ModName};

/// Runs over oanda_v2 and collects all the declarations recursively
#[test]
fn test_run_on_writer() {
    pretty_env_logger::init();
    let base_path = "../oanda_v2/src";
    let mod_name = ModName::new(base_path).add_part("lib");
    // let files_to_ignore = ["host", "error", "client"]
    //     .into_iter()
    //     .map(|s| ModName::new(base_path).add_part(s))
    //     .collect();
    let mod_info = mod_info_recursive(mod_name);
    // For now just print it, but eventually we'll turn this into an actual test
    println!("{mod_info:#?}");
}
