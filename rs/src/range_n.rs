use std::ops::{Add, Div, Mul, Range, Sub};

use num::{One, Zero};

/// Cartesian product of two ranges.
///
/// Equivalent of an axis-aligned bounding rectangle.
///
/// ```
/// use aoc::Range2;
///
/// let r2 = Range2::new(-1..2, 0..2);
///
/// assert_eq!(
///     r2.into_iter().collect::<Vec<(i32, usize)>>(),
///     vec![(-1, 0), (0, 0), (1, 0),
///          (-1, 1), (0, 1), (1, 1)]);
/// ```
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Range2<X, Y> {
    x1: X,
    x2: X,
    y1: Y,
    y2: Y,
}

pub fn range2<X, Y>(x: Range<X>, y: Range<Y>) -> Range2<X, Y>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
{
    Range2::new(x, y)
}

pub fn area<X, Y>(dim: impl Into<(X, Y)>) -> Range2<X, Y>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
{
    let (w, h) = dim.into();
    Range2::sized(w, h)
}

pub fn rect<X, Y>(p1: impl Into<(X, Y)>, p2: impl Into<(X, Y)>) -> Range2<X, Y>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
{
    let (x1, y1) = p1.into();
    let (x2, y2) = p2.into();
    Range2::new(x1..x2, y1..y2)
}

impl<X, Y> Range2<X, Y>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
{
    pub fn new(x: Range<X>, y: Range<Y>) -> Self {
        Range2 {
            x1: x.start,
            x2: x.end,
            y1: y.start,
            y2: y.end,
        }
    }

    pub fn sized(x: X, y: Y) -> Self {
        Range2::new(Zero::zero()..x, Zero::zero()..y)
    }

    pub fn contains(&self, e: impl Into<(X, Y)>) -> bool {
        let (x, y) = e.into();
        (self.x1..self.x2).contains(&x) && (self.y1..self.y2).contains(&y)
    }

    pub fn min<T: From<(X, Y)>>(&self) -> T {
        T::from((self.x1, self.y1))
    }

    pub fn max<T: From<(X, Y)>>(&self) -> T {
        T::from((self.x2, self.y2))
    }
}

impl<X, Y> Range2<X, Y>
where
    X: Copy + PartialOrd + Zero + One,
    Y: Copy + PartialOrd + Zero + One,
{
    pub fn from_points(i: impl IntoIterator<Item = impl Into<(X, Y)>>) -> Self {
        let mut i = i.into_iter();
        if let Some((x, y)) = i.next().map(|e| e.into()) {
            let (x1, x2, y1, y2) = i.map(|e| e.into()).fold(
                (x, x, y, y),
                |(x1, x2, y1, y2), (x, y)| {
                    (pmin(x1, x), pmax(x2, x), pmin(y1, y), pmax(y2, y))
                },
            );
            Range2::new(x1..x2, y1..y2)
        } else {
            Range2::sized(Zero::zero(), Zero::zero())
        }
    }

    pub fn from_points_inclusive(
        i: impl IntoIterator<Item = impl Into<(X, Y)>>,
    ) -> Self {
        let mut i = i.into_iter();
        if let Some((x, y)) = i.next().map(|e| e.into()) {
            let (x1, x2, y1, y2) = i.map(|e| e.into()).fold(
                (x, x + One::one(), y, y + One::one()),
                |(x1, x2, y1, y2), (x, y)| {
                    (
                        pmin(x1, x),
                        pmax(x2, x + One::one()),
                        pmin(y1, y),
                        pmax(y2, y + One::one()),
                    )
                },
            );
            Range2::new(x1..x2, y1..y2)
        } else {
            Range2::sized(Zero::zero(), Zero::zero())
        }
    }
}

