use crate::error::ErrorKind;
use bson::{from_bson, to_bson, Bson, Document};
use mongodb::{error::Error, options::FindOptions, Database};
use serde::{de::DeserializeOwned, Serialize};

// traits bound
pub(crate) trait MongoModel: Serialize + DeserializeOwned {
    const COLLECTION_NAME: &'static str;

    fn find(
        db: &Database,
        filter: impl Into<Option<Document>>,
        options: impl Into<Option<FindOptions>>,
    ) -> Result<Vec<Self>, ErrorKind> {
        db.collection(Self::COLLECTION_NAME)
            .find(filter, options)
            .map_err(ErrorKind::from)
            .map(|cursor| {
                cursor
                    .flat_map(|res| -> Result<_, Error> { Ok(res?) })
                    .flat_map(|doc| from_bson(Bson::from(doc)))
                    .collect::<Vec<Self>>()
            })
    }

    fn insert_self(&self, db: &Database) -> Result<Bson, ErrorKind> {
        if let Ok(Bson::Document(doc)) = to_bson(self) {
            db.collection(Self::COLLECTION_NAME)
                .insert_one(doc, None)
                .map_err(Into::into)
                .map(|res| res.inserted_id)
        } else {
            unreachable!();
        }
    }
}
