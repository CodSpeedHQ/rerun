use re_build_tools::is_tracked_env_var_set;

fn main() {
    // Required for `cargo build` to work on mac: https://pyo3.rs/main/building-and-distribution#macos
    pyo3_build_config::add_extension_module_link_args();

    re_build_tools::export_build_info_vars_for_crate("rerun_py");

    let rerun_bin = std::env::current_dir().unwrap().join("rerun_sdk/bin/rerun");

    // Fail if bin/rerun is missing and this isn't a maturin dev build
    if !is_tracked_env_var_set("RERUN_PY_DEV_BUILD") && !rerun_bin.exists() {
        eprintln!("ERROR: Expected to find `rerun` at `{rerun_bin:?}`.");
        std::process::exit(1);
    }
}
