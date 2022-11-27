use hard_xml::XmlRead;
use std::borrow::Cow;

#[derive(Debug, XmlRead)]
#[xml(tag = "AttributeCdata")]
struct AttributeCdata<'t> {
    #[xml(attr = "field1", cdata)]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "AttributeChild")]
struct AttributeChild<'t> {
    #[xml(attr = "field1", child = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "AttributeFlattenText")]
struct AttributeFlattenText<'t> {
    #[xml(attr = "field1", flatten_text = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "AttributeText")]
struct AttributeText<'t> {
    #[xml(attr = "field1", text)]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "CdataAttribute")]
struct CdataAttribute<'t> {
    #[xml(cdata, attr = "field1")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "CdataChild")]
struct CdataChild<'t> {
    #[xml(cdata, child = "field1")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "ChildAttribute")]
struct ChildAttribute<'t> {
    #[xml(child = "field1", attr = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "ChildCdata")]
struct ChildCdata<'t> {
    #[xml(child = "field1", cdata)]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "ChildFlattenText")]
struct ChildFlattenText<'t> {
    #[xml(child = "field1", flatten_text = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "ChildText")]
struct ChildText<'t> {
    #[xml(child = "field1", text)]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "FlattenTextAttribute")]
struct FlattenTextAttribute<'t> {
    #[xml(flatten_text = "field1", attr = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "FlattenTextChild")]
struct FlattenTextChild<'t> {
    #[xml(flatten_text = "field1", child = "field2")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "FlattenTextText")]
struct FlattenTextText<'t> {
    #[xml(flatten_text = "field1", text)]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "TextAttribute")]
struct TextAttribute<'t> {
    #[xml(text, attr = "field1")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "TextChild")]
struct TextChild<'t> {
    #[xml(text, child = "field1")]
    field: Cow<'t, str>,
}

#[derive(Debug, XmlRead)]
#[xml(tag = "TextFlattenText")]
struct TextFlattenText<'t> {
    #[xml(text, flatten_text = "field1")]
    field: Cow<'t, str>,
}

fn main() {}
