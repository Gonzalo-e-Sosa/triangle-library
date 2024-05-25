pub struct Triangle {
    side1: u32,
    side2: u32,
    side3: u32,
}

impl Triangle {
    pub fn new(side1: u32, side2: u32, side3: u32) -> Self {
        if side1 == 0 || side2 == 0 || side3 == 0 {
            panic!("Can not create instance with a side length of zero");
        }
        if !Self::is_triangle(side1, side2, side3) {
            panic!("Is nou32 a u32riangle");
        }
        Triangle {
            side1,
            side2,
            side3,
        }
    }

    fn is_triangle(side1: u32, side2: u32, side3: u32) -> bool {
        side1 < side2 + side3 && side2 < side1 + side3 && side3 < side1 + side2
    }

    pub fn get_greater_side(&self) -> u32 {
        self.side1.max(self.side2).max(self.side3)
    }

    pub fn get_triangle_type(&self) -> String {
        if self.side1 == self.side2 && self.side2 == self.side3 {
            String::from("Equilateral")
        } else if self.side1 != self.side2 && self.side2 != self.side3 && self.side1 != self.side3 {
            String::from("Scalene")
        } else {
            String::from("Isosceles")
        }
    }
}

impl std::fmt::Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Triangle {{ side1: {}, side2: {}, side3: {} }}",
            self.side1, self.side2, self.side3
        )
    }
}

// test lib
#[cfg(test)]
mod tests {
    use super::*;
    use Triangle;

    #[test]
    fn it_works() {
        let t = Triangle::new(2, 2, 3);
        assert_eq!(Triangle::get_greater_side(&t), 3);
        assert_eq!(Triangle::get_triangle_type(&t), "Isosceles");

        let equilateral = Triangle::new(3, 3, 3);
        assert_eq!(Triangle::get_triangle_type(&equilateral), "Equilateral");

        let scalene = Triangle::new(3, 4, 5);
        assert_eq!(Triangle::get_triangle_type(&scalene), "Scalene");

        // test for panic when creating a triangle with a side length of zero
        let result = std::panic::catch_unwind(|| Triangle::new(0, 4, 5));
        assert!(result.is_err());

        // test for panic when creating a non-triangle
        let result = std::panic::catch_unwind(|| Triangle::new(1, 2, 3));
        assert!(result.is_err());
    }
}
