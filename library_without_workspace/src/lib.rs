mod greeting {
    pub mod hi {
        pub fn get() -> &'static str {
            "hi"
        }
    }

    pub mod bye {
        pub fn get() -> &'static str {
            "bye"
        }
    }
}

use greeting::{bye, hi};

pub fn get_hi() -> &'static str {
    hi::get()
}

pub fn get_bye() -> &'static str {
    bye::get()
}
