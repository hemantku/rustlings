// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.


struct ColorClassicStruct {
    green: i32,
    red: i32,
    blue: i32,
}

impl ColorClassicStruct {
    fn new() -> ColorClassicStruct {
        ColorClassicStruct{
            green: 255,
            red: 0,
            blue: 0,
        }
    }
}
struct ColorTupleStruct(i32, i32, i32);

impl ColorTupleStruct {
    fn new() -> ColorTupleStruct {
        ColorTupleStruct(0, 255, 0)
    }
}

#[derive(Debug)]
struct UnitLikeStruct;
impl UnitLikeStruct {
    fn new() -> UnitLikeStruct {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct::new();
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct::new();

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct::new();
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
