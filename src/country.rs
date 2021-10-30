use std::fmt;
use std::str::FromStr;

use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, FromFormField, ValueField};

use rocket::serde::de::Visitor;
use rocket::serde::{de, Deserializer};

#[derive(Debug)]
pub struct Country(pub(crate) celes::Country);

impl Country {
    pub fn code(&self) -> &str {
        self.0.alpha2
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Country {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        celes::Country::from_str(field.value)
            .map(Country)
            .map_err(|_err| form::Error::validation("invalid country code").into())
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let limit = 2.bytes();

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_string().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        celes::Country::from_str(&bytes)
            .map(Country)
            .map_err(|_err| form::Error::validation("invalid country code").into())
    }
}

pub(crate) fn country_from_code<'de, D>(deserializer: D) -> Result<Country, D::Error>
where
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct CountryCodeVisitor;

    impl<'de> Visitor<'de> for CountryCodeVisitor {
        type Value = Country;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("country code")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).map(Country).unwrap())
        }
    }

    deserializer.deserialize_str(CountryCodeVisitor)
}
