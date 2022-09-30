pub use paste::paste;

#[macro_export]
macro_rules! struct_with_fields {
    (
        struct $name:ident {
            $( $field_name:ident: $type:ty, )*
        }
    ) => {
        $crate::paste!{
            struct $name<'a> {
                $( $field_name: $type, )*
            }

            #[allow(non_camel_case_types)]
            enum [<$name FieldId>] { $($field_name,)* }
        }
    };
}

pub fn hello() {
    println!("hello {}", base64::encode(b"world"));
}
