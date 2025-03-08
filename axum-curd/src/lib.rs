mod curd;
pub use curd::*;

pub use axum_curd_macro::{impl_all, impl_curd, route_all, route_curd};

#[cfg(test)]
mod tests {
    use crate::impl_curd;
    use rest_model::rest_model;
    use rest_model_postgres::Db;
    use serde::{Deserialize, Serialize};

    use crate::curd;
    const DB_NAME: &str = "mydb.public";
    const TABLE_NAME: &str = "guest";
    

    #[rest_model(db(Db, DB_NAME, TABLE_NAME), with(all))]
    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct Guest {
        name: String,
        age: i32,
    }

    impl_curd!(Guest, get, get_with_id, put, patch, delete);
    // impl curd::Get<Guest> for Guest {}
    // impl curd::GetWithId<Guest> for Guest {}
    // impl curd::Put<Guest> for Guest {}
    // impl curd::Patch<Guest> for Guest {}
    // impl curd::Delete<Guest> for Guest {}

    #[test]
    fn it_works() {
        // Guest::route
    }
}
