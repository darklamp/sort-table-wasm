use std::cmp::Ordering;

use rayon::prelude::*;
use wasm_bindgen::prelude::*;
extern crate serde_json;


#[macro_use]
extern crate serde;

#[derive(Serialize, Deserialize, Eq, Ord)]
pub struct Row {
    name: String,
    questionnaireId: i32,
    datetime: String,
    image: String,
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.questionnaireId == other.questionnaireId
    }
}

#[derive(Serialize, Deserialize, Eq)]
pub struct RowDate {
    datetime: String,
    questionnaireId: i32,
    name: String, 
    image: String,
}

impl Ord for RowDate {
    fn cmp(&self, other: &Self) -> Ordering {
        if js_sys::Date::new(&JsValue::from_serde(&self.datetime).unwrap()).get_time() - js_sys::Date::new(&JsValue::from_serde(&other.datetime).unwrap()).get_time()  > 0.0 {
            Ordering::Greater
        }
        else {
            Ordering::Less
        }
    }
}

impl PartialOrd for RowDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RowDate {
    fn eq(&self, other: &Self) -> bool {
        self.datetime == other.datetime
    }
}

#[wasm_bindgen]
pub fn sort(data: JsValue, reverse: bool, by_date: bool) -> JsValue {
    match by_date {
        true => {
            let mut parsed_data: Vec<RowDate> = data.into_serde().unwrap();
            parsed_data.par_sort_unstable();
            if reverse {
                parsed_data.reverse();
            }
            return JsValue::from_serde(&parsed_data).unwrap();
        }
        _ => {
            let mut parsed_data: Vec<Row> = data.into_serde().unwrap();
            parsed_data.par_sort_unstable();
            if reverse {
                parsed_data.reverse();
            }
            return JsValue::from_serde(&parsed_data).unwrap();
        }
    }
}
