use xshell::cmd;

fn main() {
    cmd!("cargo fmt --all").run().unwrap();
    cmd!("cargo clippy --workspace --all-targets -- -W clippy::pedantic -A clippy::must_use_candidate")
        .run()
        .expect("Fix clippy errors");
}
