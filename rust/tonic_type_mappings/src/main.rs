pub mod helloworld;

use helloworld::MessageA as ProtoMessageA;
use helloworld::MessageB as ProtoMessageB;

use proto_map::*;

trait IntoProtobuf {
    fn internal_print_into_protobuf_debug_info();
}

#[derive(IntoProtobuf, Debug)]
struct MessageA {
    name: String,
    value: i32,
}

#[derive(IntoProtobuf, Debug)]
struct MessageB {
    name: Option<String>,
    value: f32,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let rust_a = MessageA {
        name: "RustA".to_string(),
        value: 0x1337,
    };

    let msga_proto_from_macro: ProtoMessageA = rust_a.into();
    let msga_rust_from_proto: MessageA = msga_proto_from_macro.clone().into();
    dbg!(&msga_proto_from_macro);
    dbg!(&msga_rust_from_proto);
    print_type_of(&msga_proto_from_macro);
    print_type_of(&msga_rust_from_proto);
}
