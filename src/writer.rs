/// Trait MLLWriter (Markup-language-like Writer) describes a common behavior for all sub-types. Sub-types will
/// be a version which prints a HTML-file, a XML-file or a JSON-file. All those file-types have a structural-pattern
/// in common, even when a JSON-file is no markup-file - that's why it is a markup-language-like writer.
pub trait MLLWriter {
    /// Method opens a new block, e.g. <div> tag
    fn w_open_element(&mut self, tag: &str);

    /// Method closes the last opened block, e.g. </div> tag.
    fn w_close_element(&mut self);

    /// Method prints a single-tag element into the content-string
    fn w_single_element(&mut self, tag: &str);

    /// Method adds a single property-value-pair and pushes it onto the content-string retroactively.
    fn w_property(&mut self, name: &str, value: &str);

    /// Method generates a property-string out of given properties and pushes it onto content-string retroactively.
    /// It uses therefor the Property-struct definition to be able to accept an arbitrary number of properties.
    fn w_properties(&mut self, properties: &Property);

    /// Method adds a line feed to content string and writes the current indent
    fn w_lf(&mut self);

    /// Method meaningful combines inc_indent_step() and w_lf() 
    fn w_lf_inc(&mut self);

    /// Method meaningful combines dec_indent_step() and w_lf() 
    fn w_lf_dec(&mut self);

    /// Method increases the current indent by indent_step_size
    fn inc_indent_step(&mut self);

    /// Method decreases the current indent by indent_step_size
    fn dec_indent_step(&mut self);

    /// Set an arbitrary certain indent step. The method automatically multiplies the given value with current indent_step_size
    /// and resets an internal string for faster inserting the current indent during the document generation progress.
    fn set_indent_step(&mut self, indent_step: usize);

    /// Set the indent-step-size (the number of whitespaces per indent-step). Default is 4 whitespaces. Method results an Err if
    /// called after started editing (content isn't empty anymore).
    fn set_indent_step_size(&mut self, indent_step_size: usize);

    /// Method resets the writer to defaults and empties the content-string as well
    fn clear(&mut self);
}


/// The Property struct simplifies to encapsule several properties, e.g. class="superhero" and style="width: auto". These can
/// be passed to the Writer, which pushes it onto the content-string in the right way
pub struct Property {
    pub(crate) p: Vec<(String,String)>
}

impl Property {
    /// A default new method with one first property pair to be passed
    pub fn new(name: &str, value: &str) -> Property {
        let mut p = Property{ p: Vec::new() };
        p.p.push((name.to_string(), value.to_string()));
        p
    }

    /// Simple method to add other properties to the stack
    pub fn add(&mut self, name: &str, value: &str) {
        self.p.push((name.to_string(), value.to_string()));
    }
}


/// The typical Writer struct, to be used to fill the content-string with HTML, XML or JSON.
/// To be implemented in each variant in a module. Have a look into the html-module.
#[derive(Debug, Clone)]
pub struct WriterCore {
    // holds the whole file content as long the Writer is used
    pub content: String,
    // number of whitespaces one indent-step means
    pub(crate) indent_step_size: usize,
    // holds the current indent as a string for quick adding into content
    pub(crate) indent: String,
    // holds a stack with opened/unclosed block-tags
    pub(crate) block_stack: Vec<String>
}

impl WriterCore {
    // Methods to be implemented by each subtype individually
    pub fn new(indent_step_size: usize) -> WriterCore {
        WriterCore{
            content: String::new(),
            indent_step_size: indent_step_size,
            indent: String::new(),
            block_stack: Vec::new(),
        }
    }
}

impl MLLWriter for WriterCore {
    fn w_open_element(&mut self, _tag: &str) {
        // Nothing, because each variant does it in its own way
    }

    fn w_close_element(&mut self) {
        // Nothing, because each variant does it in its own way
    }

    fn w_single_element(&mut self, _tag: &str) {
        // Nothing, because each variant does it in its own way
    }

    fn w_property(&mut self, _name: &str, _value: &str) {
        // Nothing, because each variant does it in its own way
    }

    fn w_properties(&mut self, _properties: &Property) {
        // Nothing, because each variant does it in its own way
    }

    fn w_lf(&mut self) {
        self.content.push_str(&["\n".to_string() + &self.indent].concat());
    }

    fn w_lf_inc(&mut self) {
        self.inc_indent_step();
        self.w_lf();
    }

    fn w_lf_dec(&mut self) {
        self.dec_indent_step();
        self.w_lf();
    }

    fn inc_indent_step(&mut self) {
        self.indent.push_str(" ".repeat(self.indent_step_size).as_str());
    }

    fn dec_indent_step(&mut self) {
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

    fn set_indent_step(&mut self, indent_step: usize) {
        self.indent = " ".repeat(indent_step * self.indent_step_size).to_string();
    }

    fn set_indent_step_size(&mut self, indent_step_size: usize) {
        self.indent_step_size = indent_step_size;
    }

    fn clear(&mut self) {
        self.content.clear();
        self.set_indent_step(0);
        // self.set_indent_step_size(4);
        self.block_stack.clear();
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property() {
        let mut prop = Property::new("class", "superhero");
        assert_eq!(prop.p[0], ("class".to_string(), "superhero".to_string()));

        prop.add("style", "width: auto");
        assert_eq!(prop.p[1], ("style".to_string(), "width: auto".to_string()));
    }

    #[test]
    fn test_indent_methods() {
        let mut wr = WriterCore::new(4);
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
}