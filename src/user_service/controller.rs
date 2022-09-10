use poem_openapi::{OpenApi, payload::PlainText};

pub struct UserRouter;

#[OpenApi]
impl UserRouter {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }

    // #[oai(path = "/user", method = "get")]
    // async fn index(&self) -> PlainText<&'static str> {
    //     PlainText("Hello World")
    // }

    // #[oai(path = "/user", method = "get")]
    // async fn index(&self) -> PlainText<&'static str> {
    //     PlainText("Hello World")
    // }

    // #[oai(path = "/user", method = "get")]
    // async fn index(&self) -> PlainText<&'static str> {
    //     PlainText("Hello World")
    // }
}