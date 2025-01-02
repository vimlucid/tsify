pub struct Namespace<'a>(pub &'a str);

pub fn wrap(item: &str, namespace: Namespace) -> String {
    // TODO Declare?
    format!("namespace {} {{\n{}\n}}", namespace.0, item)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_wrap() {
        let wrapped = wrap("inner", Namespace("some_ns"));
        let expected = indoc! {r#"
            namespace some_ns {
            inner
            }"#
        };
        assert_eq!(wrapped, expected);
    }
}