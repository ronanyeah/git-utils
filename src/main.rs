use std::{fs, io};

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir("..")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_by(|a, b| a.file_name().unwrap().cmp(&b.file_name().unwrap()));

    println!("-> NO GIT");
    for dir in &entries {
        let mut sub = fs::read_dir(dir)?;
        let res = sub.find(|v| v.as_ref().unwrap().file_name().to_str().unwrap() == ".git");
        if res.is_none() {
            println!("{:?}", dir.file_name().unwrap());
        }
    }

    println!("\n-> EMPTY");
    for dir in &entries {
        let mut sub = fs::read_dir(dir)?;
        let res = sub.find(|v| v.as_ref().unwrap().file_name().to_str().unwrap() == ".git");
        if res.is_some() {
            let empty = git2::Repository::open(res.unwrap().unwrap().path())
                .unwrap()
                .is_empty()
                .unwrap();
            if empty {
                println!("{:?}", dir.file_name().unwrap());
            }
        }
    }

    println!("\n-> DIRTY");
    for dir in &entries {
        let mut sub = fs::read_dir(dir)?;
        let res = sub.find(|v| v.as_ref().unwrap().file_name().to_str().unwrap() == ".git");
        if res.is_some() {
            let path = res.unwrap().unwrap().path();
            let repo = git2::Repository::open(path.clone()).unwrap();
            if !repo.is_empty().unwrap() {
                let diff = repo
                    .diff_index_to_workdir(
                        None,
                        Some(
                            git2::DiffOptions::new()
                                .include_untracked(true)
                                .recurse_untracked_dirs(true),
                        ),
                    )
                    .unwrap();

                let deltas = diff.deltas().len();

                let statuses = repo
                    .statuses(Some(git2::StatusOptions::new().include_untracked(true)))
                    .unwrap()
                    .len();

                if deltas > 0 || statuses > 0 {
                    println!(
                        "{} - {}",
                        dir.file_name().unwrap().to_str().unwrap(),
                        std::cmp::max(deltas, statuses)
                    );
                }
            }
        }
    }

    Ok(())
}
