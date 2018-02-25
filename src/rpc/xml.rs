use std::str::FromStr;
use std::fmt;
use std::marker::Sized;

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

trait GetElement<'a> {
    fn get_xml_text(&self) -> &'a str;

    fn get_element(&'a self, element_name: &'a str) -> Result<XmlElement, String> {
        match self.extract_element_text(element_name) {
            Ok(element_text) => Ok(XmlElement::new(element_text)),
            Err(e) => Err(e),
        }
    }

    fn extract_element_text(&self, element_name: &'a str) -> Result<&'a str, String> {
        let tags = XmlTags::new(element_name);
        let xml_text = self.get_xml_text();
        match xml_text.find(tags.open()) {
            Some(start_index) => match xml_text.find(tags.close()) {
                Some(end_index) => Ok(&xml_text[start_index..end_index + tags.close().len()]),
                None => Err(format!("closing tag {0} not found.", tags.close())),
            },
            None => Err(format!("opening tag {0} not found.", tags.open())),
        }
    }
}

// XmlElement
struct XmlElement<'a> {
    name: &'a str,
    xml_text: &'a str,
}

impl<'a> GetElement<'a> for XmlElement<'a> {
    fn get_xml_text(&self) -> &'a str {
        self.xml_text
    }
}

impl<'a> XmlElement<'a> {
    pub fn new(xml_text: &'a str) -> XmlElement {
        let name = &xml_text[xml_text.find("<").unwrap() + 1..xml_text.find(">").unwrap()];
        XmlElement { name, xml_text }
    }
    pub fn get_value(&self) -> &'a str {
        let element_text = self.extract_element_text(self.name)
            .expect("malformed element");
        &element_text[element_text.find('>').unwrap() + 1..element_text.rfind('<').unwrap()]
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

        // Element.
        let string_value = xml.get_element("string_element").unwrap().get_value();
        assert_eq!("string", string_value);

        // Nested element.
        let outer_element = xml.get_element("outer_element").unwrap();
        let inner_element = outer_element.get_element("inner_element").unwrap();
        let inner_value = inner_element.get_value();
        assert_eq!("inner", inner_value);
    }
}
