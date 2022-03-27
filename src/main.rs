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
            let empty = git2::Repository::open(path.clone())
                .unwrap()
                .is_empty()
                .unwrap();
            if !empty {
                let repo = git2::Repository::open(path).unwrap();
                let changes = repo
                    .diff_index_to_workdir(None, None)
                    .unwrap()
                    .deltas()
                    .len();
                if changes > 0 {
                    println!("{:?}", dir.file_name().unwrap());
                }
            }
        }
    }

    Ok(())
}
