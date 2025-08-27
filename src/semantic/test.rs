use crate::parser::parse;
use crate::semantic::validate;
use crate::pretty::{annotate, remove_annotation};

fn assert_error(expected_annotated: &str) {
    let clean_source = remove_annotation(expected_annotated);
    let ast = parse(&clean_source).expect("Parse error");
    let Err(error) = validate(ast) else {
        panic!("No error produced!")
    };
    let actual_annotated = annotate(&clean_source, &error);
    assert_eq!(actual_annotated, expected_annotated);
}

mod test_chapter_5;
mod test_chapter_6;
mod test_chapter_7;
mod test_chapter_8;
mod test_chapter_9;
mod test_chapter_10;
mod test_chapter_11;
mod test_chapter_12;
mod test_chapter_13;
mod test_chapter_14;
mod test_chapter_15;
mod test_chapter_16;
mod test_chapter_17;
mod test_chapter_18;
