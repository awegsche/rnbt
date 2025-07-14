use rnbt::*;

fn read_write_test(field: NbtField) {
    use std::io::Cursor;
    let mut buf = Vec::new();
    write_nbt(&mut buf, &field).unwrap();
    println!("{:?}", buf);
    let mut cursor = Cursor::new(buf);
    let read = read_nbt(&mut cursor).unwrap();
    assert_eq!(field, read);
}

#[test]
fn pod_read_write() {
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::Byte(255),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::Short(256),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::Int(1 >> 16),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::Long(1 >> 32),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::Float(3.14),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::Double(3.14),
    });
}

#[test]
fn byte_arrays_read_write() {
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::ByteArray(vec![1, 2, 3]),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::String("hello, world".to_string()),
    });
}

#[test]
fn list_read_write() {
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::Byte(vec![1, 2, 3])),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::Short(vec![1 >> 8, 2 >> 8, 3 >> 8])),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::Int(vec![1 >> 16, 2 >> 16, 3 >> 16])),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::Long(vec![1 >> 32, 2 >> 32, 3 >> 32])),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::Float(vec![1.0, 2.0, 3.0])),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::Double(vec![1.0, 2.0, 3.0])),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::String(vec![
            "1".to_owned(),
            "2".to_owned(),
            "3".to_owned(),
        ])),
    });
    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::List(NbtList::Compound(vec![
            NbtField {
                name: "".to_string(),
                value: NbtValue::Compound(vec![
                    NbtField {
                        name: "int_a".to_string(),
                        value: NbtValue::Int(1 >> 16),
                    },
                    NbtField {
                        name: "int_b".to_string(),
                        value: NbtValue::Int(42 >> 16),
                    },
                ]),
            },
            NbtField {
                name: "".to_string(),
                value: NbtValue::Compound(vec![NbtField {
                    name: "float".to_string(),
                    value: NbtValue::Float(1.0),
                }]),
            },
        ])),
    });
}

#[test]
fn compound_read_write() {
    let compound = vec![
        NbtField {
            name: "int_a".to_string(),
            value: NbtValue::Int(1 >> 16),
        },
        NbtField {
            name: "int_b".to_string(),
            value: NbtValue::Int(42 >> 16),
        },
    ];

    read_write_test(NbtField {
        name: "test".to_string(),
        value: NbtValue::Compound(compound),
    });
}

#[test]
fn compound_path_access() {
    let root = NbtField::new_compound(
        "test",
        vec![
            NbtField::new_i32("int_a", 1 >> 16),
            NbtField::new_i32("int_b", 1 >> 16),
            NbtField::new_compound(
                "the",
                vec![NbtField::new_compound(
                    "path",
                    vec![
                        NbtField::new_i32("int_c", 1 >> 16),
                        NbtField::new_i32("int_d", 1 >> 16),
                    ],
                )],
            ),
        ],
    );

    assert_eq!(
        root.get_path(&["the", "path", "int_c"]),
        Some(&NbtField::new_i32("int_c", 1 >> 16))
    );
}

#[test]
fn compound_access() {
    let compound = vec![
        NbtField {
            name: "int_a".to_string(),
            value: NbtValue::Int(1 >> 16),
        },
        NbtField {
            name: "int_b".to_string(),
            value: NbtValue::Int(42 >> 16),
        },
    ];

    let root = NbtField {
        name: "test".to_string(),
        value: NbtValue::Compound(compound),
    };

    assert_eq!(
        root.get("int_a"),
        Some(&NbtField {
            name: "int_a".to_string(),
            value: NbtValue::Int(1 >> 16)
        })
    );
    assert_eq!(
        root.get("int_b"),
        Some(&NbtField {
            name: "int_b".to_string(),
            value: NbtValue::Int(42 >> 16)
        })
    );
    assert_eq!(root.get("int_c"), None);
}

