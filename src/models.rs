use rocket_db_pools::diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::_public::domains)]
pub struct AddDomainOwnership {
    pub domain: String,
    pub owned_by: uuid::Uuid,
}
