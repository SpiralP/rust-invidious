pub use error_chain::bail;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        ParseFloatError(::std::num::ParseFloatError);
        ParseIntError(::std::num::ParseIntError);
        Reqwest(reqwest::Error);
        SerdeJson(serde_json::Error);
    }

    errors {
        Invidious(message: String) {
            description("invidious api error")
            display("invidious error: {}", message)
        }
    }
}
