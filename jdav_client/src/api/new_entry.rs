use shared::Kilometer;
use yewtil::fetch::{FetchRequest, Json, MethodBody};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KilometerRequest {
    pub id: String,
    pub payload: Kilometer,
    pub kind: String,
}

impl KilometerRequest {
    pub fn new(distance: String, username: String, kind: String) -> Self {
        KilometerRequest {
            id: username,
            payload: Kilometer {
                kilometers: distance.parse::<f32>().unwrap(),
            },
            kind,
        }
    }
}

impl FetchRequest for KilometerRequest {
    type RequestBody = Kilometer;
    type ResponseBody = String;
    type Format = Json;

    fn url(&self) -> String {
        format!("/distanz/{}/{}", self.id, self.kind)
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
