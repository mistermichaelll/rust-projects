use reqwest;
use std::fs::write;
use serde_json::to_vec;
use open_data_dc::ApiResponse;
use open_data_dc::Feature;
use open_data_dc::TotalRecordCount;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let url = build_crashes_url();
    let total_records = get_total_record_count().await?;
    let mut offset = 0;
    let rate_limit = 1000;
    let mut records:Vec<Feature> = Vec::new();

    let client = reqwest::Client::builder()
        .build()?;

    println!("{:?} records to fetch.", total_records);
    while offset < total_records {
        let api_response = client
            .get(&url)
            .query(&[("resultOffset", &offset),("resultRecordCount", &rate_limit)])
            .send()
            .await?;

        let mut r:ApiResponse = api_response
            .json()
            .await?;

        offset += r.features.len() as i64;
        println!("Fetched {} records", offset);
        records.append(&mut r.features)
    }

    let serialized = to_vec(&records)?;

    write("output.json", serialized).expect("Failed to serialized to JSON.");

    Ok(())
}

fn build_crashes_url() -> String {
    let mut url = String::from("https://maps2.dcgis.dc.gov/dcgis/rest/services/DCGIS_DATA/Public_Safety_WebMercator/MapServer/24/query?where=1%3D1");

    // we can adjust this as-needed, but generally these are the fields I want.
    let field_names: [&str; 33] = [
        "CRIMEID",
        "REPORTDATE",
        "ADDRESS",
        "LATITUDE",
        "LONGITUDE",
        "XCOORD",
        "YCOORD",
        "WARD",
        "MAJORINJURIES_BICYCLIST",
        "MINORINJURIES_BICYCLIST",
        "UNKNOWNINJURIES_BICYCLIST",
        "FATAL_BICYCLIST",
        "MAJORINJURIES_DRIVER",
        "MINORINJURIES_DRIVER",
        "UNKNOWNINJURIES_DRIVER",
        "FATAL_DRIVER",
        "MAJORINJURIES_PEDESTRIAN",
        "MINORINJURIES_PEDESTRIAN",
        "UNKNOWNINJURIES_PEDESTRIAN",
        "FATAL_PEDESTRIAN",
        "TOTAL_VEHICLES",
        "TOTAL_BICYCLES",
        "TOTAL_PEDESTRIANS",
        "PEDESTRIANSIMPAIRED",
        "BICYCLISTSIMPAIRED",
        "DRIVERSIMPAIRED",
        "TOTAL_TAXIS",
        "TOTAL_GOVERNMENT",
        "LASTUPDATEDATE",
        "FATALPASSENGER",
        "MAJORINJURIESPASSENGER",
        "MINORINJURIESPASSENGER",
        "UNKNOWNINJURIESPASSENGER"
    ];

    let out_fields = field_names.join(",");

    url.push_str(&format!("&outfields={}", out_fields));
    url.push_str("&returnGeometry=false&outSR=4326&f=json");

    return url
}

async fn get_total_record_count() -> Result<i64, Box<dyn std::error::Error>> {
    let count_url: &str = &build_crashes_url();
    
    let client = reqwest::Client::builder() 
        .build()?;

    let api_response = client 
        .get(count_url)
        .query(&[("returnCountOnly", "true")])
        .send()
        .await?;

    let r: TotalRecordCount = api_response 
        .json::<TotalRecordCount>()
        .await?;
    
    return Ok(r.count);
}
