#![allow(unused)]
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response,Session};

#[tokio::main]
async fn main()->Result<()>{
    // let ds = Datastore::new("http://localhost:8080", "admin", "admin").await?;
    let ds= Datastore::new("memory").await?;
    let ses =Session::for_db("my_ns","my_db");

    //ccreate a table
    let sql ="CREATE task:1 SET title = 'Task 01', priority=10";
    let res = ds.execute(sql, &ses, None, false).await?;
    println!("{:?}", res);

    Ok(())
}