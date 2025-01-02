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
