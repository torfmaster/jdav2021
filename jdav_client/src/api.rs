use shared::Kilometer;
use yewtil::fetch::{FetchRequest, Json, MethodBody};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BackendRequest {
    pub id: String,
    pub payload: Kilometer,
}

impl BackendRequest {
    pub fn new(distance: String) -> Self {
        BackendRequest {
            id: "wurst".to_owned(),
            payload: Kilometer {
                kilometers: distance.parse::<f32>().unwrap(),
            },
        }
    }
}

impl FetchRequest for BackendRequest {
    type RequestBody = Kilometer;
    type ResponseBody = String;
    type Format = Json;

    fn url(&self) -> String {
        format!("/distanz/someone/laufen")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Put(&self.payload)
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![("Content-Type".to_owned(), "application/json".to_owned())]
    }

    fn use_cors(&self) -> bool {
        true
    }
}
