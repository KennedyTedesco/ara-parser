use ara_parser::parser;
use ara_parser::tree::describe_tree;
use ara_reporting::error::Error;
use ara_source::source::{Source, SourceKind};

fn main() -> Result<(), Error> {
    let code = r#"
        class Foo {
          public function bar(u8 $a, u8 $b): u16 {
             $result = $a + $b;
             $result
          }
        }
    "#;

    let source = Source::inline(SourceKind::Script, code);

    let tree = parser::parse(&source).unwrap();

    println!("{}", code);
    print!("{}", describe_tree(&tree));

    Ok(())
}