impl<X, Y> Range2<X, Y>
where
    X: Copy + Add<Output = X> + Sub<Output = X>,
    Y: Copy + Add<Output = Y> + Sub<Output = Y>,
{
    pub fn size<T: From<(X, Y)>>(&self) -> T {
        T::from((self.x2 - self.x1, self.y2 - self.y1))
    }

    pub fn width(&self) -> X {
        self.x2 - self.x1
    }

    pub fn height(&self) -> Y {
        self.y2 - self.y1
    }

    /// Inflate by given amount along each dimension in both directions.
    pub fn inflate(&self, amount: impl Into<(X, Y)>) -> Self {
        let (x, y) = amount.into();
        Range2 {
            x1: self.x1 - x,
            x2: self.x2 + x,
            y1: self.y1 - y,
            y2: self.y2 + y,
        }
    }
}

impl<X> Range2<X, X>
where
    X: Copy + Add<Output = X> + Sub<Output = X> + Mul<Output = X>,
{
    pub fn area(&self) -> X {
        self.width() * self.height()
    }
}

impl<X, Y> Range2<X, Y>
where
    X: Copy + One + Add<Output = X> + Sub<Output = X> + Div<Output = X>,
    Y: Copy + One + Add<Output = Y> + Sub<Output = Y> + Div<Output = Y>,
{
    pub fn center<T: From<(X, Y)>>(&self) -> T {
        T::from((
            self.x1 + self.width() / (X::one() + X::one()),
            self.y1 + self.height() / (Y::one() + Y::one()),
        ))
    }
}

impl<X, Y> IntoIterator for Range2<X, Y>
where
    X: Copy + PartialOrd + One + Add<Output = X>,
    Y: Copy + PartialOrd + One + Add<Output = Y>,
{
    type Item = (X, Y);

    type IntoIter = Range2Iter<X, Y>;

    fn into_iter(self) -> Range2Iter<X, Y> {
        Range2Iter {
            inner: self,
            x: self.x1,
        }
    }
}

pub struct Range2Iter<X, Y> {
    inner: Range2<X, Y>,
    x: X,
}

impl<X, Y> Iterator for Range2Iter<X, Y>
where
    X: Copy + PartialOrd + One + Add<Output = X>,
    Y: Copy + PartialOrd + One + Add<Output = Y>,
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.inner.x2 {
            // End of scan, move to next line.
            self.x = self.inner.x1;
            self.inner.y1 = self.inner.y1 + Y::one();
            if self.inner.y1 >= self.inner.y2 || self.x >= self.inner.x2 {
                // Out of content.
                return None;
            }
        }
        let ret = Some((self.x, self.inner.y1));
        self.x = self.x + X::one();
        ret
    }
}

/// Cartesian product of three ranges.
///
/// Equivalent of an axis-aligned bounding box.
///
/// ```
/// use aoc::Range3;
///
/// let r3 = Range3::new(5..7, 0..2, 10..12);
/// assert_eq!(
///     r3.into_iter().collect::<Vec<(i32, usize, u32)>>(),
///     vec![(5, 0, 10), (6, 0, 10),
///          (5, 1, 10), (6, 1, 10),
///          (5, 0, 11), (6, 0, 11),
///          (5, 1, 11), (6, 1, 11)]);
/// ```
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub struct Range3<X, Y, Z> {
    x1: X,
    x2: X,
    y1: Y,
    y2: Y,
    z1: Z,
    z2: Z,
}

pub fn range3<X, Y, Z>(x: Range<X>, y: Range<Y>, z: Range<Z>) -> Range3<X, Y, Z>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
    Z: Copy + PartialOrd + Zero,
{
    Range3::new(x, y, z)
}

pub fn volume<X, Y, Z>(dim: impl Into<(X, Y, Z)>) -> Range3<X, Y, Z>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
    Z: Copy + PartialOrd + Zero,
{
    let (w, h, d) = dim.into();
    Range3::sized(w, h, d)
}

pub fn cube<X, Y, Z>(
    p1: impl Into<(X, Y, Z)>,
    p2: impl Into<(X, Y, Z)>,
) -> Range3<X, Y, Z>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
    Z: Copy + PartialOrd + Zero,
{
    let (x1, y1, z1) = p1.into();
    let (x2, y2, z2) = p2.into();
    Range3::new(x1..x2, y1..y2, z1..z2)
}

