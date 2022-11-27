use hard_xml::XmlRead;
use std::borrow::Cow;

#[derive(Debug, XmlRead)]
#[xml(tag = "Attributes")]
struct Attributes<'t> {
    #[xml(attr = "field1", attr = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Cdata")]
struct Cdata<'t> {
    #[xml(text, cdata, cdata)]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Child")]
struct FlattenText<'t> {
    #[xml(flatten_text = "field1", flatten_text = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "Text")]
struct Text<'t> {
    #[xml(text, text)]
    field: Cow<'t, str>,
}

fn main() {}
