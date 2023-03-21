use ni_file::NIFileType;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<std::path::PathBuf> = glob::glob("tests/data/files/**/*.*")?
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    // repository containers (used in most instruments)
    for path in paths {
        println!("checking: {:?}", path);

        let file = std::fs::read(path)?;
        println!("detected: {:?}", NIFileType::detect(&file));
    }

    Ok(())
}
