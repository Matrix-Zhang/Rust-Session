mod person;

pub(crate) mod prelude {
    pub(crate) use super::person::*;
    pub(crate) use crate::mongo::*;
}
