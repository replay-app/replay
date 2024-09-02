fn main() -> Result<(), build_deps::Error> {
    build_deps::rerun_if_changed_paths("res/**")?;
    build_deps::rerun_if_changed_paths("res")?;

    Ok(())
}
