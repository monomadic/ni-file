use ni_file::{NIFileType, NIRepository, RepositoryRoot};

fn setup_logger() {
    let _ = log::set_logger(&loggy::Loggy {
        prefix: "",
        show_time: false,
        show_thread: true,
    });
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();

    // lets read a Kontakt 7 file.
    let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    // make sure this is a valid repository
    if NIFileType::detect(file) == NIFileType::Repository {
        // read the repository
        let mut repo = NIRepository::read(file)?;
        let root: RepositoryRoot = repo.data()?;

        // // print the major version number
        // println!(
        //     "NISound Version: {}.{}.{}",
        //     root.major_version(),
        //     root.minor_version(),
        //     root.patch_version(),
        // );

        // // iterate children
        // println!("children found: {}", repo.children()?.len());
        // for item in repo.children()? {
        //     println!("{:?}", item.frame_stack()?.frame()?)
        // }
    }

    Ok(())
}
