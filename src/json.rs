/// This module implements a JSON-variant of the MLLWriter-trait
use super::writer::*;

/// The Writer struct/class, to be used to fill the content-string with JSON.
#[derive(Debug, Clone)]
pub struct JSONWriter {
    /// WriterCore in a composition
    pub core: WriterCore
}


impl JSONWriter {
    /// Returns a new JSONWriter struct with default indent-step-size of 2.
    pub fn new() -> JSONWriter {
        JSONWriter { core: WriterCore::new(2) }
    }


    // This method checks the current ending and does correct line-feed, ether with indent-increment or with comma
    fn prepare_property_write(&mut self) {
        // Check the current ending
        if self.core.content.ends_with("{") {
            // if it is a '{' add a line-feed with indent-increment
            self.w_lf_inc();
        } else if self.core.content.len() > 0 {
            // there must be at least one property, so separate them by a comma
            self.core.content.push_str(&[",\n".to_string() + &self.core.indent].concat());
        }
    }
}


// The philosophy here is, only to write the current desired task, nothing more! E.g. w_open_element()
// writes only the '{' and nothing else. w_property() writes only the property. If a line feed or indent
// is needed, the method checks the current ending and adds this task before adding the true task.
impl MLLWriter for JSONWriter {
    fn w_open_element(&mut self, tag: &str) {
        self.prepare_property_write();
        if tag.len() > 0 {
            self.core.content.push_str(&["\"".to_string() + tag + "\":\n" + &self.core.indent + "{"].concat());
        } else {
            self.core.content.push_str("{");
        }
    }

    
    fn w_close_element(&mut self) {
        self.core.w_lf_dec();
        self.core.content.push_str("}");
    }

    
    fn w_single_element(&mut self, _tag: &str) {
        panic!("there is no single_element in the JSONWriter");
    }

    
    fn w_property(&mut self, name: &str, value: &str) {
        self.prepare_property_write();
        self.core.content.push_str(&["\"".to_string() + name + "\": " + value].concat());
    }

    
    fn w_properties(&mut self, properties: &Property) {
        properties.p.iter().for_each(|x| self.w_property(&x.0, &x.1) );
    }

    
    fn clear(&mut self) { self.core.clear(); }

    fn w_lf(&mut self) { self.core.w_lf(); }
    
    fn w_lf_inc(&mut self) { self.core.w_lf_inc(); }

    fn w_lf_dec(&mut self) { self.core.w_lf_dec(); }
    
    fn inc_indent_step(&mut self) { self.core.inc_indent_step(); }

    fn dec_indent_step(&mut self) { self.core.dec_indent_step(); }

    fn set_indent_step(&mut self, indent_step: usize) { self.core.set_indent_step(indent_step); }

    fn set_indent_step_size(&mut self, indent_step_size: usize) { self.core.set_indent_step_size(indent_step_size); }
}


impl std::fmt::Display for JSONWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.core.fmt(f)
    }
}


impl std::fmt::Write for JSONWriter {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.core.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error> {
        self.core.write_char(c)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), std::fmt::Error> {
        self.core.write_fmt(args)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "there is no single_element in the JSONWriter")]
    fn test_single_element() {
        let mut wr = JSONWriter::new();
        wr.w_single_element("img");    
    }

    #[test]
    fn test_dual_elements() {
        let mut wr = JSONWriter::new();
        wr.w_open_element("");
        wr.w_close_element();
        assert_eq!(wr.core.content, "{\n}".to_string());
    }

    #[test]
    fn test_mixed_entries() {
        let mut wr = JSONWriter::new();
        wr.w_open_element("");
        wr.w_property("Name", "\"Eberhardt\"");
        wr.w_property("Vorname", "\"Michael\"");
        wr.w_open_element("Daten");
        wr.w_property("Geburtstag", "\"03.10.1985\"");
        wr.w_close_element();
        wr.w_close_element();
        assert_eq!(wr.core.content, 
            "{\n  \"Name\": \"Eberhardt\",\n  \"Vorname\": \"Michael\",\n  \"Daten\":\n  {\n    \"Geburtstag\": \"03.10.1985\"\n  }\n}"
        );
    }

    #[test]
    fn test_property_string() {
        let mut properties = Property::new("Name", "\"Eberhardt\"");
        properties.add("Alter", "35");
        let mut wr = JSONWriter::new();
        wr.w_open_element("");
        wr.w_properties(&properties);
        wr.w_close_element();
        assert_eq!(wr.core.content, "{\n  \"Name\": \"Eberhardt\",\n  \"Alter\": 35\n}".to_string());

        wr.clear();
        assert_eq!(wr.core.content, "");
    }
}