impl<X, Y, Z> Range3<X, Y, Z>
where
    X: Copy + PartialOrd + Zero,
    Y: Copy + PartialOrd + Zero,
    Z: Copy + PartialOrd + Zero,
{
    pub fn new(x: Range<X>, y: Range<Y>, z: Range<Z>) -> Self {
        Range3 {
            x1: x.start,
            x2: x.end,
            y1: y.start,
            y2: y.end,
            z1: z.start,
            z2: z.end,
        }
    }

    pub fn sized(x: X, y: Y, z: Z) -> Self {
        Range3::new(Zero::zero()..x, Zero::zero()..y, Zero::zero()..z)
    }

    pub fn contains(&self, e: impl Into<(X, Y, Z)>) -> bool {
        let (x, y, z) = e.into();
        (self.x1..self.x2).contains(&x)
            && (self.y1..self.y2).contains(&y)
            && (self.z1..self.z2).contains(&z)
    }

    pub fn min<T: From<(X, Y, Z)>>(&self) -> T {
        T::from((self.x1, self.y1, self.z1))
    }

    pub fn max<T: From<(X, Y, Z)>>(&self) -> T {
        T::from((self.x2, self.y2, self.z2))
    }
}

impl<X, Y, Z> Range3<X, Y, Z>
where
    X: Copy + PartialOrd + Zero + One,
    Y: Copy + PartialOrd + Zero + One,
    Z: Copy + PartialOrd + Zero + One,
{
    pub fn from_points(
        i: impl IntoIterator<Item = impl Into<(X, Y, Z)>>,
    ) -> Self {
        let mut i = i.into_iter();
        if let Some((x, y, z)) = i.next().map(|e| e.into()) {
            let (x1, x2, y1, y2, z1, z2) = i.map(|e| e.into()).fold(
                (x, x, y, y, z, z),
                |(x1, x2, y1, y2, z1, z2), (x, y, z)| {
                    (
                        pmin(x1, x),
                        pmax(x2, x),
                        pmin(y1, y),
                        pmax(y2, y),
                        pmin(z1, z),
                        pmax(z2, z),
                    )
                },
            );
            Range3::new(x1..x2, y1..y2, z1..z2)
        } else {
            Range3::sized(Zero::zero(), Zero::zero(), Zero::zero())
        }
    }

    pub fn from_points_inclusive(
        i: impl IntoIterator<Item = impl Into<(X, Y, Z)>>,
    ) -> Self {
        let mut i = i.into_iter();
        if let Some((x, y, z)) = i.next().map(|e| e.into()) {
            let (x1, x2, y1, y2, z1, z2) = i.map(|e| e.into()).fold(
                (x, x + One::one(), y, y + One::one(), z, z + One::one()),
                |(x1, x2, y1, y2, z1, z2), (x, y, z)| {
                    (
                        pmin(x1, x),
                        pmax(x2, x + One::one()),
                        pmin(y1, y),
                        pmax(y2, y + One::one()),
                        pmin(z1, z),
                        pmax(z2, z + One::one()),
                    )
                },
            );
            Range3::new(x1..x2, y1..y2, z1..z2)
        } else {
            Range3::sized(Zero::zero(), Zero::zero(), Zero::zero())
        }
    }
}

impl<X, Y, Z> Range3<X, Y, Z>
where
    X: Copy + Add<Output = X> + Sub<Output = X>,
    Y: Copy + Add<Output = Y> + Sub<Output = Y>,
    Z: Copy + Add<Output = Z> + Sub<Output = Z>,
{
    pub fn size<T: From<(X, Y, Z)>>(&self) -> T {
        T::from((self.x2 - self.x1, self.y2 - self.y1, self.z2 - self.z1))
    }

    pub fn width(&self) -> X {
        self.x2 - self.x1
    }

    pub fn height(&self) -> Y {
        self.y2 - self.y1
    }

    pub fn depth(&self) -> Z {
        self.z2 - self.z1
    }

    /// Inflate by given amount along each dimension in both directions.
    pub fn inflate(&self, amount: impl Into<(X, Y, Z)>) -> Self {
        let (x, y, z) = amount.into();
        Range3 {
            x1: self.x1 - x,
            x2: self.x2 + x,
            y1: self.y1 - y,
            y2: self.y2 + y,
            z1: self.z1 - z,
            z2: self.z2 + z,
        }
    }
}

