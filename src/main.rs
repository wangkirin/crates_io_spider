use crates_io_api::{Category, Error as cError, SyncClient};
use csv::ReaderBuilder;
use std::fs::File;
use std::{error::Error, io, process};
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
struct Crate_info {
    name: String,
    // categories: Vec<String>,
    downloads: u64,
    description:String,
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
        println!("{}", record.as_slice());
        let client = SyncClient::new(
            "my-user-agent (my-contact@domain.com)",
            std::time::Duration::from_millis(1000),
        )
        .unwrap();
        let krate = client.get_crate(record.as_slice()).unwrap();
        writer.serialize(Crate_info{
            name: record.as_slice().to_string(),
            // categories: krate.categories.,
            downloads: krate.crate_data.downloads,
            description: krate.crate_data.description.unwrap(),
        })?;
        writer.flush()?;
    }
    Ok(())
}

fn main() {
    // list_reverse_dependencies("rayon").unwrap();
    // list_category("tokio")
    csv_demo("./demo.csv").expect("error");
}
