use std::error;

use crate::connection::CityInfo;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub city: Vec<&'static str>,
    pub indicator : usize,
    pub data: Option<CityInfo>
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            city: vec!["Bucharest", "London", "Paris", "New York", "Tokyo", "Sydney", "Berlin", "Rome", "Madrid", "Moscow",
             "Beijing", "Cairo", "Rio de Janeiro", "Buenos Aires", "Mexico City", "Los Angeles", "Toronto", "Vancouver",
              "Montreal", "Chicago", "Houston", "Miami", "Dallas", "Seattle", "Las Vegas", "San Francisco", "Boston",
               "Washington", "Philadelphia", "Atlanta", "Detroit", "Phoenix", "Minneapolis", "Denver", "Salt Lake City",
                "Honolulu", "Anchorage", "Havana", "Kingston", "Santo Domingo", "San Juan", "Bogota", "Lima", "Santiago",
                 "Buenos Aires", "Sao Paulo", "Rio de Janeiro", "Caracas", "Bogota", "Quito", "Lima", "La Paz", "Santiago",
                  "Buenos Aires", "Montevideo", "Asuncion", "Brasilia", "Sao Paulo", "Rio de Janeiro", "Belo Horizonte"],
            indicator: 0,
            data: None
        }
    }
}
