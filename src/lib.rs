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
//! of the ```single_tag()``` method. It writes only properties and blocks (a structural sub-property is just another
//! block, opened by ```open_tag()``` and a property-name passed to the tag-argument).
//! 
//! There are different default indent-step-sizes, e.g. 4 whitespaces in the XMLWriter and HTMLWriter, and 2 for the JSONWriter.
//! 
//! More individual behavior of the given writer-types will be implemented in the future, when needed or requested.
//! 
//! ## Examples
//! 
//! In this example a html-div-tag will be written with a certain id and class, and it will enclose an img-single-tag.
//! ```
//! # use mllwriter::{MLLWriter,HTMLWriter};
//! let mut wr = HTMLWriter::new();
//! 
//! wr.open_tag_w_property("div", "class", "container");
//! wr.add_property("id", "logo");
//! wr.line_feed_inc();
//! wr.single_tag("img");
//! wr.add_property("style", "width: auto");
//! wr.line_feed_dec();
//! wr.close_tag();
//! ```
//! 
//! This example writes a simple JSON-file with a couple of properties.
//! ```
//! # use mllwriter::{MLLWriter,JSONWriter};
//! let mut wr = JSONWriter::new();
//! 
//! wr.open_tag("");
//! wr.add_property("First Name", "\"Muster\"");
//! wr.add_property("Second Name", "\"Max\"");
//! wr.open_tag("Data");
//! wr.add_property("Date of Birth", "\"05.06.1981\"");
//! wr.add_property("Number of Kids", "2");
//! wr.close_tag();
//! wr.close_tag();
//! ```

use std::result::Result;

/// Trait MLLWriter (Markup-language-like Writer) describes a common behavior for all writer-types. Writer-types will
/// be a version which prints a HTML-file, a XML-file or a JSON-file each. All those file-types have a structural-pattern
/// in common, even when a JSON-file is no markup-file - that's why it is a markup-language-like writer.
pub trait MLLWriter {
    /// Method opens a new block, e.g. the 'div'-HTML-tag or '{'-block in JSON.
    fn open_tag(&mut self, tag: &str);

    /// Combines open_tag() and add_property()
    fn open_tag_w_property(&mut self, tag: &str, prop: &str, value: &str);

    /// Method closes the last opened block, e.g. '/div'-HTML-tag or '}'-block in JSON.
    fn close_tag(&mut self);

    /// Method prints a single-tag element into the content-string, e.g. 'img' in HTML, no use-case in JSON.
    fn single_tag(&mut self, tag: &str);

    /// Combines single_tag() and add_property()
    fn single_tag_w_property(&mut self, tag: &str, prop: &str, value: &str);

    /// Method adds a single property-value-pair and pushes it onto the content-string retroactively.
    fn add_property(&mut self, name: &str, value: &str);

    /// Method generates a property-string out of given properties and pushes it onto content-string retroactively.
    /// It uses therefor the Property-struct definition to be able to accept an arbitrary number of properties.
    fn add_properties(&mut self, properties: &Property);

    /// Method adds a single comment at current cursor position
    fn add_comment(&mut self, comment: &str);

    /// Method adds n line feed(s) to content string and writes the current indent
    fn line_feed(&mut self, n: usize);

    /// Method meaningful combines inc_indent_step() and line_feed() 
    fn line_feed_inc(&mut self);

    /// Method meaningful combines dec_indent_step() and line_feed() 
    fn line_feed_dec(&mut self);

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
    // number of whitespaces one indent-step means
    pub(crate) indent_step_size: usize,
    // holds the current indent as a string for quick adding into content
    pub(crate) indent: String,
    // holds a stack with opened/unclosed block-tags
    pub(crate) block_stack: Vec<String>
}


impl WriterCore {
    // Methods to be implemented by each subtype individually
    fn new(indent_step_size: usize) -> WriterCore {
        WriterCore{
            indent_step_size,
            indent: String::new(),
            block_stack: Vec::new(),
        }
    }


