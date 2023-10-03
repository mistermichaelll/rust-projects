use reqwest::Error;
use serde_json::from_str;
use open_data_dc::ApiResponse;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = build_crashes_url();

    let response_text: String = reqwest::get(url)
        .await?
        .text()
        .await?;

    let api_response: ApiResponse = from_str(&response_text).expect("Failed to parse");

    print_output(api_response);

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

    url
}

fn print_output(a:ApiResponse) {
    // look at a few of the features of the API Response
    for feature in a.features {
        println!("CRIMEID: {}", feature.attributes.crime_id);
        println!("REPORTDATE: {}", feature.attributes.report_date);
        println!("ADDRESS: {}", feature.attributes.address);
        println!("LATITUDE: {}", feature.attributes.latitude);
        println!("LONGITUDE: {}", feature.attributes.longitude);
        println!("TOTAL CYCLISTS: {}", feature.attributes.total_bicycles);
        println!("--------------------------------------------");
    }
}