use serde::{Deserialize};

// the API for ArcGIS is kind of annoying–where the actual data we want is 
//  a few layers down in the "features" layer. So we create a few different structs:
//    * a struct, Feature, containing the attributes we're interested in.
//    * a struct containing the Crashes columns (Crashes), basically the attributes of each feature.
//    * a struct containing the API Response, which is a vector of features.
#[derive(Debug, Deserialize)]
pub struct Feature {
    pub attributes: Crashes,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub features: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
pub struct Crashes {
    #[serde(rename = "CRIMEID")]
    pub crime_id:String,
    #[serde(rename = "REPORTDATE")]
    pub report_date:i64,
    #[serde(rename = "ADDRESS")]
    pub address:String,
    #[serde(rename = "LATITUDE")]
    pub latitude:f64,
    #[serde(rename = "LONGITUDE")]
    pub longitude:f64,
    #[serde(rename = "XCOORD")]
    pub x_coord:f64,
    #[serde(rename = "YCOORD")]
    pub y_coord:f64,
    #[serde(rename = "WARD")]
    pub ward:String,
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