    fn clear(&mut self, indent_step: usize) {
        self.indent_step_size = indent_step;
        self.indent.clear();
        self.block_stack.clear();
    }


    fn line_feed(&mut self, content: &mut String, n: usize) {
        for _i in 0..n { content.push('\n'); }
        content.push_str(&self.indent);
    }


    fn line_feed_inc(&mut self, content: &mut String) {
        self.inc_indent_step();
        self.line_feed(content, 1);
    }


    fn line_feed_dec(&mut self, content: &mut String) {
        self.dec_indent_step();
        self.line_feed(content, 1);
    }


    fn inc_indent_step(&mut self) {
        self.indent.push_str(" ".repeat(self.indent_step_size).as_str());
    }


    fn dec_indent_step(&mut self) {
        let len = self.indent.len();
        if self.indent_step_size > len {
            self.indent = String::new();
        } else {
            self.indent.truncate(len - self.indent_step_size);
        }
    }


    pub fn set_indent_step(&mut self, indent_step: usize) {
        self.indent = " ".repeat(indent_step * self.indent_step_size);
    }


    pub fn set_indent_step_size(&mut self, indent_step_size: usize) {
        self.indent_step_size = indent_step_size;
    }
}


// ================================================================================================
/// Implementation of the HTMLWriter for writing HTML-files. Default indent-step-size is 4. There is
/// no auto-fill in any way. The user has to use ```line_feed()```, ```line_feed_inc()``` and ```line_feed_dec()```
/// for line-feeds and to style his HTML-files in its own taste.
#[derive(Debug, Clone)]
pub struct HTMLWriter {
    /// Content held by the writer
    pub content: String,
    /// WriterCore in a composition
    pub core: WriterCore
}


impl HTMLWriter {
    pub fn new() -> HTMLWriter {
        HTMLWriter { 
            content: String::new(),
            core: WriterCore::new(4)
        }
    }
}


impl Default for HTMLWriter {
    fn default() -> Self {
        HTMLWriter::new()
    }
}


impl MLLWriter for HTMLWriter {
    /// Accepts only ASCII-lowercase
    fn open_tag(&mut self, tag: &str) {
        assert_html_notation(tag);
        self.content.push('<');
        self.content.push_str(tag);
        self.content.push('>');
        self.core.block_stack.push(tag.to_string());
    }


    fn open_tag_w_property(&mut self, tag: &str, prop: &str, value: &str) {
        assert_html_notation(tag);
        self.open_tag(tag);
        self.add_property(prop, value);
    }

    
    fn close_tag(&mut self) {
        let tag = self.core.block_stack.pop().unwrap();
        self.content.push_str("</");
        self.content.push_str(&tag);
        self.content.push('>');
    }


    /// Accepts only ASCII-lowercase
    fn single_tag(&mut self, tag: &str) {
        assert_html_notation(tag);
        self.content.push('<');
        self.content.push_str(tag);
        self.content.push('>');
    }


    fn single_tag_w_property(&mut self, tag: &str, prop: &str, value: &str) {
        self.single_tag(tag);
        self.add_property(prop, value);
    }


    /// Accepts only ASCII-lowercase for the name-attribute
    fn add_property(&mut self, prop: &str, value: &str) {
        assert_html_notation(prop);
        // First we remove the '>' of the last entry
        self.content.pop();
        // Then add the property-value-pair and close the tag again after insertion
        self.content.push(' ');
        self.content.push_str(prop);
        self.content.push_str("=\"");
        self.content.push_str(value);
        self.content.push_str("\">");
    }

    
    fn add_properties(&mut self, properties: &Property) {
        // First we remove the '>' of the last entry
        self.content.pop();
        // Then, we add our property-string
        properties.p.iter().for_each(|x| self.content.push_str(
            &(" ".to_string() + &x.0 + "=\"" + &x.1 + "\"")
        ));
        // Finally, we close the tag again
        self.content.push('>');
    }


    fn add_comment(&mut self, comment: &str) {
        self.content.push_str("<!-- ");
        self.content.push_str(comment);
        self.content.push_str(" -->");
    }


