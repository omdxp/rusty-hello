struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {}

fn give_two() -> i32 {
    2
}

#[cfg(test)]
mod fish_tests {
    #[test]
    #[should_panic]
    fn it_works() {
        assert_eq!(2 + super::give_two(), 4);
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]
    fn it_works_2() {
        assert_eq!(super::give_two() + 2, 5);
        assert_ne!(2 + super::give_two(), 3);
    }

    #[test]
    #[should_panic]
    fn test_structs() {
        let r = super::Rectangle {
            width: 20,
            height: 10,
        };
        assert!(r.is_square());
    }
}
