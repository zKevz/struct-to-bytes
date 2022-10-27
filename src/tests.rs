#[cfg(test)]
mod tests {
    use crate::{de, ser, FormatOptions};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct Employee {
        name: String,
    }

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct Company {
        name: String,
        employees: Vec<Employee>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    enum Job {
        Student(String),
        Police { name: String, rank: String },
        Doctor,
    }

    #[test]
    pub fn struct_test() {
        let company = Company {
            name: String::from("Walter White Company"),
            employees: vec![Employee {
                name: String::from("Jesse Pinkman"),
            }],
        };

        let bytes = ser::to_bytes(&company).unwrap();
        assert_eq!(
            bytes,
            [
                0x14, 0x00, 0x57, 0x61, 0x6C, 0x74, 0x65, 0x72, 0x20, 0x57, 0x68, 0x69, 0x74, 0x65,
                0x20, 0x43, 0x6F, 0x6D, 0x70, 0x61, 0x6E, 0x79, 0x01, 0x00, 0x0D, 0x00, 0x4A, 0x65,
                0x73, 0x73, 0x65, 0x20, 0x50, 0x69, 0x6E, 0x6B, 0x6D, 0x61, 0x6E
            ]
        );

        let expected = de::from_bytes(&bytes).unwrap();
        assert_eq!(company, expected);
    }

    #[test]
    pub fn struct_with_option_test() {
        let company = Company {
            name: String::from("Walter White Company"),
            employees: vec![Employee {
                name: String::from("Jesse Pinkman"),
            }],
        };

        let options = FormatOptions {
            short_length: false,
            short_variant_index: false,
        };
        let bytes = ser::to_bytes_with_options(&company, options).unwrap();
        assert_eq!(
            bytes,
            [
                0x14, 0x00, 0x00, 0x00, 0x57, 0x61, 0x6C, 0x74, 0x65, 0x72, 0x20, 0x57, 0x68, 0x69,
                0x74, 0x65, 0x20, 0x43, 0x6F, 0x6D, 0x70, 0x61, 0x6E, 0x79, 0x01, 0x00, 0x00, 0x00,
                0x0D, 0x00, 0x00, 0x00, 0x4A, 0x65, 0x73, 0x73, 0x65, 0x20, 0x50, 0x69, 0x6E, 0x6B,
                0x6D, 0x61, 0x6E
            ]
        );

        let expected = de::from_bytes_with_options(&bytes, options).unwrap();
        assert_eq!(company, expected);
    }

    #[test]
    pub fn enum_test() {
        let jobs = vec![
            Job::Student(String::from("Rickrolled")),
            Job::Doctor,
            Job::Police {
                name: String::from("Hank Schrader"),
                rank: String::from("Newbie"),
            },
            Job::Doctor,
        ];
        let bytes = ser::to_bytes(&jobs).unwrap();
        assert_eq!(
            bytes,
            [
                0x04, 0x00, 0x00, 0x0A, 0x00, 0x52, 0x69, 0x63, 0x6B, 0x72, 0x6F, 0x6C, 0x6C, 0x65,
                0x64, 0x02, 0x01, 0x0D, 0x00, 0x48, 0x61, 0x6E, 0x6B, 0x20, 0x53, 0x63, 0x68, 0x72,
                0x61, 0x64, 0x65, 0x72, 0x06, 0x00, 0x4E, 0x65, 0x77, 0x62, 0x69, 0x65, 0x02
            ]
        );

        let expected = de::from_bytes::<Vec<Job>>(&bytes).unwrap();
        assert_eq!(jobs, expected);
    }

    #[test]
    pub fn hashmap_test() {
        let map = HashMap::from([(10, String::from("Hey!")), (69, String::from("Hola!"))]);
        let bytes = ser::to_bytes(&map).unwrap();
        let expected = de::from_bytes(&bytes).unwrap();
        assert_eq!(map, expected);
    }
}
