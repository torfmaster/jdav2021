use shared::{KilometerEntry, UserAuth};
use yewtil::fetch::{FetchRequest, Json, MethodBody};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EntriesRequest {
    pub auth: UserAuth,
}

impl EntriesRequest {
    pub fn new(auth: UserAuth) -> Self {
        EntriesRequest { auth }
    }
}

impl FetchRequest for EntriesRequest {
    type RequestBody = ();
    type ResponseBody = shared::Entries;
    type Format = Json;

    fn url(&self) -> String {
        format!("/entries/{}", self.auth.name)
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
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
