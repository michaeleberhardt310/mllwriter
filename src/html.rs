/// First version of a HTML-only-Writer

use std::result::Result;


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


// To simplify writing that
pub type Property = Vec<(String,String)>;


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

    /// Method opens a new block, e.g. <div> tag
    pub fn open_element(&mut self, tag: &str, properties: &Property) -> Result<(), String> {
        Ok(())
    }

    /// Method closes the last opened block, e.g. </div> tag.
    pub fn close_element(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Method prints a single-tag element into the content-string
    pub fn single_element(&mut self, tag: &str, properties: &Property) -> Result<(), String> {
        Ok(())
    }

    /// Method increases the current indent by indent_step_size
    pub fn inc_indent_step(&mut self) -> Result<(), String> {
        self.indent.push_str(" ".repeat(self.indent_step_size).as_str());
        if self.indent.len() % self.indent_step_size == 0 { Ok(()) }
        else { Err(format!("indent-len: {}, indent-step-size: {}", self.indent.len(), self.indent_step_size)) }
    }

    /// Method decreases the current indent by indent_step_size
    pub fn dec_indent_step(&mut self) -> Result<(), String> {
        if self.indent.len() >= self.indent_step_size {
            for i in 0..self.indent_step_size {
                self.indent.pop();
            }
            Ok(())
        } else {
            Err(format!("indent-len: {}, indent-step-size: {}", self.indent.len(), self.indent_step_size))
        }
    }

    /// Set an arbitrary certain indent step. The method automatically multiplies the given value with current indent_step_size
    /// and resets an internal string for faster inserting the current indent during the document generation progress.
    pub fn set_indent_step(&mut self, indent_step: usize) -> Result<(), String> {
        self.indent = " ".repeat(indent_step * self.indent_step_size).to_string();
        Ok(())
    }

    /// Set the indent-step-size (the number of whitespaces per indent-step). Default is 4 whitespaces. Method results an Err if
    /// called after started editing (content isn't empty anymore).
    pub fn set_indent_step_size(&mut self, indent_step_size: usize) -> Result<(), String> {
        if self.content.len() > 0 {
            Err("content editing has already started".to_string())
        } else {
            self.indent_step_size = indent_step_size;
            Ok(())
        }
    }

    // Method generates a property-string out of given properties
    fn wr_property_string(&mut self, properties: &Property) -> Result<(), String> {
        properties.iter().for_each(|x| self.content.push_str(
            &(x.0.clone() + "=\"" + &x.1.clone() + "\" ")
        ));
        self.content.pop();
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_string() {
        let mut properties = Property::new();
        properties.push(("class".to_string(), "container".to_string()));
        properties.push(("style".to_string(), "width: auto".to_string()));
        assert_eq!(property_string(&properties), 
            "class=\"container\" style=\"width: auto\"".to_string());
    }

    #[test]
    fn test_new() {
        let wr = Writer::new();
        assert_eq!(wr.content, "");
        assert_eq!(wr.indent_step_size, 4);
        assert_eq!(wr.indent, "");
        assert_eq!(wr.block_stack, Vec::<String>::new());
    }

    #[test]
    fn test_indent_methods() {
        let mut wr = Writer::new();
        assert_eq!(wr.indent, "".to_string());

        assert_eq!(wr.set_indent_step(2), Ok(()));
        assert_eq!(wr.indent, "        ".to_string());

        assert_eq!(wr.dec_indent_step(), Ok(()));
        assert_eq!(wr.indent, "    ".to_string());

        assert_eq!(wr.inc_indent_step(), Ok(()));
        assert_eq!(wr.indent, "        ".to_string());

        assert_eq!(wr.set_indent_step_size(3), Ok(()));
        assert_eq!(wr.set_indent_step(1), Ok(()));
        assert_eq!(wr.indent, "   ");

        wr.content = "My father told me...".to_string();
        assert_eq!(wr.set_indent_step_size(4), Err("content editing has already started".to_string()));
    }
}