use couchbase::Cluster;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Airport {
    airportname: String,
    icao: String,
}

fn main() {
    let mut cluster = Cluster::connect("couchbase://127.0.0.1", "Administrator", "password");
    let bucket = cluster
        .bucket("travel-sample")
        .expect("Could not open bucket");
    let collection = bucket.default_collection();

    let found_doc = collection
        .get("airport_1297", None)
        .expect("Error while loading doc");
    println!("Airline Document: {:?}", found_doc);

    if found_doc.is_some() {
        println!(
            "Content Decoded {:?}",
            found_doc.unwrap().content_as::<Airport>()
        );
    }

    println!("Airline Document: {:?}", collection.get("enoent", None));

    println!("Upsert: {:?}", collection.upsert("foo", "bar", None));
    println!("Get: {:?}", collection.get("foo", None));

    println!("Remove: {:?}", collection.remove("foo", None));
    println!("Get: {:?}", collection.get("foo", None));

    println!("First Insert: {:?}", collection.insert("bla", "bla", None));
    println!("Second Insert: {:?}", collection.insert("bla", "bla", None));

    // cluster.disconnect();

    std::thread::sleep(std::time::Duration::from_secs(100));
}