    fn line_feed(&mut self, n: usize) { self.core.line_feed(&mut self.content, n); }
    
    fn line_feed_inc(&mut self) { self.core.line_feed_inc(&mut self.content); }

    fn line_feed_dec(&mut self) { self.core.line_feed_dec(&mut self.content); }
    
    fn inc_indent_step(&mut self) { self.core.inc_indent_step(); }

    fn dec_indent_step(&mut self) { self.core.dec_indent_step(); }

    fn set_indent_step(&mut self, indent_step: usize) { self.core.set_indent_step(indent_step); }

    fn set_indent_step_size(&mut self, indent_step_size: usize) { self.core.set_indent_step_size(indent_step_size); }

    fn clear(&mut self) { 
        self.content.clear(); 
        self.core.clear(4);
    }
}


impl std::fmt::Display for HTMLWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "indent_step_size: {}\nindent: {}\nblock_stack: {:?}\n{}\n",
            self.core.indent_step_size, self.core.indent.len(), self.core.block_stack, self.content)
    }
}


impl std::fmt::Write for HTMLWriter {
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
/// Implementation of the XMLWriter for writing XML-files. Default indent-step-size is 2. There is
/// no auto-fill in any way. The user has to use ```line_feed()```, ```line_feed_inc()``` and ```line_feed_dec()```
/// for line-feeds and to style his XML-files in its own taste. To be adapted in the future...
#[derive(Debug, Clone)]
pub struct XMLWriter {
    /// Content held by the writer
    pub content: String,
    /// WriterCore in a composition
    pub core: WriterCore
}


impl XMLWriter {
    pub fn new() -> XMLWriter {
        XMLWriter { 
            content: String::new(),
            core: WriterCore::new(2) 
        }
    }
}


impl Default for XMLWriter {
    fn default() -> Self {
        XMLWriter::new()
    }
}


impl MLLWriter for XMLWriter {
    /// Accepts only ASCII-lowercase for the name-attribute
    fn open_tag(&mut self, tag: &str) {
        assert_html_notation(tag);
        self.content.push('<');
        self.content.push_str(tag);
        self.content.push('>');
        self.core.block_stack.push(tag.to_string());
    }


    fn open_tag_w_property(&mut self, tag: &str, prop: &str, value: &str) {
        assert_html_notation(tag);
        self.open_tag(tag);
        self.add_property(prop, value);
    }

    
    fn close_tag(&mut self) {
        let tag = self.core.block_stack.pop().unwrap();
        self.content.push_str("</");
        self.content.push_str(&tag);
        self.content.push('>');
    }

    
    /// Accepts only ASCII-lowercase for the name-attribute
    fn single_tag(&mut self, tag: &str) {
        assert_html_notation(tag);
        self.content.push('<');
        self.content.push_str(tag);
        self.content.push('>');
    }


    fn single_tag_w_property(&mut self, tag: &str, prop: &str, value: &str) {
        self.single_tag(tag);
        self.add_property(prop, value);
    }

    
    /// Accepts only ASCII-lowercase for the name-attribute
    fn add_property(&mut self, name: &str, value: &str) {
        assert_html_notation(name);
        // First we remove the '>' of the last entry
        self.content.pop();
        // Then add the property-value-pair and close the tag again after insertion
        self.content.push(' ');
        self.content.push_str(name);
        self.content.push_str("=\"");
        self.content.push_str(value);
        self.content.push_str("\">");
    }
    
    
    fn add_comment(&mut self, comment: &str) {
        self.content.push_str("<!-- ");
        self.content.push_str(comment);
        self.content.push_str(" -->");
    }

    
    fn add_properties(&mut self, properties: &Property) {
        // First we remove the '>' of the last entry
        self.content.pop();
        // Then, we add our property-string
        properties.p.iter().for_each(|x| self.content.push_str(
            &(" ".to_string() + &x.0 + "=\"" + &x.1 + "\"")
        ));
        // Finally, we close the tag again
        self.content.push('>');
    }


