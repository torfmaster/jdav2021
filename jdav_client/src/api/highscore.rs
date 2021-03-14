use shared::{Highscore, UserAuth};
use yewtil::fetch::{FetchRequest, Json, MethodBody};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HighscoreRequest {
    pub auth: UserAuth,
}

impl HighscoreRequest {
    pub fn new(auth: UserAuth) -> Self {
        HighscoreRequest { auth }
    }
}

impl FetchRequest for HighscoreRequest {
    type RequestBody = ();
    type ResponseBody = Highscore;
    type Format = Json;

    fn url(&self) -> String {
        "/highscore".to_owned()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Put(&())
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
