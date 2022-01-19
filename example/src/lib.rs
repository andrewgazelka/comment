use comment_derive::Comment;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Comment)]
struct Tester {
    /// hello there
    a: String,
}
