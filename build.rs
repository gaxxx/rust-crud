use protobuf_codegen_pure::Customize;

fn main() {
    println!("gen proto code");
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/protos")
        .inputs(&[
            "protos/hero.proto"
        ])
        .customize(Customize {
            expose_fields : Some(true),
            generate_accessors : Some(false),
            serde_derive: Some(true),
            ..Default::default()
        })
        .include("protos")
        .run()
        .expect("Codegen failed.");
}
