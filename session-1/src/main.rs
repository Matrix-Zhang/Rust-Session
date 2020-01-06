#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

mod error;
mod model;
mod mongo;

use actix_web::{middleware, web, App, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client, Database};
use serde_json::Value;

use crate::{error::ErrorKind, model::prelude::*};

async fn show_person(db: web::Data<Database>) -> impl Responder {
    Person::find(&db, None, None).map(web::Json)
}

async fn new_person(
    (person, db): (web::Json<Person>, web::Data<Database>),
) -> Result<web::Json<Value>, ErrorKind> {
    person
        .insert_self(&db)
        .map(|id| web::Json(json!({ "id": id })))
}

fn setup_mongodb() -> Result<Database, ErrorKind> {
    ClientOptions::parse("mongodb://localhost:27017")
        .and_then(|mut options| {
            options.app_name = Some("session-1".to_string());
            let client = Client::with_options(options)?;
            Ok(client)
        })
        .map_err(Into::into)
        .map(|client| client.database("session-1"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db = web::Data::new(setup_mongodb().unwrap());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db.clone())
            .service(
                web::scope("/person").service(
                    web::resource("/")
                        .route(web::get().to(show_person))
                        .route(web::post().to(new_person)),
                ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};
    use bson::{from_bson, Bson};

    #[actix_rt::test]
    async fn test_new_person() -> Result<(), ErrorKind> {
        #[derive(Deserialize, Serialize)]
        struct NewPersonResp {
            id: bson::oid::ObjectId,
        }

        let db = web::Data::new(setup_mongodb().unwrap());

        let mut app = test::init_service(
            App::new()
                .app_data(db)
                .service(web::resource("/person").route(web::post().to(new_person))),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/person")
            .set_json(&Person::new("matrix", 30, Sex::Male))
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => {
                let json = serde_json::from_slice::<Value>(&bytes).unwrap();
                from_bson::<NewPersonResp>(Bson::from(json)).unwrap();
            }
            _ => panic!("Response error"),
        };

        Ok(())
    }
}