    fn line_feed(&mut self, n: usize) { self.core.line_feed(&mut self.content, n); }
    
    fn line_feed_inc(&mut self) { self.core.line_feed_inc(&mut self.content); }

    fn line_feed_dec(&mut self) { self.core.line_feed_dec(&mut self.content); }
    
    fn inc_indent_step(&mut self) { self.core.inc_indent_step(); }

    fn dec_indent_step(&mut self) { self.core.dec_indent_step(); }

    fn set_indent_step(&mut self, indent_step: usize) { self.core.set_indent_step(indent_step); }

    fn set_indent_step_size(&mut self, indent_step_size: usize) { self.core.set_indent_step_size(indent_step_size); }

    fn clear(&mut self) { 
        self.core.clear(2); 
        self.content.clear();
    }
}


impl std::fmt::Display for XMLWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "indent_step_size: {}\nindent: {}\nblock_stack: {:?}\n{}\n",
            self.core.indent_step_size, self.core.indent.len(), self.core.block_stack, self.content)
    }
}


impl std::fmt::Write for XMLWriter {
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
/// The JSON-implementation of MLLWriter. The JSONWriter has a default indent-step-size of 2 and does
/// auto line-feed, when adding properties or closing blocks. Multiple properties can be passed via
/// the ```add_properties()``` method, but no structural-properties. If a sub-struct as a property has
/// to be added, the ```open_tag()``` has to be used with the property-name as tag-parameter.
#[derive(Debug, Clone)]
pub struct JSONWriter {
    /// Content held by the writer
    pub content: String,
    /// WriterCore in a composition
    pub core: WriterCore,
    /// Counter for comments, interal
    comment_cnt: usize
}


impl Default for JSONWriter {
    fn default() -> Self {
        JSONWriter::new()
    }
}


impl JSONWriter {
    /// Returns a new JSONWriter struct with default indent-step-size of 2.
    pub fn new() -> JSONWriter {
        JSONWriter { 
            content: String::new(),
            core: WriterCore::new(2),
            comment_cnt: 0
        }
    }


    // This method checks the current ending and does correct line-feed, ether with indent-increment or with comma
    fn prepare_property_write(&mut self) {
        // Check the current ending
        if self.content.ends_with('{') {
            // if it is a '{' add a line-feed with indent-increment
            self.line_feed_inc();
        } else if !self.content.is_empty() {
            // there must be at least one property, so separate them by a comma
            self.content.push_str(",\n");
            self.content.push_str(&self.core.indent);
        }
    }
}


// The philosophy here is, only to write the current desired task, nothing more! E.g. open_tag()
// writes only the '{' and nothing else. add_property() writes only the property. If a line feed or indent
// is needed, the method checks the current ending and adds this task before adding the true task.
impl MLLWriter for JSONWriter {
    fn open_tag(&mut self, tag: &str) {
        self.prepare_property_write();
        if !tag.is_empty() {
            self.content.push('\"');
            self.content.push_str(tag);
            self.content.push_str("\":\n");
            self.content.push_str(&self.core.indent);
            self.content.push('{');
        } else {
            self.content.push('{');
        }
    }


    fn open_tag_w_property(&mut self, tag: &str, prop: &str, value: &str) {
        self.open_tag(tag);
        self.add_property(prop, value);
    }

    
    fn close_tag(&mut self) {
        self.core.line_feed_dec(&mut self.content);
        self.content.push('}');
    }

    
    fn single_tag(&mut self, _tag: &str) {
        panic!("there is no single_element in the JSONWriter");
    }


    fn single_tag_w_property(&mut self, tag: &str, prop: &str, value: &str) {
        self.single_tag(tag);
        self.add_property(prop, value);
    }

    
    fn add_property(&mut self, name: &str, value: &str) {
        self.prepare_property_write();
        self.content.push('\"');
        self.content.push_str(name);
        self.content.push_str("\": ");
        self.content.push_str(value);
    }

    
    fn add_properties(&mut self, properties: &Property) {
        properties.p.iter().for_each(|x| self.add_property(&x.0, &x.1) );
    }


