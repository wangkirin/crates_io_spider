use crates_io_api::{Error, SyncClient};

fn list_top_dependencies() -> Result<(), Error> {
    // Instantiate the client.
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000),
    ).unwrap();
    // Retrieve summary data.
    let summary = client.summary()?;
    for c in summary.most_downloaded {
        println!("{}:", c.id);
        for dep in client.crate_dependencies(&c.id, &c.max_version)? {
            // Ignore optional dependencies.
            if !dep.optional {
                println!("    * {} - {}", dep.id, dep.version_id);
            }
        }
    }
    Ok(())
}

fn list_reverse_dependencies(softwareName: &str) -> Result<(), Error> {
    // Instantiate the client.
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000),
    ).unwrap();
    // Retrieve summary data.
    let reverse_dep = client.crate_reverse_dependencies(softwareName)?;
    println!(
        "Number of reverse_dependencies {}:",
        reverse_dep.dependencies.len()
    );

    for rd in reverse_dep.dependencies {
        let v = rd.crate_version;
        // let publisher = match v.published_by {
        //     Some(p) => p,
        //     None => crates_io_api::User {
        //         name: String::from("NONE"),
        //         ...
        //     },
        // };
        println!(
            // "[{}]  - [{}]  -[{:#?}]]  ",
            "[{}]  - [{}]   ",
            v.crate_name,
            v.downloads,
            // v.published_by.unwrap().name,
        );
    }
    Ok(())
}

fn main() {
    let _l1 = list_reverse_dependencies("rayon").unwrap();
}
