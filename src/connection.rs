use crate::app::App;



#[derive(Debug)]
pub struct CityInfo {
    // TODO: define elements in the structure
    pub temp: f64,
    pub hum: f64,
}

/// Method that is handling the request to the OpenWeather api
/// and parsing the response
///
/// Returns weather details about a certain city
pub async fn get_data(app: &App) -> Result<CityInfo, MyError>{
    let name = app.city[app.indicator];
    let my_url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&APPID=22817049fc6a80979de5bb18c1109696&units=metric", name);
    match reqwest::get(my_url).await {
        Ok(response) => {
            // Check status code
            // Parse response

            // let first = response.text().await;
            // let second = first.map_err(|err| MyError::ReqwestError(err));
            // let third = second?;

            let element = response.text().await.map_err(|err| MyError::ReqwestError(err))?;
            let parsed: serde_json::Value = serde_json::from_str(&element).map_err(|err| MyError::SerdeError(err))?;

            let parsed_temp = &parsed["main"]["temp"].clone();
            let temp: f64 = serde_json::from_value(parsed_temp.clone()).map_err(|err| MyError::SerdeError(err))?;

            let humy: f64 = serde_json::from_value(parsed_temp.clone()).map_err(|err| MyError::SerdeError(err))?;

            let info = CityInfo {
                temp,
                hum: humy
            };

            Ok(info)
        },
        Err(error) => {
            // Handle error
            Err(MyError::ReqwestError(error))
        }
    }
}

pub enum MyError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error)
}