use crate::database::establish_connection;

use diesel::prelude::*;

pub fn has_project(project: String) -> bool {
    use diesel::sql_types::VarChar;

    #[derive(QueryableByName)]
    struct Project {
        #[diesel(sql_type = VarChar)]
        name: String,
    }

    let mut conn = establish_connection();
    let results = diesel::sql_query(r#"SELECT DISTINCT project AS name FROM frames"#)
        .load::<Project>(&mut conn)
        .expect("Query failed");

    for result in results {
        if result.name == project {
            return true;
        }
    }

    false
}
