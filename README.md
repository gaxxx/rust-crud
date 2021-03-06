# How to build a rust service using actix & postgresql

## prequesition
* postgresql
* rust

## Setup environment

* clion
* postgrsql: grant a member, generate connection string
* rust: 
    * using stable channel
    * install disel


## Initial

* create a dummy projects


## Add json support

* add serde



## setup diesel
add to cargo.toml
```
diesel = { version = "1.0.0", features = ["postgres"] }
dotenv = "0.9.0"
r2d2-diesel = "1.0.0"
```

```
cargo install diesel_cli --no-default-features --features postgres
```

## add db support
add line to hda_file if needed
```
host    all             all             127.0.0.1/32            md5
```
setup a postgresql db

```
sudo -u postgres psql
ALTER USER postgres PASSWORD 'password';
create database heroes;
```

the connection string would be "postgres://postgres:password@127.0.0.1/heroes"

```
echo DATABASE_URL=postgres://postgres:password@127.0.0.1/heroes > .env
disel setup 
// generage module tables
diesel migration generate heroes
```

setup users model & table
update up.sql
```
create table users (
id serial primary key,
name varchar not null  default '',
identity varchar not null default '',
hometown varchar not null default '',
age int not null default 0
);
```

```
diesel migration run
diesel migration redo
```

## update crud

```
// create 
diesel::insert_into(schema::users::table)
        .values(hero.into_inner())
// read
users.filter(id.eq(update_id.into_inner()))
        .first::<hero::Hero>(&*db.get());
// update
diesel::update(users.filter(id.eq(update_id.into_inner())))
        .set(hero.into_inner())
// delete
diesel::delete(users.filter(id.eq(update_id.into_inner())))


//list
users.load::<hero::Hero>(&*db.get()).unwrap();
```

## pagination

make query pagable
```
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
```

how to use
```
let query = users.order(id.asc());
let query = query.paginate(req.start.unwrap_or(0).into());
let all_users : Vec<_> = query.load::<(hero::Hero, i64)>(&*db.get()).unwrap();
web::Json(all_users.iter().map(|v| v.0.clone()).collect::<Vec<_>>())
```










