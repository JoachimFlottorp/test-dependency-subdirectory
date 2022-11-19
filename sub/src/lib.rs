pub mod shirts {
    include!(concat!(env!("OUT_DIR"), "/shirts.rs"));
}

use shirts::{Shirt, shirt::Size};

pub fn cool_function(size: Size) -> Shirt {
    let mut shirt = Shirt::default();
    shirt.colour = "blue".to_string();
    shirt.set_size(size);
    shirt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cool_function() {
        let shirt = cool_function(Size::Large);
        assert_eq!(shirt.colour, "blue");
        assert_eq!(shirt.size, Size::Large.into());
    }
}