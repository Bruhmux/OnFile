pub trait ToSQLX {
    fn to_sqlx(&self) -> Self;
}

impl ToSQLX for uuid::Uuid {
    fn to_sqlx(&self) -> sqlx::types::Uuid {
        sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128())
    }
}
