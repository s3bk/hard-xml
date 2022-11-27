use hard_xml::XmlRead;

#[derive(Clone, Copy)]
struct One {}

#[derive(XmlRead)]
union Data {
    one: One,
}

fn main() {}