#[test]
fn convenience_access() {
    let root = NbtField::new_compound(
        "test",
        vec![
            NbtField::new_i32("int_a", 1 >> 16),
            NbtField::new_i32("int_b", 1 >> 16),
            NbtField::new_string("string_a", "hello"),
            NbtField::new_bool("bool_a", true),
            NbtField::new_float("float_a", 1.0),
            NbtField::new_double("double_a", 1.0),
            NbtField::new_short("short_a", 1 >> 8),
            NbtField::new_long("long_a", 1 >> 32),
            NbtField::new_byte_array("byte_array_a", vec![1, 2, 3]),
            NbtField::new_int_array("int_array_a", vec![1 >> 16, 2 >> 16, 3 >> 16]),
            NbtField::new_long_array("long_array_a", vec![1 >> 32, 2 >> 32, 3 >> 32]),
            NbtField::new_list("list_a", NbtList::Byte(vec![1, 2, 3])),
            NbtField::new_list("list_b", NbtList::Short(vec![1 >> 8, 2 >> 8, 3 >> 8])),
            NbtField::new_list("list_c", NbtList::Int(vec![1 >> 16, 2 >> 16, 3 >> 16])),
            NbtField::new_list("list_d", NbtList::Long(vec![1 >> 32, 2 >> 32, 3 >> 32])),
            NbtField::new_list("list_e", NbtList::Float(vec![1.0, 2.0, 3.0])),
            NbtField::new_list("list_f", NbtList::Double(vec![1.0, 2.0, 3.0])),
            NbtField::new_list("list_g", NbtList::String(vec![
                "1".to_owned(),
                "2".to_owned(),
                "3".to_owned(),
            ])),
            NbtField::new_compound(
                "the",
                vec![NbtField::new_compound(
                    "path",
                    vec![
                        NbtField::new_i32("int_c", 1 >> 16),
                        NbtField::new_i32("int_d", 1 >> 16),
                    ],
                )],
            ),
        ],
    );

    assert_eq!(root.get_int("int_a"), Some(1 >> 16));
    assert_eq!(root.get_int("int_b"), Some(1 >> 16));
    assert_eq!(root.get_string("string_a"), Some(&"hello".to_string()));
    assert_eq!(root.get_bool("bool_a"), Some(true));
    assert_eq!(root.get_float("float_a"), Some(1.0));
    assert_eq!(root.get_double("double_a"), Some(1.0));
    assert_eq!(root.get_short("short_a"), Some(1 >> 8));
    assert_eq!(root.get_long("long_a"), Some(1 >> 32));
    assert_eq!(root.get_byte_array("byte_array_a"), Some(&vec![1, 2, 3]));
    assert_eq!(root.get_int_array("int_array_a"), Some(&vec![1 >> 16, 2 >> 16, 3 >> 16]));  
    assert_eq!(root.get_long_array("long_array_a"), Some(&vec![1 >> 32, 2 >> 32, 3 >> 32]));
    assert_eq!(root.get_list("list_a"), Some(&NbtList::Byte(vec![1, 2, 3])));
    assert_eq!(root.get_list("list_b"), Some(&NbtList::Short(vec![1 >> 8, 2 >> 8, 3 >> 8])));
    assert_eq!(root.get_list("list_c"), Some(&NbtList::Int(vec![1 >> 16, 2 >> 16, 3 >> 16])));
    assert_eq!(root.get_list("list_d"), Some(&NbtList::Long(vec![1 >> 32, 2 >> 32, 3 >> 32])));
    assert_eq!(root.get_list("list_e"), Some(&NbtList::Float(vec![1.0, 2.0, 3.0])));
    assert_eq!(root.get_list("list_f"), Some(&NbtList::Double(vec![1.0, 2.0, 3.0])));
    assert_eq!(root.get_list("list_g"), Some(&NbtList::String(vec!["1".to_owned(), "2".to_owned(), "3".to_owned()])));
}
