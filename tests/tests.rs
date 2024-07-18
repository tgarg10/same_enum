use same_enum::{from, to};

#[from(TargetEnum)]
#[derive(PartialEq, Debug)]
enum SourceEnumWithFrom {
    One,
    Two,
}

#[to(TargetEnum)]
#[derive(PartialEq, Debug)]
enum SourceEnumWithTo {
    One,
    Two,
}

#[derive(PartialEq, Debug)]
enum TargetEnum {
    One,
    Two,
}

mod tests {
    use super::*;

    #[test]
    fn test_source_enum_from_target_enum() {
        // Arrange
        let source_one = SourceEnumWithTo::One;
        let source_two = SourceEnumWithTo::Two;

        // Act
        let target_one = TargetEnum::from(source_one);
        let target_two: TargetEnum = source_two.into();

        // Assert
        assert_eq!(target_one, TargetEnum::One);
        assert_eq!(target_two, TargetEnum::Two);
    }

    #[test]
    fn test_source_enum_to_target_enum() {
        // Arrange
        let target_one = TargetEnum::One;
        let target_two = TargetEnum::Two;

        // Act
        let source_one = SourceEnumWithFrom::from(target_one);
        let source_two: SourceEnumWithFrom = target_two.into();

        // Assert
        assert_eq!(source_one, SourceEnumWithFrom::One);
        assert_eq!(source_two, SourceEnumWithFrom::Two);
    }
}
