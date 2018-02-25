use std::str::FromStr;
use std::fmt;

// b"<boinc_gui_rpc_request>\n<get_state/>\n</boinc_gui_rpc_request>\n\x03"

// XmlTags
struct XmlTags {
    open: String,
    close: String,
}

impl XmlTags {
    pub fn new(for_element: &str) -> XmlTags {
        XmlTags {
            open: format!("<{}>", for_element),
            close: format!("</{}>", for_element),
        }
    }
    pub fn open(&self) -> &str {
        &self.open
    }
    pub fn close(&self) -> &str {
        &self.close
    }
}

// XmlElement
struct XmlElement<'a> {
    name: &'a str,
    text: &'a str,
}

impl<'a> XmlElement<'a> {
    pub fn new(text: &'a str) -> XmlElement {
        let name = &text[text.find("<").unwrap() + 1..text.find(">").unwrap()];
        XmlElement { name, text }
    }
    pub fn get_element(&self, name: &str) -> Result<XmlElement, String> {
        let tags = XmlTags::new(name);
        match self.text.find(tags.open()) {
            Some(start_index) => match self.text.find(tags.close()) {
                Some(end_index) => Ok(XmlElement::new(&self.text[start_index..end_index + tags.close().len()])),
                None => Err(format!("closing tag {0} not found.", tags.close())),
            },
            None => Err(format!("opening tag {0} not found.", tags.open())),
        }
    }
    pub fn get_value<T>(&self) -> Result<T, String>
    where
        T: FromStr,
        T::Err: fmt::Debug
    {
        let tags = XmlTags::new(self.name);
        match self.text.find(tags.open()) {
            Some(start_index) => match self.text.find(tags.close()) {
                Some(end_index) => Ok(T::from_str(
                    &self.text[start_index + tags.open().len()..end_index],
                ).unwrap()),
                None => Err(format!("closing tag {0} not found.", tags.close())),
            },
            None => Err(format!("opening tag {0} not found.", tags.open())),
        }
    }
}

// Tests
mod tests {
    use super::*;

    #[test]
    fn get_value() {
        let xml = XmlElement::new(
            "<string_element>string</string_element>\n
             <int_element>42</int_element>\n
             <outer_element>\n
                <inner_element>inner</inner_element>\n
             </outer_element>\n",
        );

        let string_element: String = xml.get_element("string_element").unwrap().get_value().unwrap();
        assert_eq!("string", string_element.as_str());
    }
}
