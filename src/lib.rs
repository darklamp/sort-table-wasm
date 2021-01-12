use std::cmp::Ordering;

use rayon::prelude::*;
use wasm_bindgen::prelude::*;
extern crate serde_json;
extern crate console_error_panic_hook;
use std::panic;


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


#[wasm_bindgen]
pub fn sort(data: JsValue, reverse: bool, by_date: bool) -> JsValue {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    let order_by_date = |a : &Row, b : &Row| {
        if js_sys::Date::new(&JsValue::from_serde(&a.datetime).unwrap()).get_time() - js_sys::Date::new(&JsValue::from_serde(&b.datetime).unwrap()).get_time()  > 0.0 {
            Ordering::Greater
        }
        else {
            Ordering::Less
        }
    };
    let mut parsed_data: Vec<Row> = data.into_serde().unwrap();

    match by_date {
        true => {
            parsed_data.par_sort_unstable_by(order_by_date);
            
        }
        _ => {
            parsed_data.par_sort_unstable();
        }
    }
    if reverse {
        parsed_data.reverse();
    }
    JsValue::from_serde(&parsed_data).unwrap()
}
