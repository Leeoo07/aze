use crate::database::establish_connection;

use diesel::prelude::*;

pub fn has_tag(tag: String) -> bool {
    use diesel::sql_types::VarChar;

    #[derive(QueryableByName)]
    struct Tag {
        #[diesel(sql_type = VarChar)]
        name: String,
    }

    let mut conn = establish_connection();
    let results = diesel::sql_query(r#"SELECT DISTINCT tags AS name FROM frames"#)
        .load::<Tag>(&mut conn)
        .expect("Query failed");

    for result in results {
        let tags_json: Vec<String> =
            serde_json::from_str(&result.name).expect("Tags are not valid json");
        for tag_json in tags_json {
            if tag_json == tag {
                return true;
            }
        }
    }

    return false;
}
