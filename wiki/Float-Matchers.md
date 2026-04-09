# Float Matchers

Float matchers provide approximate equality and special value classification for `f32` and `f64` types.

## to_be_close_to

Checks if a floating point number is within a tolerance of an expected value. Uses absolute difference: `(actual - expected).abs() <= tolerance`.

```rust
fn test_close_to() {
    let result = 0.1 + 0.2;

    expect!(result).to_be_close_to(0.3, 0.001);                           // Passes
    expect!(std::f64::consts::PI).to_be_close_to(3.14159, 0.00001);       // Passes
    expect!(result).not().to_be_close_to(0.5, 0.001);                     // Passes
}
```

Also works with `f32`:

```rust
fn test_close_to_f32() {
    let result: f32 = 0.1_f32 + 0.2_f32;

    expect!(result).to_be_close_to(0.3_f32, 0.001_f32);  // Passes
}
```

## to_be_nan

Checks if a floating point number is NaN (Not a Number).

```rust
fn test_nan() {
    expect!(f64::NAN).to_be_nan();             // Passes
    expect!(f32::NAN).to_be_nan();             // Passes
    expect!(1.0_f64).not().to_be_nan();        // Passes
}
```

## to_be_infinite

Checks if a floating point number is infinite (positive or negative infinity).

```rust
fn test_infinite() {
    expect!(f64::INFINITY).to_be_infinite();       // Passes
    expect!(f64::NEG_INFINITY).to_be_infinite();   // Passes
    expect!(1.0_f64).not().to_be_infinite();       // Passes
}
```

## to_be_finite

Checks if a floating point number is finite (not NaN and not infinite).

```rust
fn test_finite() {
    expect!(1.0_f64).to_be_finite();               // Passes
    expect!(0.0_f32).to_be_finite();               // Passes
    expect!(-42.5_f64).to_be_finite();             // Passes
    expect!(f64::NAN).not().to_be_finite();        // Passes
    expect!(f64::INFINITY).not().to_be_finite();   // Passes
}
```

## Chaining

Float matchers work with `.and()`, `.or()`, and `.not()` modifiers:

```rust
fn test_chaining() {
    expect!(3.14_f64).to_be_close_to(std::f64::consts::PI, 0.01)
                     .and().to_be_positive();

    expect!(0.1_f64 + 0.2_f64).to_be_close_to(0.3, 0.001)
                                .and().to_be_finite();
}
```

## Edge Cases

- NaN is never close to anything, including itself: `expect!(f64::NAN).not().to_be_close_to(f64::NAN, 1.0)` passes
- Negative zero is close to positive zero: `expect!(-0.0_f64).to_be_close_to(0.0, 0.0)` passes
- Infinity is not close to any finite value, regardless of tolerance