    fn add_comment(&mut self, comment: &str) {
        // Increase the comment counter before, because we init it with zero
        self.comment_cnt += 1;
        let prop = "_comment".to_string() + &self.comment_cnt.to_string();
        let value = "\"".to_string() + comment + "\"";
        self.add_property(&prop, &value);
    }


    fn line_feed(&mut self, n: usize) { self.core.line_feed(&mut self.content, n); }
    
    fn line_feed_inc(&mut self) { self.core.line_feed_inc(&mut self.content); }

    fn line_feed_dec(&mut self) { self.core.line_feed_dec(&mut self.content); }
    
    fn inc_indent_step(&mut self) { self.core.inc_indent_step(); }

    fn dec_indent_step(&mut self) { self.core.dec_indent_step(); }

    fn set_indent_step(&mut self, indent_step: usize) { self.core.set_indent_step(indent_step); }

    fn set_indent_step_size(&mut self, indent_step_size: usize) { self.core.set_indent_step_size(indent_step_size); }

    fn clear(&mut self) { 
        self.core.clear(2);
        self.content.clear();
    }
}


impl std::fmt::Display for JSONWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "indent_step_size: {}\nindent: {}\nblock_stack: {:?}\n{}\n",
            self.core.indent_step_size, self.core.indent.len(), self.core.block_stack, self.content)
    }
}


impl std::fmt::Write for JSONWriter {
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
fn assert_html_notation(tag: &str) {
    assert!(tag.chars().all(|c| c.is_ascii_alphanumeric()));
    assert!(tag.chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_lowercase()));
}


