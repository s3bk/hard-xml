use hard_xml::XmlError;
use hard_xml::XmlRead;
use hard_xml::XmlResult;
use std::borrow::Cow;

#[derive(Debug, PartialEq, XmlRead)]
#[xml(strict(unknown_attribute, unknown_element), tag = "root")]
struct Root<'a> {
    #[xml(flatten_text = "data")]
    data: Cow<'a, str>,

    #[xml(attr = "version")]
    version: Cow<'a, str>,
}

#[test]
fn test() -> XmlResult<()> {
    assert_eq!(
        Root::from_str(r#"<root version="2.0"><data>This is some test data!</data></root>"#)?,
        Root {
            data: "This is some test data!".into(),
            version: "2.0".into()
        }
    );

    Ok(())
}

#[test]
fn test_unknown_attribute() -> XmlResult<()> {
    assert!(matches!(
        Root::from_str(r#"<root attr="test" version="2.0"><data>This is some test data!</data></root>"#),
        Err(XmlError::UnknownField { name, field }) if name == "Root" && field == "attr"
    ));

    Ok(())
}

#[test]
fn test_unknown_child() -> XmlResult<()> {
    assert!(matches!(
        Root::from_str(r#"<root version="2.0"><data>This is some test data!</data><child /></root>"#),
        Err(XmlError::UnknownField { name, field }) if name == "Root" && field == "child"
    ));

    Ok(())
}
