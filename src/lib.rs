pub mod r4 {
    pub mod elements {
        pub mod complex_types;
        pub mod element;
        pub mod reference;
    }
    pub mod resources {
        pub mod organization;
        pub mod resource;
    }
}

pub use r4::*;
