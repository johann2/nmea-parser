/*
Copyright 2021 Linus Eing

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use super::*;

/// MDA - Meteorological Composite
#[derive(Clone, Debug, PartialEq)]
pub struct MdaData {
    /// Barometric pressure, inches of mercury
    pub pressure_inches: Option<f64>,

    /// Barometric pressure, barsy
    pub pressure_bars: Option<f64>,

    /// Air temperature, degrees C
    pub air_temperature: Option<f64>,

    /// Water temperature, degrees C
    pub water_temperature: Option<f64>,

    /// Relative humidity, percent
    pub relative_humidity: Option<f64>,

    /// Absolute humidity, percent
    pub absolute_humidity: Option<f64>,

    /// Dew point, degrees C
    pub dew_point: Option<f64>,

    /// wind angle, 0 to 359 degrees, degrees true
    pub wind_angle_true: Option<f64>,

    /// wind angle, 0 to 359 degrees, degrees magnetic
    pub wind_angle_magnetic: Option<f64>,

    /// Wind speed - knots
    pub wind_speed_knots: Option<f64>,

    /// Wind speed - m/s
    pub wind_speed_ms: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxMDA - Meteorological Composite

pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();
    check_field_count(&split, 20)?;
    Ok(ParsedMessage::Mda(MdaData {
        pressure_inches: pick_number_field(&split, 1)?,
        pressure_bars: pick_number_field(&split, 3)?,
        air_temperature: pick_number_field(&split, 5)?,
        water_temperature: pick_number_field(&split, 7)?,
        relative_humidity: pick_number_field(&split, 9)?,
        absolute_humidity: pick_number_field(&split, 10)?,
        dew_point: pick_number_field(&split, 11)?,
        wind_angle_true: pick_number_field(&split, 13)?,
        wind_angle_magnetic: pick_number_field(&split, 15)?,
        wind_speed_knots: pick_number_field(&split, 17)?,
        wind_speed_ms: pick_number_field(&split, 19)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mda() {
        match NmeaParser::new().parse_sentence("$WIMDA,30.0615,I,1.0180,B,23.6,C,,,25.3,,2.5,C,,,,,,,,*4E") {
            Ok(ps) => match ps {
                ParsedMessage::Mda(mwv) => {
                    assert_eq!(mwv.pressure_inches, Some(30.0615));
                    assert_eq!(mwv.pressure_bars, Some(1.0180));
                    assert_eq!(mwv.air_temperature, Some(23.6));
                    assert_eq!(mwv.dew_point, Some(2.5));
                }
                _ => {
                    assert!(false);
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
}
