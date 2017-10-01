extern crate serde_json;

use super::ProbStart;

lazy_static! {
    pub static ref P: ProbStart = {
        let data = r#"{"B": -0.26268660809250016, "E": -3.14e+100, "M": -3.14e+100, "S": -1.4652633398537678}"#;
        let v: ProbStart = serde_json::from_str(data).unwrap();
        println!("{:?}", v);
        v
    };
}