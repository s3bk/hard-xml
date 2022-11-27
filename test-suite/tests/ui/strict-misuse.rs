use hard_xml::XmlRead;

// TODO this compiles. Should it fail instead?
#[derive(Clone, Copy, XmlRead)]
#[xml(strict, tag = "Bare")]
struct Bare {}

// TODO this compiles. Should it fail instead?
#[derive(Clone, Copy, XmlRead)]
#[xml(strict = "all", tag = "KeyValue")]
struct KeyValue {}

#[derive(Clone, Copy, XmlRead)]
#[xml(strict(data), tag = "UnsupportedArgument")]
struct UnsupportedArgument {}

#[derive(Clone, Copy, XmlRead)]
#[xml(strict(data = "unknown_attributes"), tag = "UnsupportedMeta")]
struct UnsupportedMeta {}

fn main() {}
