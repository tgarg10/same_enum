# same_enum

Generates `From` trait implementations for enums with the same unit variants.

# Get Started

Place a `#[from(path::to::TargetEnum)]` or `#[to(path::to::TargetEnum)]`
attribute above the source enum definition to generate `From` trait implementations.

## `#[from(path::to::TargetEnum)]`

Generates an implementation of the From trait to convert from a target enum to the annotated enum.

### Example

```
use same_enum::from;
#[from(TargetEnum)]
#[derive(PartialEq, Debug)]
enum SourceEnum {
    One,
    Two,
}
#[derive(PartialEq, Debug)]
enum TargetEnum {
    One,
    Two,
}

let target_one = TargetEnum::One;
let source_one: SourceEnum = target_one.into(); // from implementation gives us into() as well

assert_eq!(source_one, SourceEnum::One);
```

## `#[to(path::to::TargetEnum)]`

Generates an implementation of the From trait to convert from the annotated enum to a target enum.

### Example

```
use same_enum::to;
#[to(TargetEnum)]
#[derive(PartialEq, Debug)]
enum SourceEnum {
    One,
    Two,
}
#[derive(PartialEq, Debug)]
enum TargetEnum {
    One,
    Two,
}

let source_one = SourceEnum::One;
let target_one = TargetEnum::from(source_one);

assert_eq!(target_one, TargetEnum::One);
```

## Panics

`same_enum` panics if the annotated_item is not an enum with Unit variants.

An example of an enum **with** Unit variants would be:

```
enum BinaryNumbers {
    Zero,
    One,
}
```
