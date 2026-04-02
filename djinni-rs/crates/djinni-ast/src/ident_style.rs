// vinsg
// Identifier style conversion utilities, translated from generator.scala

pub type IdentConverter = Box<dyn Fn(&str) -> String + Send + Sync>;

fn first_upper(token: &str) -> String {
    if token.is_empty() {
        return String::new();
    }
    let mut chars = token.chars();
    let first = chars.next().unwrap().to_uppercase().to_string();
    first + chars.as_str()
}

fn leading_upper_strict(token: &str) -> String {
    if token.is_empty() {
        return String::new();
    }
    let head = token.chars().next().unwrap().to_uppercase().to_string();
    let tail = &token[token.chars().next().unwrap().len_utf8()..];
    // Preserve mixed case identifiers like 'XXFoo':
    // Convert tail to lowercase only when it is full uppercase.
    if tail == tail.to_uppercase() {
        head + &tail.to_lowercase()
    } else {
        head + tail
    }
}

pub fn camel_upper(s: &str) -> String {
    s.split(&['-', '_'][..])
        .map(|part| first_upper(part))
        .collect()
}

pub fn camel_lower(s: &str) -> String {
    let parts: Vec<&str> = s.split('_').collect();
    if parts.is_empty() {
        return String::new();
    }
    let mut result = parts[0].to_string();
    for part in &parts[1..] {
        result.push_str(&first_upper(part));
    }
    result
}

pub fn under_lower(s: &str) -> String {
    s.to_string()
}

pub fn under_upper(s: &str) -> String {
    s.split('_')
        .map(|part| first_upper(part))
        .collect::<Vec<_>>()
        .join("_")
}

pub fn under_caps(s: &str) -> String {
    s.to_uppercase()
}

pub fn camel_upper_strict(s: &str) -> String {
    s.split(&['-', '_'][..])
        .map(|part| leading_upper_strict(part))
        .collect()
}

pub fn camel_lower_strict(s: &str) -> String {
    let parts: Vec<&str> = s.split('_').collect();
    if parts.is_empty() {
        return String::new();
    }
    let mut result = parts[0].to_lowercase();
    for part in &parts[1..] {
        result.push_str(&leading_upper_strict(part));
    }
    result
}

pub fn under_lower_strict(s: &str) -> String {
    s.to_lowercase()
}

pub fn under_upper_strict(s: &str) -> String {
    s.split('_')
        .map(|part| leading_upper_strict(part))
        .collect::<Vec<_>>()
        .join("_")
}

pub fn prefix_converter(pfx: String, inner: fn(&str) -> String) -> IdentConverter {
    Box::new(move |s: &str| format!("{}{}", pfx, inner(s)))
}

pub fn suffix_converter(inner: fn(&str) -> String, sfx: String) -> IdentConverter {
    Box::new(move |s: &str| format!("{}{}", inner(s), sfx))
}

pub fn prefix_suffix_converter(pfx: String, inner: fn(&str) -> String, sfx: String) -> IdentConverter {
    Box::new(move |s: &str| format!("{}{}{}", pfx, inner(s), sfx))
}

/// Infer an IdentConverter from a style string like "mFooBar", "foo_bar!_native", etc.
/// Returns None if no known style pattern is found.
pub fn infer(input: &str) -> Option<IdentConverter> {
    // Ordered: check longest patterns first to avoid partial matches
    let styles: Vec<(&str, fn(&str) -> String)> = vec![
        ("FOO_BAR!", under_caps),
        ("FooBar!", camel_upper_strict),
        ("fooBar!", camel_lower_strict),
        ("foo_bar!", under_lower_strict),
        ("Foo_Bar!", under_upper_strict),
        ("FOO_BAR", under_caps),
        ("FooBar", camel_upper),
        ("fooBar", camel_lower),
        ("foo_bar", under_lower),
        ("Foo_Bar", under_upper),
    ];

    for (pattern, func) in styles {
        if let Some(index) = input.find(pattern) {
            let before = &input[..index];
            let after = &input[index + pattern.len()..];

            match (before.is_empty(), after.is_empty()) {
                (true, true) => return Some(Box::new(func)),
                (false, true) => {
                    let pfx = before.to_string();
                    return Some(prefix_converter(pfx, func));
                }
                (true, false) => {
                    let sfx = after.to_string();
                    return Some(suffix_converter(func, sfx));
                }
                (false, false) => {
                    let pfx = before.to_string();
                    let sfx = after.to_string();
                    return Some(prefix_suffix_converter(pfx, func, sfx));
                }
            }
        }
    }

    None
}

// Language-specific identifier style collections

pub struct CppIdentStyle {
    pub ty: fn(&str) -> String,
    pub enum_type: fn(&str) -> String,
    pub type_param: fn(&str) -> String,
    pub method: fn(&str) -> String,
    pub field: fn(&str) -> String,
    pub local: fn(&str) -> String,
    pub enum_: fn(&str) -> String,
    pub const_: fn(&str) -> String,
}

pub struct JavaIdentStyle {
    pub ty: IdentConverter,
    pub type_param: fn(&str) -> String,
    pub method: fn(&str) -> String,
    pub field: IdentConverter,
    pub local: fn(&str) -> String,
    pub enum_: fn(&str) -> String,
    pub const_: fn(&str) -> String,
}

pub struct ObjcIdentStyle {
    pub ty: fn(&str) -> String,
    pub type_param: fn(&str) -> String,
    pub method: fn(&str) -> String,
    pub field: fn(&str) -> String,
    pub local: fn(&str) -> String,
    pub enum_: fn(&str) -> String,
    pub const_: fn(&str) -> String,
}

