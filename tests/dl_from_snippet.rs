use annotate_snippets::{display_list as dl, formatter::get_term_style, snippet};

#[test]
fn test_format_title() {
    let input = snippet::Snippet {
        title: Some(snippet::Annotation {
            id: Some("E0001".to_string()),
            label: Some("This is a title".to_string()),
            annotation_type: snippet::AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![],
        opt: Default::default(),
    };
    let output = dl::DisplayList {
        body: vec![dl::DisplayLine::Raw(dl::DisplayRawLine::Annotation {
            annotation: dl::Annotation {
                annotation_type: dl::DisplayAnnotationType::Error,
                id: Some("E0001".to_string()),
                label: vec![dl::DisplayTextFragment {
                    content: "This is a title".to_string(),
                    style: dl::DisplayTextStyle::Emphasis,
                }],
            },
            source_aligned: false,
            continuation: false,
        })],
        stylesheet: get_term_style(input.opt.color),
        anonymized_line_numbers: input.opt.anonymized_line_numbers,
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_slice() {
    let line_1 = "This is line 1".to_string();
    let line_2 = "This is line 2".to_string();
    let source = vec![line_1.clone(), line_2.clone()].join("\n");
    let input = snippet::Snippet {
        title: None,
        footer: vec![],
        slices: vec![snippet::Slice {
            source: source.clone(),
            line_start: 5402,
            origin: None,
            annotations: vec![],
            fold: false,
        }],
        opt: Default::default(),
    };
    let output = dl::DisplayList {
        body: vec![
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(5402),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: line_1.clone(),
                    range: (0, line_1.len()),
                },
            },
            dl::DisplayLine::Source {
                lineno: Some(5403),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    range: (line_1.len() + 1, source.len()),
                    text: line_2,
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
        ],
        stylesheet: get_term_style(input.opt.color),
        anonymized_line_numbers: input.opt.anonymized_line_numbers,
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_slices_continuation() {
    let src_0 = "This is slice 1".to_string();
    let src_0_len = src_0.len();
    let src_1 = "This is slice 2".to_string();
    let src_1_len = src_1.len();
    let input = snippet::Snippet {
        title: None,
        footer: vec![],
        slices: vec![
            snippet::Slice {
                source: src_0.clone(),
                line_start: 5402,
                origin: Some("file1.rs".to_string()),
                annotations: vec![],
                fold: false,
            },
            snippet::Slice {
                source: src_1.clone(),
                line_start: 2,
                origin: Some("file2.rs".to_string()),
                annotations: vec![],
                fold: false,
            },
        ],
        opt: Default::default(),
    };
    let output = dl::DisplayList {
        body: vec![
            dl::DisplayLine::Raw(dl::DisplayRawLine::Origin {
                path: "file1.rs".to_string(),
                pos: None,
                header_type: dl::DisplayHeaderType::Initial,
            }),
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(5402),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: src_0,
                    range: (0, src_0_len),
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Raw(dl::DisplayRawLine::Origin {
                path: "file2.rs".to_string(),
                pos: None,
                header_type: dl::DisplayHeaderType::Continuation,
            }),
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(2),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: src_1,
                    range: (0, src_1_len),
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
        ],
        stylesheet: get_term_style(input.opt.color),
        anonymized_line_numbers: input.opt.anonymized_line_numbers,
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_slice_annotation_standalone() {
    let line_1 = "This is line 1".to_string();
    let line_2 = "This is line 2".to_string();
    let source = vec![line_1.clone(), line_2.clone()].join("\n");
    // In line 2
    let range = (22, 24);
    let input = snippet::Snippet {
        title: None,
        footer: vec![],
        slices: vec![snippet::Slice {
            source: source.clone(),
            line_start: 5402,
            origin: None,
            annotations: vec![snippet::SourceAnnotation {
                range,
                label: "Test annotation".to_string(),
                annotation_type: snippet::AnnotationType::Info,
            }],
            fold: false,
        }],
        opt: Default::default(),
    };
    let output = dl::DisplayList {
        body: vec![
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(5402),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    range: (0, line_1.len()),
                    text: line_1.clone(),
                },
            },
            dl::DisplayLine::Source {
                lineno: Some(5403),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    range: (line_1.len() + 1, source.len()),
                    text: line_2,
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Annotation {
                    annotation: dl::Annotation {
                        annotation_type: dl::DisplayAnnotationType::Info,
                        id: None,
                        label: vec![dl::DisplayTextFragment {
                            content: "Test annotation".to_string(),
                            style: dl::DisplayTextStyle::Regular,
                        }],
                    },
                    range: (range.0 - (line_1.len() + 1), range.1 - (line_1.len() + 1)),
                    annotation_type: dl::DisplayAnnotationType::Info,
                    annotation_part: dl::DisplayAnnotationPart::Standalone,
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
        ],
        stylesheet: get_term_style(input.opt.color),
        anonymized_line_numbers: input.opt.anonymized_line_numbers,
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_label() {
    let input = snippet::Snippet {
        title: None,
        footer: vec![snippet::Annotation {
            id: None,
            label: Some("This __is__ a title".to_string()),
            annotation_type: snippet::AnnotationType::Error,
        }],
        slices: vec![],
        opt: Default::default(),
    };
    let output = dl::DisplayList {
        body: vec![dl::DisplayLine::Raw(dl::DisplayRawLine::Annotation {
            annotation: dl::Annotation {
                annotation_type: dl::DisplayAnnotationType::Error,
                id: None,
                label: vec![
                    dl::DisplayTextFragment {
                        content: "This ".to_string(),
                        style: dl::DisplayTextStyle::Regular,
                    },
                    dl::DisplayTextFragment {
                        content: "is".to_string(),
                        style: dl::DisplayTextStyle::Emphasis,
                    },
                    dl::DisplayTextFragment {
                        content: " a title".to_string(),
                        style: dl::DisplayTextStyle::Regular,
                    },
                ],
            },
            source_aligned: true,
            continuation: false,
        })],
        stylesheet: get_term_style(input.opt.color),
        anonymized_line_numbers: input.opt.anonymized_line_numbers,
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
#[should_panic]
fn test_i26() {
    let source = "short".to_string();
    let label = "label".to_string();
    let input = snippet::Snippet {
        title: None,
        footer: vec![],
        slices: vec![snippet::Slice {
            annotations: vec![snippet::SourceAnnotation {
                range: (0, source.len() + 1),
                label,
                annotation_type: snippet::AnnotationType::Error,
            }],
            source,
            line_start: 0,
            origin: None,
            fold: false,
        }],
        opt: Default::default(),
    };

    let _ = dl::DisplayList::from(input);
}