use std::fmt::{self, Write};

use crate::docs::Docs;
use crate::fields::Fields;
use crate::formatter::Formatter;

use crate::r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    fields: Fields,
    annotations: Vec<String>,
    doc: Option<Docs>
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: &str) -> Self {
        Variant {
            name: name.to_string(),
            fields: Fields::Empty,
            annotations: Vec::new(),
            doc: None,
        }
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Set the variant documentation.
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.doc = Some(Docs::new(docs));
        self
    }

    /// Add an anotation to the variant.
    pub fn annotation(&mut self, annotation: &str) -> &mut Self {
        self.annotations.push(annotation.to_string());
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for a in &self.annotations {
            write!(fmt, "{}", a)?;
            write!(fmt, "\n")?;
        }
        if let Some(ref doc) = self.doc {
            doc.fmt(fmt);
        }
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        write!(fmt, ",\n")?;

        Ok(())
    }
}
