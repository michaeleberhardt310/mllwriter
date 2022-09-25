/// First version of a HTML-only-Writer

/// The Writer struct/class, to be used to fill the content-string with HTML.
#[derive(Debug, Clone)]
pub struct Writer {
    // holds the whole file content as long the Writer is used
    pub content: String,
    // number of whitespaces one indent-step means
    indent_step_size: usize,
    // holds the current indent as a string for quick adding into content
    indent: String,
    // holds a stack with opened/unclosed block-tags
    block_stack: Vec<String>
}


impl Writer {
    /// Method generates a new Writer with empty content and presets, e.g. zero indent
    pub fn new() -> Writer {
        Writer{
            content: String::new(),
            indent_step_size: 4,
            indent: String::new(),
            block_stack: Vec::new(),
        }
    }

    /// Method resets the writer to defaults and empties the content-string as well
    pub fn clear(&mut self) {
        self.content.clear();
        self.set_indent_step(0);
        self.set_indent_step_size(4);
        self.block_stack.clear();
    }

    /// Method opens a new block, e.g. <div> tag
    pub fn w_open_element(&mut self, tag: &str) {
        self.content.push_str(&["<".to_string() + tag + ">"].concat());
        self.block_stack.push(tag.to_string());
    }

    /// Method closes the last opened block, e.g. </div> tag.
    pub fn w_close_element(&mut self) {
        let tag = self.block_stack.pop().unwrap();
        self.content.push_str(&["</".to_string() + &tag + ">"].concat());
    }

    /// Method prints a single-tag element into the content-string
    pub fn w_single_element(&mut self, tag: &str) {
        self.content.push_str(&["<".to_string() + tag + ">"].concat());
    }

    /// Method adds a single property-value-pair and pushes it onto the content-string retroactively.
    pub fn w_property(&mut self, name: &str, value: &str) {
        // First we remove the '>' of the last entry
        self.content.pop();
        // Then add the property-value-pair and close the tag again after insertion
        self.content.push_str(&[" ".to_string() + name + "=\"" + value + "\">"].concat());
    }

    /// Method generates a property-string out of given properties and pushes it onto content-string retroactively.
    /// It uses therefor the Property-struct definition to be able to accept an arbitrary number of properties.
    pub fn w_properties(&mut self, properties: &Property) {
        // First we remove the '>' of the last entry
        self.content.pop();
        // Then, we add our property-string
        properties.0.iter().for_each(|x| self.content.push_str(
            &(" ".to_string() + &x.0 + "=\"" + &x.1 + "\"")
        ));
        // Finally, we close the tag again
        self.content.push_str(">");
    }

    /// Method adds a line feed to content string and writes the current indent
    pub fn w_lf(&mut self) {
        self.content.push_str(&["\n".to_string() + &self.indent].concat());
    }

    /// Method meaningful combines inc_indent_step() and w_lf() 
    pub fn w_lf_inc(&mut self) {
        self.inc_indent_step();
        self.w_lf();
    }

    /// Method meaningful combines dec_indent_step() and w_lf() 
    pub fn w_lf_dec(&mut self) {
        self.dec_indent_step();
        self.w_lf();
    }

    /// Method increases the current indent by indent_step_size
    pub fn inc_indent_step(&mut self) {
        self.indent.push_str(" ".repeat(self.indent_step_size).as_str());
    }

    /// Method decreases the current indent by indent_step_size
    pub fn dec_indent_step(&mut self) {
        let steps = if self.indent_step_size > self.indent.len() { 
            self.indent.len() 
        }
        else { 
            self.indent_step_size 
        };
        for _i in 0..steps {
            self.indent.pop();
        }
    }

    /// Set an arbitrary certain indent step. The method automatically multiplies the given value with current indent_step_size
    /// and resets an internal string for faster inserting the current indent during the document generation progress.
    pub fn set_indent_step(&mut self, indent_step: usize) {
        self.indent = " ".repeat(indent_step * self.indent_step_size).to_string();
    }

    /// Set the indent-step-size (the number of whitespaces per indent-step). Default is 4 whitespaces. Method results an Err if
    /// called after started editing (content isn't empty anymore).
    pub fn set_indent_step_size(&mut self, indent_step_size: usize) {
        self.indent_step_size = indent_step_size;
    }
}


/// The Property struct simplifies to encapsule several properties, e.g. class="superhero" and style="width: auto". These can
/// be passed to the Writer, which pushes it onto the content-string in the right way
pub struct Property(Vec<(String,String)>);

impl Property {
    /// A default new method with one first property pair to be passed
    pub fn new(name: &str, value: &str) -> Property {
        let mut p = Property(Vec::new());
        p.0.push((name.to_string(), value.to_string()));
        p
    }

    /// Simple method to add other properties to the stack
    pub fn add(&mut self, name: &str, value: &str) {
        self.0.push((name.to_string(), value.to_string()));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_n_clear() {
        let mut wr = Writer::new();
        assert_eq!(wr.content, "");
        assert_eq!(wr.indent_step_size, 4);
        assert_eq!(wr.indent, "");
        assert_eq!(wr.block_stack, Vec::<String>::new());

        wr.w_open_element("div");
        wr.set_indent_step(4);
        wr.set_indent_step_size(8);
        wr.clear();
        assert_eq!(wr.content, "");
        assert_eq!(wr.indent_step_size, 4);
        assert_eq!(wr.indent, "");
        assert_eq!(wr.block_stack, Vec::<String>::new());
    }

    #[test]
    fn test_single_element() {
        let mut wr = Writer::new();
        wr.w_single_element("img");
        assert_eq!(wr.content, "<img>".to_string());
    }

    #[test]
    fn test_dual_elements() {
        let mut wr = Writer::new();
        wr.w_open_element("div");
        wr.w_close_element();
        assert_eq!(wr.content, "<div></div>".to_string());
    }

    #[test]
    fn test_mixed_entries() {
        let mut wr = Writer::new();
        wr.w_open_element("div");
        wr.w_property("class", "container");
        wr.w_lf_inc();
        wr.w_single_element("img");
        wr.w_property("style", "width: auto");
        wr.w_lf_dec();
        wr.w_close_element();
        assert_eq!(wr.content, "<div class=\"container\">\n    <img style=\"width: auto\">\n</div>")
    }

    #[test]
    fn test_property_string() {
        let mut properties = Property::new("class", "container");
        properties.add("style", "width: auto");
        let mut wr = Writer::new();
        wr.w_single_element("img");
        wr.w_properties(&properties);
        assert_eq!(wr.content, "<img class=\"container\" style=\"width: auto\">".to_string());

        wr.clear();
        wr.w_single_element("img");
        wr.w_property("style", "width: auto");
        assert_eq!(wr.content, "<img style=\"width: auto\">");
    }

    #[test]
    fn test_indent_methods() {
        let mut wr = Writer::new();
        assert_eq!(wr.indent, "".to_string());

        wr.set_indent_step(2);
        assert_eq!(wr.indent, "        ".to_string());

        wr.dec_indent_step();
        assert_eq!(wr.indent, "    ".to_string());

        wr.inc_indent_step();
        assert_eq!(wr.indent, "        ".to_string());

        wr.set_indent_step_size(3);
        wr.set_indent_step(1);
        assert_eq!(wr.indent, "   ");
    }

    #[test]
    fn test_property() {
        let mut prop = Property::new("class", "superhero");
        assert_eq!(prop.0[0], ("class".to_string(), "superhero".to_string()));

        prop.add("style", "width: auto");
        assert_eq!(prop.0[1], ("style".to_string(), "width: auto".to_string()));
    }
}