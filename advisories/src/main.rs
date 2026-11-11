use rustsec::{Database, advisory::Informational, database::Query};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::fetch()?;

    let unmaintained_packages: Vec<_> = db
        .into_iter()
        .filter_map(|advisory| {
            if advisory.metadata.informational == Some(Informational::Unmaintained) {
                Some(advisory.metadata.package.clone())
            } else {
                None
            }
        })
        .collect();

    let db = Database::fetch()?;

    for package in &unmaintained_packages {
        let query = Query::new().package_name(package.clone());

        let advisories = db.query(&query);

        match advisories[..] {
            [] => panic!(
                "unmaintained package {} has no unmaintained advisory",
                package
            ),
            [advisory] => {
                assert!(advisory.metadata.informational == Some(Informational::Unmaintained));
            }
            [..] => {
                println!("---");
                println!("{}: {} advisories", package, advisories.len() - 1);
                dbg!(advisories);
            }
        }
    }

    Ok(())
}
