extern crate xmlparser as xml;

type Range = ::std::ops::Range<usize>;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Declaration(&'a str, Option<&'a str>, Option<bool>, Range),
    PI(&'a str, Option<&'a str>, Range),
    Comment(&'a str, Range),
    DtdStart(&'a str, Option<ExternalId<'a>>, Range),
    EmptyDtd(&'a str, Option<ExternalId<'a>>, Range),
    EntityDecl(&'a str, EntityDefinition<'a>, Range),
    DtdEnd(Range),
    ElementStart(&'a str, &'a str, Range),
    Attribute(&'a str, &'a str, &'a str, Range),
    ElementEnd(ElementEnd<'a>, Range),
    Text(&'a str, Range),
    Whitespaces(&'a str, Range),
    Cdata(&'a str, Range),
    Error(String),
}

#[derive(PartialEq, Debug)]
pub enum ElementEnd<'a> {
    Open,
    Close(&'a str, &'a str),
    Empty,
}

#[derive(PartialEq, Debug)]
pub enum ExternalId<'a> {
    System(&'a str),
    Public(&'a str, &'a str),
}

#[derive(PartialEq, Debug)]
pub enum EntityDefinition<'a> {
    EntityValue(&'a str),
    ExternalId(ExternalId<'a>),
}

#[macro_export]
macro_rules! test {
    ($name:ident, $text:expr, $($token:expr),*) => (
        #[test]
        fn $name() {
            let mut p = xml::Tokenizer::from($text);
            $(
                let t = p.next().unwrap();
                assert_eq!(to_test_token(t), $token);
            )*
            assert!(p.next().is_none());
        }
    )
}

#[inline(never)]
pub fn to_test_token(token: Result<xml::Token, xml::Error>) -> Token {
    match token {
        Ok(xml::Token::Declaration { version, encoding, standalone, span }) => {
            Token::Declaration(
                version.to_str(),
                encoding.map(|v| v.to_str()),
                standalone,
                span_to_range(span),
            )
        }
        Ok(xml::Token::ProcessingInstruction { target, content, span }) => {
            Token::PI(
                target.to_str(),
                content.map(|v| v.to_str()),
                span_to_range(span),
            )
        }
        Ok(xml::Token::Comment { text, span }) => Token::Comment(text.to_str(), span_to_range(span)),
        Ok(xml::Token::DtdStart { name, external_id, span }) => {
            Token::DtdStart(
                name.to_str(),
                external_id.map(|v| to_test_external_id(v)),
                span_to_range(span),
            )
        }
        Ok(xml::Token::EmptyDtd { name, external_id, span }) => {
            Token::EmptyDtd(
                name.to_str(),
                external_id.map(|v| to_test_external_id(v)),
                span_to_range(span),
            )
        }
        Ok(xml::Token::EntityDeclaration { name, definition, span }) => {
            Token::EntityDecl(
                name.to_str(),
                match definition {
                    xml::EntityDefinition::EntityValue(name) => {
                        EntityDefinition::EntityValue(name.to_str())
                    }
                    xml::EntityDefinition::ExternalId(id) => {
                        EntityDefinition::ExternalId(to_test_external_id(id))
                    }
                },
                span_to_range(span),
            )
        }
        Ok(xml::Token::DtdEnd { span }) => Token::DtdEnd(span_to_range(span)),
        Ok(xml::Token::ElementStart { prefix, local, span }) => {
            Token::ElementStart(prefix.to_str(), local.to_str(), span_to_range(span))
        }
        Ok(xml::Token::Attribute { prefix, local, value, span }) => {
            Token::Attribute(prefix.to_str(), local.to_str(), value.to_str(), span_to_range(span))
        }
        Ok(xml::Token::ElementEnd { end, span }) => {
            Token::ElementEnd(
                match end {
                    xml::ElementEnd::Open => ElementEnd::Open,
                    xml::ElementEnd::Close(prefix, local) => {
                        ElementEnd::Close(prefix.to_str(), local.to_str())
                    }
                    xml::ElementEnd::Empty => ElementEnd::Empty,
                },
                span_to_range(span)
            )
        }
        Ok(xml::Token::Text { text }) => Token::Text(text.to_str(), span_to_range(text)),
        Ok(xml::Token::Whitespaces { text }) => Token::Whitespaces(text.to_str(), span_to_range(text)),
        Ok(xml::Token::Cdata { text, span }) => Token::Cdata(text.to_str(), span_to_range(span)),
        Err(ref e) => Token::Error(e.to_string()),
    }
}

fn to_test_external_id(id: xml::ExternalId) -> ExternalId {
    match id {
        xml::ExternalId::System(name) => {
            ExternalId::System(name.to_str())
        }
        xml::ExternalId::Public(name, value) => {
            ExternalId::Public(name.to_str(), value.to_str())
        }
    }
}

fn span_to_range(span: xml::StrSpan) -> Range {
    span.start()..span.end()
}
