//! # Markup-language-like Writer (MLLWriter)
//! 
//! The MLLWriter (Markup-language-like Writer) is a small collection of writer-tools to simplify the automated
//! writing process with HTML, XML and JSON-files. This crate contains the trait MLLWriter to generalize all of
//! those sub-types, and it contains an object for each writer type, e.g. HTMLWriter, XMLWriter and JSONWriter.
//! 
//! The basic idea is, that every markup-language-like file is getting build by blocks (HTML & XML: 'div' and '/div', 
//! JSON: '{' and '}' ). That's why every writer can open and close those **elements**. In HTML and XML there is also the
//! possibility for single-elements, e.g. 'img'. Each markup-like-language has its typical syntax as well, e.g. 
//! "style=\"widht: auto\"". In JSON it is a little bit more complicated, because it supports different data types,
//! e.g. '\"Name\" = \"Michael\"' and '\"Value\" = 5'.
//! 
//! ## Behavior
//! 
//! The basic writing is quite the same in all writer-types. Advise the writers to open and close elements (or blocks). 
//! HTMLWriter and XMLWriter do not add auto-line-feed when closing or opening blocks, to keep a "styling-taste-freedom". 
//! The JSONWriter automatically adds line-feed when closing a block or adding another property, but does have a usage
//! of the ```w_single_element()``` method. It writes only properties and blocks (a structural sub-property is just another
//! block, opened by ```w_open_element()``` and a property-name passed to the tag-argument).
//! 
//! There are different default indent-step-sizes, e.g. 4 whitespaces in the XMLWriter and HTMLWriter, and 2 for the JSONWriter.
//! 
//! More individual behavior of the given writer-types will be implemented in the future, when needed or requested.
//! 
//! ## Examples
//! 
//! ```
//! let mut wr = HTMLWriter::new();
//! wr.w_open_element("div");
//! wr.w_property("class", "container");
//! wr.w_lf_inc();
//! wr.w_single_element("img");
//! wr.w_property("style", "width: auto");
//! wr.w_lf_dec();
//! wr.w_close_element();
//! ```
//! 
//! ```
//! let mut wr = JSONWriter::new();
//! wr.w_open_element("");
//! wr.w_property("First Name", "\"Muster\"");
//! wr.w_property("Second Name", "\"Max\"");
//! wr.w_open_element("Data");
//! wr.w_property("Date of Birth", "\"05.06.1981\"");
//! wr.w_property("Number of Kids", "2");
//! wr.w_close_element();
//! wr.w_close_element();
//! ```

use std::result::Result;

/// Trait MLLWriter (Markup-language-like Writer) describes a common behavior for all writer-types. Writer-types will
/// be a version which prints a HTML-file, a XML-file or a JSON-file each. All those file-types have a structural-pattern
/// in common, even when a JSON-file is no markup-file - that's why it is a markup-language-like writer.
pub trait MLLWriter {
    /// Method opens a new block, e.g. the 'div'-HTML-tag or '{'-block in JSON.
    fn w_open_element(&mut self, tag: &str);

    /// Method closes the last opened block, e.g. '/div'-HTML-tag or '}'-block in JSON.
    fn w_close_element(&mut self);

    /// Method prints a single-tag element into the content-string, e.g. 'img' in HTML, no use-case in JSON.
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


/// All Writer-types have some similarities, e.g. adding a line-feed or increment and decrement
/// the current indent in the document under edit. That's why all this common functionality is
/// encapsuled in the WriterCore struct. This struct holds:
/// - the **content**-String, which holds the markup-content under edit
/// - the indent_step_size, as a number of whitespaces to be added at current line
/// - the block_stack, for closing HTML-tags automatically without specifying again which one
/// - other useful data for internal usage
/// This struct is used as a composition in the WriterTypes: HTMLWriter, XMLWriter and JSONWriter
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


impl std::fmt::Display for WriterCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "indent_step_size: {}\nindent: {}\nblock_stack: {:?}\n{}\n",
            self.indent_step_size, self.indent.len(), self.block_stack, self.content)
    }
}


impl std::fmt::Write for WriterCore {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.content.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error> {
        self.content.write_char(c)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), std::fmt::Error> {
        self.content.write_fmt(args)
    }
}


// ================================================================================================
/// Implementation of the HTMLWriter for writing HTML-files. Default indent-step-size is 4. There is
/// no auto-fill in any way. The user has to use ```w_lf()```, ```w_lf_inc()``` and ```w_lf_dec()```
/// for line-feeds and to style his HTML-files in its own taste.
#[derive(Debug, Clone)]
pub struct HTMLWriter {
    /// WriterCore in a composition
    pub core: WriterCore
}