impl<X> Range3<X, X, X>
where
    X: Copy + Add<Output = X> + Sub<Output = X> + Mul<Output = X>,
{
    pub fn volume(&self) -> X {
        self.width() * self.height() * self.depth()
    }
}

impl<X, Y, Z> Range3<X, Y, Z>
where
    X: Copy + One + Add<Output = X> + Sub<Output = X> + Div<Output = X>,
    Y: Copy + One + Add<Output = Y> + Sub<Output = Y> + Div<Output = Y>,
    Z: Copy + One + Add<Output = Z> + Sub<Output = Z> + Div<Output = Z>,
{
    pub fn center<T: From<(X, Y, Z)>>(&self) -> T {
        T::from((
            self.x1 + self.width() / (X::one() + X::one()),
            self.y1 + self.height() / (Y::one() + Y::one()),
            self.z1 + self.depth() / (Z::one() + Z::one()),
        ))
    }
}

impl<X, Y, Z> IntoIterator for Range3<X, Y, Z>
where
    X: Copy + PartialOrd + One + Add<Output = X>,
    Y: Copy + PartialOrd + One + Add<Output = Y>,
    Z: Copy + PartialOrd + One + Add<Output = Z>,
{
    type Item = (X, Y, Z);

    type IntoIter = Range3Iter<X, Y, Z>;

    fn into_iter(self) -> Range3Iter<X, Y, Z> {
        Range3Iter {
            inner: self,
            x: self.x1,
            y: self.y1,
        }
    }
}

pub struct Range3Iter<X, Y, Z> {
    inner: Range3<X, Y, Z>,
    x: X,
    y: Y,
}

impl<X, Y, Z> Iterator for Range3Iter<X, Y, Z>
where
    X: Copy + PartialOrd + One + Add<Output = X>,
    Y: Copy + PartialOrd + One + Add<Output = Y>,
    Z: Copy + PartialOrd + One + Add<Output = Z>,
{
    type Item = (X, Y, Z);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.inner.x2 {
            // End of scan, move to next line.
            self.x = self.inner.x1;
            self.y = self.y + Y::one();

            if self.y >= self.inner.y2 {
                // End of sweep, move to next plane.
                self.y = self.inner.y1;
                self.inner.z1 = self.inner.z1 + Z::one();
            }

            if self.inner.z1 >= self.inner.z2
                || self.y >= self.inner.y2
                || self.x >= self.inner.x2
            {
                // Out of content.
                return None;
            }
        }
        let ret = Some((self.x, self.y, self.inner.z1));
        self.x = self.x + X::one();
        ret
    }
}

/// Return the larger of the two numbers. If the numbers can't be ordered, try
/// to return the number that can be ordered with itself.
pub fn pmin<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else if b.partial_cmp(&b).is_some() {
        b
    } else {
        a
    }
}

/// Return the smaller of the two numbers. If the numbers can't be ordered,
/// try to return the number that can be ordered with itself.
pub fn pmax<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else if b.partial_cmp(&b).is_some() {
        b
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pmin_pmax() {
        assert_eq!(pmax(1.0, 2.0), 2.0);
        assert_eq!(pmax(f32::NAN, 2.0), 2.0);
        assert_eq!(pmax(1.0, f32::NAN), 1.0);
        assert!(pmax(f32::NAN, f32::NAN).is_nan());

        assert_eq!(pmin(1.0, 2.0), 1.0);
        assert_eq!(pmin(f32::NAN, 2.0), 2.0);
        assert_eq!(pmin(1.0, f32::NAN), 1.0);
        assert!(pmin(f32::NAN, f32::NAN).is_nan());
    }
}
