use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Paginate: Sized {
    fn paginate(self, start: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, start: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            count: DEFAULT_PER_PAGE,
            start,
        }
    }
}

pub const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    start: i64,
    count: i64,
}

impl<T> Paginated<T> {
    pub fn  count(self, count: i64) -> Self {
        Paginated { count, ..self }
    }

    pub fn load_and_count<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
        where
            Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        Ok((records, total))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
    where
        T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.count)?;
        out.push_sql(" OFFSET ");
        let offset = self.start;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}