use shared::{KilometerEntry, UserAuth};
use yewtil::fetch::{FetchRequest, Json, MethodBody};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KilometerEditRequest {
    pub auth: UserAuth,
    pub payload: KilometerEntry,
}

impl KilometerEditRequest {
    pub fn new(auth: UserAuth, entry: KilometerEntry) -> Self {
        KilometerEditRequest {
            auth,
            payload: entry,
        }
    }
}

impl FetchRequest for KilometerEditRequest {
    type RequestBody = KilometerEntry;
    type ResponseBody = String;
    type Format = Json;

    fn url(&self) -> String {
        format!("/edit/{}", self.auth.name)
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
