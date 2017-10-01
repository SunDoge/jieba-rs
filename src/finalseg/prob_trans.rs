extern crate serde_json;

use super::ProbTrans;

lazy_static! {
    pub static ref P: ProbTrans = {
        let data = r#"{
                        "B": {
                            "E": -0.51082562376599,
                            "M": -0.916290731874155
                        },
                        "E": {
                            "B": -0.5897149736854513,
                            "S": -0.8085250474669937
                        },
                        "M": {
                            "E": -0.33344856811948514,
                            "M": -1.2603623820268226
                        },
                        "S": {
                            "B": -0.7211965654669841,
                            "S": -0.6658631448798212
                        }
                      }"#;
        let v = serde_json::from_str(data).unwrap();
        // println!("{:?}", v);
        v
    };
}
