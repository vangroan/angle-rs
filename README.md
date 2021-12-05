
# angle-rs

Simple new types for working with angles in degrees and radians.

Angle values can be wrapped in `Deg<N>` or `Rad<N>` to make it
explicit what a function expects.

```rust
type Vector = [f64; 2];

/// Rotate vector counterclockwise by the given angle
fn rotate(a: Vector, angle: Deg<f64>) -> Vector {
    let r = angle.to_radians();
    let x = a[0] * f64::cos(r) - a[1] * f64::sin(r);
    let y = a[0] * f64::sin(r) + a[1] * f64::cos(r);
    [x, y]
}

let rotated = rotate([1., 0.], Deg(90.));
```

A function can take `Into<Deg<N>>` or `<Into<Rad<N>>>` to accept
angles as either floats or wrapped values.

```rust
fn rotate<T>(a: Vector, angle: T) -> Vector
where
    T: Into<Deg<f64>>,
{
    let r = angle.into().to_radians();
    let x = a[0] * f64::cos(r) - a[1] * f64::sin(r);
    let y = a[0] * f64::sin(r) + a[1] * f64::cos(r);
    [x, y]
}

rotate([1., 0.], 90.);
rotate([1., 0.], Deg(90.));
rotate([1., 0.], Rad(std::f64::consts::PI / 2.));
```
