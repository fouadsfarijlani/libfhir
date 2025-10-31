pub mod r4 {
    pub mod elements {
        pub mod complex_types;
        pub use complex_types::*;

        pub mod element;
        pub use element::*;

        pub mod reference;
        pub use reference::*;
    }
    pub mod resources {
        pub mod endpoint;
        pub use endpoint::*;

        pub mod organization;
        pub use organization::*;

        pub mod resource;
        pub use resource::*;
    }
}

pub use r4::*;
