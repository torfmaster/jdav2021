use shared::{Kilometer, UserAuth};
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

#[derive(Debug, Clone, Default)]
pub struct RegisterRequest {
    pub payload: UserAuth,
}

impl PartialEq for RegisterRequest {
    fn eq(&self, other: &Self) -> bool {
        self.payload.name == other.payload.name && self.payload.pass == other.payload.pass
    }
}

impl RegisterRequest {
    pub fn new(username: String, password: String) -> Self {
        RegisterRequest {
            payload: {
                UserAuth {
                    name: username,
                    pass: password,
                }
            },
        }
    }
}

impl FetchRequest for RegisterRequest {
    type RequestBody = UserAuth;
    type ResponseBody = bool;
    type Format = Json;

    fn url(&self) -> String {
        "/createuser".to_string()
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