impl HTMLWriter {
    pub fn new() -> HTMLWriter {
        HTMLWriter { core: WriterCore::new(4) }
    }
}


impl MLLWriter for HTMLWriter {
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


impl std::fmt::Display for HTMLWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.core.fmt(f)
    }
}


impl std::fmt::Write for HTMLWriter {
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


// ================================================================================================
/// Implementation of the XMLWriter for writing XML-files. Default indent-step-size is 4. There is
/// no auto-fill in any way. The user has to use ```w_lf()```, ```w_lf_inc()``` and ```w_lf_dec()```
/// for line-feeds and to style his XML-files in its own taste. To be adapted in the future...
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
        self.core.indent_step_size = 2;
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


// ================================================================================================
/// The JSON-implementation of MLLWriter. The JSONWriter has a default indent-step-size of 2 and does
/// auto line-feed, when adding properties or closing blocks. Multiple properties can be passed via
/// the ```w_properties()``` method, but no structural-properties. If a sub-struct as a property has
/// to be added, the ```w_open_element()``` has to be used with the property-name as tag-parameter.
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


// ================================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================================
    // Tests for the WriterCore and the Property-struct
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

    // ============================================================================================
    // Tests for HTMLWriter
    #[test]
    fn test_html_new_n_clear() {
        let mut wr = HTMLWriter::new();
        assert_eq!(wr.core.content, "");
        assert_eq!(wr.core.indent_step_size, 4);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());

        wr.w_open_element("div");
        wr.set_indent_step(4);
        wr.set_indent_step_size(8);
        wr.clear();
        assert_eq!(wr.core.content, "");
        assert_eq!(wr.core.indent_step_size, 4);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());
    }

    #[test]
    fn test_html_single_element() {
        let mut wr = HTMLWriter::new();
        wr.w_single_element("img");
        assert_eq!(wr.core.content, "<img>".to_string());
    }

    #[test]
    fn test_html_dual_elements() {
        let mut wr = HTMLWriter::new();
        wr.w_open_element("div");
        wr.w_close_element();
        assert_eq!(wr.core.content, "<div></div>".to_string());
    }

    #[test]
    fn test_html_mixed_entries() {
        let mut wr = HTMLWriter::new();
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
    fn test_html_property_string() {
        let mut properties = Property::new("class", "container");
        properties.add("style", "width: auto");
        let mut wr = HTMLWriter::new();
        wr.w_single_element("img");
        wr.w_properties(&properties);
        assert_eq!(wr.core.content, "<img class=\"container\" style=\"width: auto\">".to_string());

        wr.clear();
        wr.w_single_element("img");
        wr.w_property("style", "width: auto");
        assert_eq!(wr.core.content, "<img style=\"width: auto\">");
    }

    // ============================================================================================
    // Tests for the XMLWriter
    #[test]
    fn test_xml_new_n_clear() {
        let mut wr = XMLWriter::new();
        assert_eq!(wr.core.content, "");
        assert_eq!(wr.core.indent_step_size, 2);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());

        wr.w_open_element("div");
        wr.set_indent_step(4);
        wr.set_indent_step_size(8);
        wr.clear();
        assert_eq!(wr.core.content, "");
        assert_eq!(wr.core.indent_step_size, 2);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());
    }

    #[test]
    fn test_xml_single_element() {
        let mut wr = XMLWriter::new();
        wr.w_single_element("img");
        assert_eq!(wr.core.content, "<img>".to_string());
    }

    #[test]
    fn test_xml_dual_elements() {
        let mut wr = XMLWriter::new();
        wr.w_open_element("div");
        wr.w_close_element();
        assert_eq!(wr.core.content, "<div></div>".to_string());
    }

    #[test]
    fn test_xml_mixed_entries() {
        let mut wr = XMLWriter::new();
        wr.w_open_element("div");
        wr.w_property("class", "container");
        wr.w_lf_inc();
        wr.w_single_element("img");
        wr.w_property("style", "width: auto");
        wr.w_lf_dec();
        wr.w_close_element();
        assert_eq!(wr.core.content, "<div class=\"container\">\n  <img style=\"width: auto\">\n</div>")
    }

    #[test]
    fn test_xml_property_string() {
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

    // ============================================================================================
    #[test]
    #[should_panic(expected = "there is no single_element in the JSONWriter")]
    fn test_json_single_element() {
        let mut wr = JSONWriter::new();
        wr.w_single_element("img");    
    }

    #[test]
    fn test_json_dual_elements() {
        let mut wr = JSONWriter::new();
        wr.w_open_element("");
        wr.w_close_element();
        assert_eq!(wr.core.content, "{\n}".to_string());
    }

    #[test]
    fn test_json_mixed_entries() {
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
    fn test_json_property_string() {
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


// ================================================================================================