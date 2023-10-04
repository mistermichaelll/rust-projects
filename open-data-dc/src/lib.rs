use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TotalRecordCount {
    pub count:i64,
}

// the API for ArcGIS is kind of annoying–where the actual data we want is 
//  a few layers down in the "features" layer. So we create a few different structs:
//    * Feature: This struct represents individual features from the API response and contains the attributes of interest.
//    * ApiResponse: This struct represents the entire API response and is a vector (array) of Feature structs.
//    * Crashes: This struct represents the attributes related to crashes. 
//          Each attribute is annotated with serde rename attributes, which specify how the fields should be deserialized from JSON. 
//          This struct defines the data structure for the crash-related attributes within each feature.
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    pub attributes: Crashes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub features: Vec<Feature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Crashes {
    #[serde(rename = "CRIMEID")]
    pub crime_id:Option<String>,
    #[serde(rename = "REPORTDATE")]
    pub report_date:Option<i64>,
    #[serde(rename = "ADDRESS")]
    pub address:Option<String>,
    #[serde(rename = "LATITUDE")]
    pub latitude:Option<f64>,
    #[serde(rename = "LONGITUDE")]
    pub longitude:Option<f64>,
    #[serde(rename = "XCOORD")]
    pub x_coord:Option<f64>,
    #[serde(rename = "YCOORD")]
    pub y_coord:Option<f64>,
    #[serde(rename = "WARD")]
    pub ward:Option<String>,
    #[serde(rename = "MAJORINJURIES_BICYCLIST")]
    pub major_injuries_bicyclist:i64,
    #[serde(rename = "MINORINJURIES_BICYCLIST")]
    pub minor_injuries_bicyclist:i64,
    #[serde(rename = "UNKNOWNINJURIES_BICYCLIST")]
    pub unknown_injuries_bicyclist:i64,
    #[serde(rename = "FATAL_BICYCLIST")]
    pub fatal_bicyclist:i64,
    #[serde(rename = "MAJORINJURIES_DRIVER")]
    pub major_injuries_driver:i64,
    #[serde(rename = "MINORINJURIES_DRIVER")]
    pub minor_injuries_driver:i64,
    #[serde(rename = "UNKNOWNINJURIES_DRIVER")]
    pub unknown_injuries_driver:i64,
    #[serde(rename = "FATAL_DRIVER")]
    pub fatal_driver:i64,
    #[serde(rename = "MAJORINJURIES_PEDESTRIAN")]
    pub major_injuries_pedestrian:i64,
    #[serde(rename = "MINORINJURIES_PEDESTRIAN")]
    pub minor_injuries_pedestrian:i64,
    #[serde(rename = "UNKNOWNINJURIES_PEDESTRIAN")]
    pub unknown_injuries_pedestrian:i64,
    #[serde(rename = "FATAL_PEDESTRIAN")]
    pub fatal_pedestrian:i64,
    #[serde(rename = "TOTAL_VEHICLES")]
    pub total_vehicles:i64,
    #[serde(rename = "TOTAL_BICYCLES")]
    pub total_bicycles:i64,
    #[serde(rename = "TOTAL_PEDESTRIANS")]
    pub total_pedestrians:i64,
    #[serde(rename = "PEDESTRIANSIMPAIRED")]
    pub pedestrians_impaired:i64,
    #[serde(rename = "BICYCLISTSIMPAIRED")]
    pub bicylists_impaired:i64,
    #[serde(rename = "DRIVERSIMPAIRED")]
    pub drivers_impaired:i64,
    #[serde(rename = "TOTAL_TAXIS")]
    pub total_taxis:i64,
    #[serde(rename = "TOTAL_GOVERNMENT")]
    pub total_government:i64,
    #[serde(rename = "FATALPASSENGER")]
    pub fatal_passanger:f64,
    #[serde(rename = "MAJORINJURIESPASSENGER")]
    pub major_injuries_passenger:f64,
    #[serde(rename = "MINORINJURIESPASSENGER")]
    pub minor_injuries_passenger:f64,
    #[serde(rename = "UNKNOWNINJURIESPASSENGER")]
    pub unknown_injuries_passenger:f64,
    #[serde(rename = "LASTUPDATEDATE")]
    pub last_update_date:Option<i64>
}
