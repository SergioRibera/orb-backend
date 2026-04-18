use sqlx::{Pool, Postgres};
use std::marker::PhantomData;

pub struct PgRepository<T> {
    pub pool: Pool<Postgres>,
    _marker: PhantomData<T>,
}

impl<T> PgRepository<T> {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }
}
