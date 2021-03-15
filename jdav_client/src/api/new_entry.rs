use shared::{Kilometer, UserAuth};
use yewtil::fetch::{FetchRequest, Json, MethodBody};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KilometerRequest {
    pub auth: UserAuth,
    pub payload: Kilometer,
    pub kind: String,
}

impl KilometerRequest {
    pub fn new(distance: f32, auth: UserAuth, kind: String) -> Self {
        KilometerRequest {
            auth,
            payload: Kilometer {
                kilometers: distance,
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
        format!("/distanz/{}/{}", self.auth.name, self.kind)
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Put(&self.payload)
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![
            ("Content-Type".to_owned(), "application/json".to_owned()),
            ("Authorization".to_owned(), self.auth.to_basic_auth_header()),
        ]
    }

    fn use_cors(&self) -> bool {
        true
    }
}
