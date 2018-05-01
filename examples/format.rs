extern crate annotate_snippets;

use annotate_snippets::snippet::{Snippet, Slice, Annotation, TitleAnnotation, AnnotationType};
use annotate_snippets::display_list::DisplayList;

fn main() {
    let snippet = Snippet {
        slice: Slice {
            source: r#") -> Option<String> {
    for ann in annotations {
        match (ann.range.0, ann.range.1) {
            (None, None) => continue,
            (Some(start), Some(end)) if start > end_index => continue,
            (Some(start), Some(end)) if start >= start_index => {
                let label = if let Some(ref label) = ann.label {
                    format!(" {}", label)
                } else {
                    String::from("")
                };

                return Some(format!(
                    "{}{}{}",
                    " ".repeat(start - start_index),
                    "^".repeat(end - start),
                    label
                ));
            }
            _ => continue,
        }
    }"#.to_string(),
         line_start: 51,
         origin: Some("src/format.rs".to_string())
         },
         title: Some(TitleAnnotation {
             label: Some("mismatched types".to_string()),
             id: Some("E0308".to_string()),
             annotation_type: AnnotationType::Error,
         }),
         fold: Some(false),
         annotations: vec![
             Annotation {
                 label: "expected `Option<String>` because of return type".to_string(),
                 annotation_type: AnnotationType::Warning,
                 range: (6, 20)
             },
             // Annotation {
             //     label: "expected enum `std::option::Option".to_string(),
             //     annotation_type: AnnotationType::Error,
             //     range: (23, 787)
             // },
         ]
        };

    println!("{}", DisplayList::from(snippet));
}