pub struct JsIdentStyle {
    pub ty: fn(&str) -> String,
    pub type_param: fn(&str) -> String,
    pub method: fn(&str) -> String,
    pub field: fn(&str) -> String,
    pub local: fn(&str) -> String,
    pub enum_: fn(&str) -> String,
    pub const_: fn(&str) -> String,
}

pub fn cpp_default() -> CppIdentStyle {
    CppIdentStyle {
        ty: camel_upper,
        enum_type: camel_upper,
        type_param: camel_upper,
        method: under_lower,
        field: under_lower,
        local: under_lower,
        enum_: under_caps,
        const_: under_caps,
    }
}

pub fn java_default() -> JavaIdentStyle {
    JavaIdentStyle {
        ty: Box::new(camel_upper),
        type_param: camel_upper,
        method: camel_lower,
        field: Box::new(camel_lower),
        local: camel_lower,
        enum_: camel_upper,
        const_: under_caps,
    }
}

pub fn objc_default() -> ObjcIdentStyle {
    ObjcIdentStyle {
        ty: camel_upper,
        type_param: camel_upper,
        method: camel_lower,
        field: camel_lower,
        local: camel_lower,
        enum_: camel_upper,
        const_: camel_upper,
    }
}

pub fn js_default() -> JsIdentStyle {
    JsIdentStyle {
        ty: camel_upper,
        type_param: camel_upper,
        method: camel_lower,
        field: camel_lower,
        local: camel_lower,
        enum_: under_caps,
        const_: under_caps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_upper() {
        assert_eq!(camel_upper("foo_bar"), "FooBar");
        assert_eq!(camel_upper("foo_bar_baz"), "FooBarBaz");
        assert_eq!(camel_upper("foo"), "Foo");
        assert_eq!(camel_upper("FOO_BAR"), "FOOBAR");
    }

    #[test]
    fn test_camel_lower() {
        assert_eq!(camel_lower("foo_bar"), "fooBar");
        assert_eq!(camel_lower("foo_bar_baz"), "fooBarBaz");
        assert_eq!(camel_lower("foo"), "foo");
    }

    #[test]
    fn test_under_lower() {
        assert_eq!(under_lower("foo_bar"), "foo_bar");
    }

    #[test]
    fn test_under_upper() {
        assert_eq!(under_upper("foo_bar"), "Foo_Bar");
        assert_eq!(under_upper("foo_bar_baz"), "Foo_Bar_Baz");
    }

    #[test]
    fn test_under_caps() {
        assert_eq!(under_caps("foo_bar"), "FOO_BAR");
    }

    #[test]
    fn test_camel_upper_strict() {
        assert_eq!(camel_upper_strict("foo_bar"), "FooBar");
        assert_eq!(camel_upper_strict("FOO_BAR"), "FooBar");
        // XXFoo has mixed-case tail "XFoo", so tail is preserved
        assert_eq!(camel_upper_strict("XXFoo"), "XXFoo");
        // FOO_BAR: each part is all-caps, so gets lowered
        assert_eq!(camel_upper_strict("FOO_BAR"), "FooBar");
    }

    #[test]
    fn test_camel_lower_strict() {
        assert_eq!(camel_lower_strict("foo_bar"), "fooBar");
        assert_eq!(camel_lower_strict("FOO_BAR"), "fooBar");
    }

    #[test]
    fn test_infer_plain_styles() {
        let conv = infer("FooBar").unwrap();
        assert_eq!(conv("my_ident"), "MyIdent");

        let conv = infer("fooBar").unwrap();
        assert_eq!(conv("my_ident"), "myIdent");

        let conv = infer("foo_bar").unwrap();
        assert_eq!(conv("my_ident"), "my_ident");

        let conv = infer("FOO_BAR").unwrap();
        assert_eq!(conv("my_ident"), "MY_IDENT");

        let conv = infer("Foo_Bar").unwrap();
        assert_eq!(conv("my_ident"), "My_Ident");
    }

    #[test]
    fn test_infer_with_prefix() {
        let conv = infer("mFooBar").unwrap();
        assert_eq!(conv("my_ident"), "mMyIdent");
    }

    #[test]
    fn test_infer_with_suffix() {
        let conv = infer("FooBarNative").unwrap();
        assert_eq!(conv("my_ident"), "MyIdentNative");
    }

    #[test]
    fn test_infer_strict_with_suffix() {
        let conv = infer("FooBar!Native").unwrap();
        assert_eq!(conv("my_ident"), "MyIdentNative");
        assert_eq!(conv("FOO_BAR"), "FooBarNative");
    }

    #[test]
    fn test_infer_with_prefix_and_suffix() {
        let conv = infer("NativeFooBar").unwrap();
        assert_eq!(conv("my_ident"), "NativeMyIdent");
    }

    #[test]
    fn test_infer_strict_prefix() {
        let conv = infer("mFooBar!").unwrap();
        assert_eq!(conv("my_ident"), "mMyIdent");
        assert_eq!(conv("FOO_BAR"), "mFooBar");
    }

    #[test]
    fn test_infer_under_strict_with_suffix() {
        let conv = infer("foo_bar!_native").unwrap();
        assert_eq!(conv("MY_IDENT"), "my_ident_native");
    }

    #[test]
    fn test_infer_none() {
        assert!(infer("xyz").is_none());
    }
}
