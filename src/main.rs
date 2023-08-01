mod lections {
    pub mod common_concepts;
    pub mod hello_world;
}

fn main() {
    crate::lections::hello_world::hello_world();
    crate::lections::common_concepts::run();
}
