use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::person_diesel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PersonDiesel {
    pub name: String,
    pub age: i32,
}
