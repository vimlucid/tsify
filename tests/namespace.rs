use indoc::indoc;
use pretty_assertions::assert_eq;
use tsify_next::Tsify;

// TODO Incompatible attributes namespaced_variants and namespace

#[test]
fn test_struct_in_namespace() {
    #[derive(Tsify)]
    #[tsify(namespace="some_ns")]
    struct Test {
        a: i32,
        b: u32,
    }

    let expected = indoc! {r#"
        namespace some_ns {
        export interface Test {
            a: number;
            b: number;
        }
        }"#
    };

    assert_eq!(Test::DECL, expected);
}