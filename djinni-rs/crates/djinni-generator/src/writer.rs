// vinsg
// IndentWriter - manages indented code output, translated from writer.scala

pub struct IndentWriter {
    buf: String,
    indent: String,
    _start_indent: String,
    current_indent: String,
    start_of_line: bool,
}

impl IndentWriter {
    pub fn new() -> Self {
        Self::with_indent("    ", "")
    }

    pub fn with_indent(indent: &str, start_indent: &str) -> Self {
        Self {
            buf: String::new(),
            indent: indent.to_string(),
            _start_indent: start_indent.to_string(),
            current_indent: start_indent.to_string(),
            start_of_line: true,
        }
    }

    pub fn into_string(self) -> String {
        self.buf
    }

    pub fn as_str(&self) -> &str {
        &self.buf
    }

    /// Write a string (with indentation if at start of line)
    pub fn w(&mut self, s: &str) -> &mut Self {
        if self.start_of_line {
            self.buf.push_str(&self.current_indent);
            self.start_of_line = false;
        }
        self.buf.push_str(s);
        self
    }

    /// Write a newline
    pub fn wl_empty(&mut self) -> &mut Self {
        self.buf.push('\n');
        self.start_of_line = true;
        self
    }

    /// Write a string followed by a newline
    pub fn wl(&mut self, s: &str) -> &mut Self {
        self.w(s);
        self.wl_empty()
    }

    /// Write a string with one level less indentation, then restore
    pub fn wl_outdent(&mut self, s: &str) -> &mut Self {
        self.decrease();
        self.wl(s);
        self.increase();
        self
    }

    pub fn increase(&mut self) {
        self.current_indent.push_str(&self.indent);
    }

    pub fn decrease(&mut self) {
        let new_len = self.current_indent.len() - self.indent.len();
        self.current_indent.truncate(new_len);
    }

    pub fn nested<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.increase();
        f(self);
        self.decrease();
    }

    pub fn nested_n<F: FnOnce(&mut Self)>(&mut self, amount: usize, f: F) {
        for _ in 0..amount {
            self.increase();
        }
        f(self);
        for _ in 0..amount {
            self.decrease();
        }
    }

    pub fn braced_end<F: FnOnce(&mut Self)>(&mut self, end: &str, f: F) {
        if self.start_of_line {
            self.wl("{");
        } else {
            self.wl(" {");
        }
        self.nested(f);
        self.wl(&format!("}}{}", end));
    }

    pub fn braced<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.braced_end("", f);
    }

    pub fn braced_semi<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.braced_end(";", f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_write() {
        let mut w = IndentWriter::new();
        w.wl("hello");
        w.wl("world");
        assert_eq!(w.into_string(), "hello\nworld\n");
    }

    #[test]
    fn test_nested_indent() {
        let mut w = IndentWriter::new();
        w.wl("outer");
        w.nested(|w| {
            w.wl("inner");
        });
        w.wl("outer again");
        assert_eq!(w.into_string(), "outer\n    inner\nouter again\n");
    }

    #[test]
    fn test_braced() {
        let mut w = IndentWriter::new();
        w.w("struct Foo");
        w.braced(|w| {
            w.wl("int x;");
        });
        assert_eq!(w.into_string(), "struct Foo {\n    int x;\n}\n");
    }

    #[test]
    fn test_braced_semi() {
        let mut w = IndentWriter::new();
        w.w("struct Foo");
        w.braced_semi(|w| {
            w.wl("int x;");
        });
        assert_eq!(w.into_string(), "struct Foo {\n    int x;\n};\n");
    }

    #[test]
    fn test_empty_line() {
        let mut w = IndentWriter::new();
        w.wl("a");
        w.wl_empty();
        w.wl("b");
        assert_eq!(w.into_string(), "a\n\nb\n");
    }

    #[test]
    fn test_start_of_line_braced() {
        let mut w = IndentWriter::new();
        w.braced(|w| {
            w.wl("content");
        });
        assert_eq!(w.into_string(), "{\n    content\n}\n");
    }

    #[test]
    fn test_double_nested() {
        let mut w = IndentWriter::new();
        w.wl("level0");
        w.nested(|w| {
            w.wl("level1");
            w.nested(|w| {
                w.wl("level2");
            });
        });
        assert_eq!(w.into_string(), "level0\n    level1\n        level2\n");
    }

    #[test]
    fn test_wl_outdent() {
        let mut w = IndentWriter::new();
        w.nested(|w| {
            w.wl("normal");
            w.wl_outdent("outdented");
            w.wl("normal again");
        });
        assert_eq!(
            w.into_string(),
            "    normal\noutdented\n    normal again\n"
        );
    }

    #[test]
    fn test_w_inline() {
        let mut w = IndentWriter::new();
        w.w("a");
        w.w("b");
        w.wl("c");
        assert_eq!(w.into_string(), "abc\n");
    }

    #[test]
    fn test_yaml_indent() {
        let mut w = IndentWriter::with_indent("  ", "");
        w.wl("---");
        w.wl("name: foo");
        w.w("cpp:");
        w.wl_empty();
        w.nested(|w| {
            w.wl("typename: 'int'");
        });
        assert_eq!(w.into_string(), "---\nname: foo\ncpp:\n  typename: 'int'\n");
    }
}
