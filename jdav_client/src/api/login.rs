use shared::UserAuth;
use yewtil::fetch::{FetchRequest, Json, MethodBody};

#[derive(Debug, Clone, Default)]
pub struct LoginRequest {
    pub payload: UserAuth,
}

impl PartialEq for LoginRequest {
    fn eq(&self, other: &Self) -> bool {
        self.payload.name == other.payload.name && self.payload.pass == other.payload.pass
    }
}

impl LoginRequest {
    pub fn new(username: String, password: String) -> Self {
        LoginRequest {
            payload: {
                UserAuth {
                    name: username,
                    pass: password,
                }
            },
        }
    }
}

impl FetchRequest for LoginRequest {
    type RequestBody = UserAuth;
    type ResponseBody = String;
    type Format = Json;

    fn url(&self) -> String {
        "/authenticate".to_string()
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
