use axns::def_resource;

struct Data {
    a: i32,
    b: u32,
}

def_resource! {
    /// A resource for testing
    static TEST_RESOURCE: Data = Data { a: 0, b: 0 };
}

#[test]
fn qwq() {}
