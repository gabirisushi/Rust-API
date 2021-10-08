/*
 * Data access object (DAO) - Provides an abstract interface to some type of database or other persistence mechanism.
*/

extern crate contract;
extern crate tokio_postgres;
extern crate uuid;

use uuid::Uuid;
use contract::Edges;
use tokio_postgres::{NoTls};
use tokio;


pub async fn connect() -> Option<tokio_postgres::Client> {

  let (client,conn) =
        tokio_postgres::connect("host=127.0.0.1 user=admin password=admin dbname=mydb port=5432", NoTls).
        await.unwrap();
  tokio::spawn(async move {
      if let Err(e) = conn.await {
          eprintln!("connection error: {}", e);
      }
  });
  return Some(client);
}

pub async fn get_edge_by_id(id:&String) -> Option<Edges> {
  let client = connect().await.unwrap();
  let rows = &client.query("SELECT id::text,url,'desc' FROM edges where id::text=$1", &[&id]).await.unwrap();
  let row = rows.get(0).unwrap();

    let edges = Edges { 
        id:   Uuid::parse_str(row.get(0)).unwrap(),
        desc: row.get(2),
        url:  row.get(1),
    };

  return Some(edges);
}

pub async fn delete_edge_by_id(id:&String) -> Option<bool> {
  let client = connect().await.unwrap();
  let _rows = &client.query("DELETE FROM edges where id::text=$1", &[&id]).await.unwrap();
  return Some(true);
}

pub async fn insert_edges(url:&String,desc:&String) -> Option<Edges> {

  let id_uuid = uuid::Uuid::new_v4();
  let _id = format!("{}", id_uuid);
  let client = connect().await.unwrap();

  let  _rows = &client.query("INSERT INTO edges VALUES(uuid_in(($1::text)::cstring),$2,$3) RETURNING id",&[&_id, &desc, &url]).await.unwrap();

  let edges = Edges {
    id:   id_uuid,
    desc: String::from(desc),
    url: String::from(url),
  };

  return Some(edges);
}

pub async fn list_edges() -> Option<Vec<Edges>> {
  let client = connect().await.unwrap();
  let mut vec_edges = Vec::new();  
  let rows = &client.query("SELECT id::text,url,'desc' FROM edges", &[]).await.unwrap();

  for row in rows {
    let edges = Edges { 
        id:   Uuid::parse_str(row.get(0)).unwrap(),
        desc: row.get(1),
        url:  row.get(2),
    };
    vec_edges.push(edges);
  }
  return Some(vec_edges);
}

pub async fn mocked_list_edges() -> Option<Vec<Edges>> {
  let mut vec_edges = Vec::new();  
  vec_edges.push(Edges {
    id: Uuid::parse_str("c368c393-ccb5-f134-213e-2c50730e75ea").unwrap(),
    desc: String::from("Edge South Brasil - POA 1"),
    url: String::from("172.1.1.1.1")
  });
  vec_edges.push(Edges {
    id: Uuid::parse_str("c368c393-ccb5-f134-213e-2c50730e75eb").unwrap(),
    desc: String::from("Edge South Brasil - POA 2"),
    url: String::from("172.1.1.1.1")
  });
  return Some(vec_edges);
}