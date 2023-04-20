use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use futures_util::io::AsyncWriteExt;

#[tokio::main]
pub async fn db_connect() -> mongodb::error::Result<()> {
  let uri = "mongodb://localhost:27017";
  let mut client_options =
      ClientOptions::parse(uri)
          .await?;
  let client = Client::with_options(client_options)?;
  let db =  client.database("test123");

  let bucket = db.gridfs_bucket(None);

  let bytes = vec![0u8; 100];
  let mut upload_stream = bucket.open_upload_stream("a.txt", None);
  upload_stream.write_all(&bytes[..]).await?;
  upload_stream.close().await?;
  Ok(())
}