// ================================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================================
    // Tests for the WriterCore and the Property-struct
    #[test]
    fn property_basic() {
        let mut prop = Property::new("class", "superhero");
        assert_eq!(prop.p[0], ("class".to_string(), "superhero".to_string()));

        prop.add("style", "width: auto");
        assert_eq!(prop.p[1], ("style".to_string(), "width: auto".to_string()));
    }

    #[test]
    fn writercore_indent_methods() {
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
    fn html_new_n_clear() {
        let mut wr = HTMLWriter::new();
        assert_eq!(wr.content, "");
        assert_eq!(wr.core.indent_step_size, 4);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());

        wr.open_tag("div");
        wr.set_indent_step(4);
        wr.set_indent_step_size(8);
        wr.clear();
        assert_eq!(wr.content, "");
        assert_eq!(wr.core.indent_step_size, 4);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());
    }

    #[test]
    fn html_single_element() {
        let mut wr = HTMLWriter::new();
        wr.single_tag("img");
        assert_eq!(wr.content, "<img>".to_string());
    }

    #[test]
    fn html_dual_elements() {
        let mut wr = HTMLWriter::new();
        wr.open_tag("div");
        wr.close_tag();
        assert_eq!(wr.content, "<div></div>".to_string());

        wr.clear();
        wr.open_tag_w_property("div", "class", "container");
        assert_eq!(wr.content, "<div class=\"container\">");
    }

    #[test]
    fn html_mixed_entries() {
        let mut wr = HTMLWriter::new();
        wr.open_tag("div");
        wr.add_property("class", "container");
        wr.line_feed_inc();
        wr.single_tag("img");
        wr.add_property("style", "width: auto");
        wr.line_feed_dec();
        wr.close_tag();
        assert_eq!(wr.content, "<div class=\"container\">\n    <img style=\"width: auto\">\n</div>")
    }

    #[test]
    fn html_property_string() {
        let mut properties = Property::new("class", "container");
        properties.add("style", "width: auto");
        let mut wr = HTMLWriter::new();
        wr.single_tag("img");
        wr.add_properties(&properties);
        assert_eq!(wr.content, "<img class=\"container\" style=\"width: auto\">".to_string());

        wr.clear();
        wr.single_tag("img");
        wr.add_property("style", "width: auto");
        assert_eq!(wr.content, "<img style=\"width: auto\">");
    }

    // ============================================================================================
    // Tests for the XMLWriter
    #[test]
    fn xml_new_n_clear() {
        let mut wr = XMLWriter::new();
        assert_eq!(wr.content, "");
        assert_eq!(wr.core.indent_step_size, 2);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());

        wr.open_tag("div");
        wr.set_indent_step(4);
        wr.set_indent_step_size(8);
        wr.clear();
        assert_eq!(wr.content, "");
        assert_eq!(wr.core.indent_step_size, 2);
        assert_eq!(wr.core.indent, "");
        assert_eq!(wr.core.block_stack, Vec::<String>::new());
    }

    #[test]
    fn xml_single_element() {
        let mut wr = XMLWriter::new();
        wr.single_tag("img");
        assert_eq!(wr.content, "<img>".to_string());
    }

    #[test]
    fn xml_dual_elements() {
        let mut wr = XMLWriter::new();
        wr.open_tag("div");
        wr.close_tag();
        assert_eq!(wr.content, "<div></div>".to_string());

        wr.clear();
        wr.open_tag_w_property("div", "class", "container");
        assert_eq!(wr.content, "<div class=\"container\">");
    }

    #[test]
    fn xml_mixed_entries() {
        let mut wr = XMLWriter::new();
        wr.open_tag("div");
        wr.add_property("class", "container");
        wr.line_feed_inc();
        wr.single_tag("img");
        wr.add_property("style", "width: auto");
        wr.line_feed_dec();
        wr.close_tag();
        assert_eq!(wr.content, "<div class=\"container\">\n  <img style=\"width: auto\">\n</div>")
    }

    #[test]
    fn xml_property_string() {
        let mut properties = Property::new("class", "container");
        properties.add("style", "width: auto");
        let mut wr = XMLWriter::new();
        wr.single_tag("img");
        wr.add_properties(&properties);
        assert_eq!(wr.content, "<img class=\"container\" style=\"width: auto\">".to_string());

        wr.clear();
        wr.single_tag("img");
        wr.add_property("style", "width: auto");
        assert_eq!(wr.content, "<img style=\"width: auto\">");
    }

    // ============================================================================================
    #[test]
    #[should_panic(expected = "there is no single_element in the JSONWriter")]
    fn json_single_element() {
        let mut wr = JSONWriter::new();
        wr.single_tag("img");    
    }

    #[test]
    fn json_dual_elements() {
        let mut wr = JSONWriter::new();
        wr.open_tag("");
        wr.close_tag();
        assert_eq!(wr.content, "{\n}".to_string());

        wr.clear();
        wr.open_tag_w_property("", "Name", "\"Mustermann\"");
        assert_eq!(wr.content, "{\n  \"Name\": \"Mustermann\"");
    }

    #[test]
    fn json_mixed_entries() {
        let mut wr = JSONWriter::new();
        wr.open_tag("");
        wr.add_property("Name", "\"Eberhardt\"");
        wr.add_property("Vorname", "\"Michael\"");
        wr.open_tag("Daten");
        wr.add_property("Geburtstag", "\"03.10.1985\"");
        wr.close_tag();
        wr.close_tag();
        assert_eq!(wr.content, 
            "{\n  \"Name\": \"Eberhardt\",\n  \"Vorname\": \"Michael\",\n  \"Daten\":\n  {\n    \"Geburtstag\": \"03.10.1985\"\n  }\n}"
        );
    }

    #[test]
    fn json_property_string() {
        let mut properties = Property::new("Name", "\"Eberhardt\"");
        properties.add("Alter", "35");
        let mut wr = JSONWriter::new();
        wr.open_tag("");
        wr.add_properties(&properties);
        wr.close_tag();
        assert_eq!(wr.content, "{\n  \"Name\": \"Eberhardt\",\n  \"Alter\": 35\n}".to_string());

        wr.clear();
        assert_eq!(wr.content, "");
    }

}


// ================================================================================================