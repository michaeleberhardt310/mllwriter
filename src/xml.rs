/// This module implements a HTML-variant of the MLLWriter-trait
use super::writer::*;

/// The Writer struct/class, to be used to fill the content-string with HTML.
#[derive(Debug, Clone)]
pub struct XMLWriter {
    /// WriterCore in a composition
    pub core: WriterCore
}


impl XMLWriter {
    pub fn new() -> XMLWriter {
        XMLWriter { core: WriterCore::new(2) }
    }
}


impl MLLWriter for XMLWriter {
    fn w_open_element(&mut self, tag: &str) {
        self.core.content.push_str(&["<".to_string() + tag + ">"].concat());
        self.core.block_stack.push(tag.to_string());
    }

    
    fn w_close_element(&mut self) {
        let tag = self.core.block_stack.pop().unwrap();
        self.core.content.push_str(&["</".to_string() + &tag + ">"].concat());
    }

    
    fn w_single_element(&mut self, tag: &str) {
        self.core.content.push_str(&["<".to_string() + tag + ">"].concat());
    }

    
    fn w_property(&mut self, name: &str, value: &str) {
        // First we remove the '>' of the last entry
        self.core.content.pop();
        // Then add the property-value-pair and close the tag again after insertion
        self.core.content.push_str(&[" ".to_string() + name + "=\"" + value + "\">"].concat());
    }

    
    fn w_properties(&mut self, properties: &Property) {
        // First we remove the '>' of the last entry
        self.core.content.pop();
        // Then, we add our property-string
        properties.p.iter().for_each(|x| self.core.content.push_str(
            &(" ".to_string() + &x.0 + "=\"" + &x.1 + "\"")
        ));
        // Finally, we close the tag again
        self.core.content.push_str(">");
    }


    fn w_lf(&mut self) { self.core.w_lf(); }
    
    fn w_lf_inc(&mut self) { self.core.w_lf_inc(); }

    fn w_lf_dec(&mut self) { self.core.w_lf_dec(); }
    
    fn inc_indent_step(&mut self) { self.core.inc_indent_step(); }

    fn dec_indent_step(&mut self) { self.core.dec_indent_step(); }

    fn set_indent_step(&mut self, indent_step: usize) { self.core.set_indent_step(indent_step); }

    fn set_indent_step_size(&mut self, indent_step_size: usize) { self.core.set_indent_step_size(indent_step_size); }

    fn clear(&mut self) { 
        self.core.clear(); 
        self.core.indent_step_size = 4;
    }
}


impl std::fmt::Display for XMLWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.core.fmt(f)
    }
}


impl std::fmt::Write for XMLWriter {
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
    fn test_new_n_clear() {
        let mut wr = XMLWriter::new();
        assert_eq!(wr.core.content, "");
        assert_eq!(wr.core.indent_step_size, 4);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());

        wr.w_open_element("div");
        wr.set_indent_step(2);
        wr.set_indent_step_size(8);
        wr.clear();
        assert_eq!(wr.core.content, "");
        assert_eq!(wr.core.indent_step_size, 2);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());
    }

    #[test]
    fn test_single_element() {
        let mut wr = XMLWriter::new();
        wr.w_single_element("img");
        assert_eq!(wr.core.content, "<img>".to_string());
    }

    #[test]
    fn test_dual_elements() {
        let mut wr = XMLWriter::new();
        wr.w_open_element("div");
        wr.w_close_element();
        assert_eq!(wr.core.content, "<div></div>".to_string());
    }

    #[test]
    fn test_mixed_entries() {
        let mut wr = XMLWriter::new();
        wr.w_open_element("div");
        wr.w_property("class", "container");
        wr.w_lf_inc();
        wr.w_single_element("img");
        wr.w_property("style", "width: auto");
        wr.w_lf_dec();
        wr.w_close_element();
        assert_eq!(wr.core.content, "<div class=\"container\">\n    <img style=\"width: auto\">\n</div>")
    }

    #[test]
    fn test_property_string() {
        let mut properties = Property::new("class", "container");
        properties.add("style", "width: auto");
        let mut wr = XMLWriter::new();
        wr.w_single_element("img");
        wr.w_properties(&properties);
        assert_eq!(wr.core.content, "<img class=\"container\" style=\"width: auto\">".to_string());

        wr.clear();
        wr.w_single_element("img");
        wr.w_property("style", "width: auto");
        assert_eq!(wr.core.content, "<img style=\"width: auto\">");
    }
}