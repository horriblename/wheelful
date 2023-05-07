pub mod geometry {
    #[derive(Debug)]
    pub struct Coord {
        pub x: f64,
        pub y: f64,
    }

    impl Coord {
        pub fn from_tuple(coord: (f64, f64)) -> Self {
            Coord {
                x: coord.0,
                y: coord.1,
            }
        }

        pub fn gradient(&self, destination: &Coord) -> f64 {
            (destination.y - self.y) / (destination.x - self.x)
        }

        /// Starts from the right and rotates clockwise.
        /// Return value is in the range [-pi, pi).
        pub fn rotation_angle(&self, destination: &Coord) -> f64 {
            use std::f64::consts::PI;

            let dx = destination.x - self.x;
            let atan = f64::atan(self.gradient(destination));
            if atan >= 0.0 && dx < 0.0 {
                return atan - PI;
            }
            if atan < 0.0 && dx < 0.0 {
                return atan + PI;
            }
            atan
        }

        pub fn distance(&self, destination: &Coord) -> f64 {
            let dx = destination.x - self.x;
            let dy = destination.y - self.y;
            f64::sqrt(dx * dx + dy * dy)
        }
    }

    #[cfg(test)]
    mod test {
        use super::Coord;
        use std::f64::consts::PI;

        struct Test {
            origin: Coord,
            destination: Coord,
            result: f64,
        }

        #[test]
        fn test_rotation_angle() {
            let tests: Vec<Test> = vec![
                Test {
                    origin: Coord { x: 0.0, y: 0.0 },
                    destination: Coord { x: 1.0, y: 0.0 },
                    result: 0.0,
                },
                Test {
                    origin: Coord { x: 0.0, y: 0.0 },
                    destination: Coord { x: 0.0, y: 1.0 },
                    result: PI / 2.0,
                },
                Test {
                    origin: Coord { x: 0.0, y: 0.0 },
                    destination: Coord { x: -1.0, y: 0.0 },
                    result: -PI,
                },
                Test {
                    origin: Coord { x: 0.0, y: 0.0 },
                    destination: Coord { x: 0.0, y: -1.0 },
                    result: -PI / 2.0,
                },
            ];

            for test in &tests {
                assert_eq!(
                    Coord::rotation_angle(&test.origin, &test.destination),
                    test.result
                )
            }
        }
    }
}
