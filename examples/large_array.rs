const NUM_ELEMENT: usize = 210000;

mod first {

    #[derive(Clone, Default, Copy)]
    pub struct Element {
        pub v1: f32,
    }

    #[derive(Clone)]
    pub struct Foo {
        pub data: [Element; super::NUM_ELEMENT],
    }
}

mod second {
    use struct_auto_from::auto_from_ns;

    #[derive(Clone, Copy, Default)]
    #[auto_from_ns(super::first)]
    pub struct Element {
        pub v1: f32,
    }

    #[derive(Clone)]
    #[auto_from_ns(super::first)]
    pub struct Foo {
        pub data: [Element; super::NUM_ELEMENT],
    }
}

fn main() {
    let foo = first::Foo {
        data: [first::Element { v1: 1.0 }; NUM_ELEMENT],
    };
    let bar: second::Foo = foo.into();
    assert_eq!(1.0, bar.data[0].v1);

    println!("Done!")
}
