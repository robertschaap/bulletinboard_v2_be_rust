extern crate postgres;

use postgres::{Connection, TlsMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<i32>,
    pub title: String,
    pub name: String,
    pub avatar: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct Comments {
    pub comments: Vec<Comment>
}

pub fn get_comments(offset: Option<String>, sort: Option<String>) -> Vec<Comment> {
    let offset_number: i32 = match offset {
        Some(n) => n.parse::<i32>().unwrap_or(0),
        None => 0,
    };

    let sort_direction: String = match sort {
        Some(ref s) => match &s[..] {
            "asc" => "asc".to_string(),
            _ => "desc".to_string(),
        },
        None => "desc".to_string(),
    };

    let conn = match Connection::connect("postgres://robertschaap@localhost/bulletinboard", TlsMode::None) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };

    let sql = format!(
        "SELECT * FROM posts LIMIT={limit} OFFSET={offset} SORT={sort}",
        limit = 4,
        offset = offset_number,
        sort = sort_direction,
    );

    let mut comments: Vec<Comment> = Vec::new();

    for row in &conn.query(&sql, &[]).unwrap() {
        comments.push(Comment {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            avatar: row.get("avatar"),
            name: row.get("name"),
        });
    }

    return comments;
}

pub fn create_comment(comment: Comment) -> Comment {
    let conn = match Connection::connect("postgres://robertschaap@localhost/bulletinboard", TlsMode::None) {
        Ok(r) => r,
        Err(_) => return Comment {
            id: Some(1),
            title: "".to_string(),
            name: "".to_string(),
            body: "".to_string(),
            avatar: "".to_string(),
        },
    };

    let sql = "INSERT INTO posts (name, title, body, avatar) VALUES ($1, $2, $3, $4)";

    let result = conn.execute(sql, &[
        &"name",
        &"title",
        &"body",
        &"avatar",
    ]).unwrap();

    println!("{}", result);

    return comment;
}
