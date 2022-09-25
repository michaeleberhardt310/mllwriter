/// Trait MLLWriter (Markup-language-like Writer) describes a common behavior for all sub-types. Sub-types will
/// be a version which prints a HTML-file, a XML-file or a JSON-file. All those file-types have a structural-pattern
/// in common, even when a JSON-file is no markup-file - that's why it is a markup-language-like writer.
pub trait MLLWriter {
    // The ascociated sub-type, e.g. HTMLWriter, XMLWriter or JSONWriter
    type MLLWriter;

    /// Method generates a new Writer with empty content and presets, e.g. zero indent
    fn new() -> Self::MLLWriter;

    /// Method resets the writer to defaults and empties the content-string as well
    fn clear(&mut self);

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
}


/// The typical Writer struct, to be used to fill the content-string with HTML, XML or JSON.
/// To be implemented in each variant in a module. Have a look into the html-module.
/// #[derive(Debug, Clone)]
/// pub struct Writer {
///     // holds the whole file content as long the Writer is used
///     pub content: String,
///     // number of whitespaces one indent-step means
///     indent_step_size: usize,
///     // holds the current indent as a string for quick adding into content
///     indent: String,
///     // holds a stack with opened/unclosed block-tags
///     block_stack: Vec<String>
/// }


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

    /// Method generates the HTML-String variant
    pub fn html_str(&self) -> String {
        let mut s = String::new();
        self.0.iter().for_each(|x| s.push_str(
            &(" ".to_string() + &x.0 + "=\"" + &x.1 + "\"")
        ));
        s
    }

    /// Method generates the XML-String variant
    pub fn xml_str(&self) -> String {
        String::new()
    }

    /// Method generates the JSON-String variant
    pub fn json_str(&self) -> String {
        String::new()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property() {
        let mut prop = Property::new("class", "superhero");
        assert_eq!(prop.0[0], ("class".to_string(), "superhero".to_string()));

        prop.add("style", "width: auto");
        assert_eq!(prop.0[1], ("style".to_string(), "width: auto".to_string()));
    }
}