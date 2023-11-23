use crates_io_api::{Error as cError, SyncClient};
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;
use serde::Serialize;

#[allow(dead_code)]
fn list_top_dependencies() -> Result<(), cError> {
    // Instantiate the client.
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();
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

#[allow(dead_code)]
fn list_reverse_dependencies(software_name: &str) -> Result<(), cError> {
    // Instantiate the client.
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();
    // Retrieve summary data.
    let reverse_dep = client.crate_reverse_dependencies(software_name)?;
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

#[allow(dead_code)]
fn list_category(software_name: &str) {
    // Instantiate the client.
    let client = SyncClient::new(
        "my-user-agent (my-contact@domain.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();
    let krate = client.get_crate(software_name).unwrap();
    let downloads =  krate.crate_data.downloads;
    let description = krate.crate_data.description.unwrap();
    for category in krate.categories {
        println!(
            "the category of crate [{}] is [{}], downloads [{}], description [{}]",
            software_name, category.category, downloads,description
        )
    }
}

#[derive(Debug, Serialize)]
struct CrateInfo {
    name: String,
    downloads: u64,
    description:String,
    categories: String,
}

#[allow(dead_code)]
fn csv_demo(software_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(software_path).expect("failed to open CSV file");
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    // Build the CSV reader and iterate over each record.

    let out_path="./out.csv";
    let out_file = std::fs::File::create(out_path).expect("failed to create output file");
    let mut writer = csv::Writer::from_writer(out_file);

    for result in reader.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;

        let client = SyncClient::new(
            "my-user-agent (my-contact@domain.com)",
            std::time::Duration::from_millis(1000),
        )
        .unwrap();
        match client.get_crate(record.as_slice()) {
            Ok(krate)=>{
                let mut cate_string =String::new();
                for category in krate.categories {
                    cate_string.push_str(&category.category);
                    cate_string.push_str("/");
                }
                writer.serialize(CrateInfo {
                    name: record.as_slice().to_string(),
                    downloads: krate.crate_data.downloads,
                    description: krate.crate_data.description.unwrap(),
                    categories: cate_string,
                })?;
                println!("{} OK", record.as_slice());
            }
            Err(err)=>{
                writer.serialize(CrateInfo {
                    name: record.as_slice().to_string(),
                    downloads: 0,
                    description: "".to_string(),
                    categories: "".to_string(),
                })?;
                println!("{} Error:{}", record.as_slice(),err);
            }
        }
        writer.flush()?;
    }
    Ok(())
}

fn main() {
    // list_reverse_dependencies("rayon").unwrap();
    // list_category("tokio")
    csv_demo("./demo.csv").expect("error");
}
