use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use num::{traits::Euclid, One, Zero};

/// Cartesian product of several ranges.
///
/// Equivalent to an axis-aligned bounding rectangle, bounding box etc.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NRange<X, const N: usize> {
    p0: [X; N],
    p1: [X; N],
}

pub fn area<T>(w: T, h: T) -> NRange<T, 2>
where
    T: Copy + PartialOrd + Zero,
{
    NRange::sized([w, h])
}

pub fn volume<T, const N: usize>(p: impl Into<[T; N]>) -> NRange<T, N>
where
    T: Copy + PartialOrd + Zero,
{
    NRange::sized(p)
}

impl<T, const N: usize> NRange<T, N> {
    /// Faster than `NRange::new`, but does not check that dimensions are
    /// positive.
    ///
    /// # Safety
    ///
    /// Caller must ensure `p1[i] >= p0[i]` for all i.
    pub unsafe fn new_unsafe(
        p0: impl Into<[T; N]>,
        p1: impl Into<[T; N]>,
    ) -> NRange<T, N> {
        NRange {
            p0: p0.into(),
            p1: p1.into(),
        }
    }
}

impl<T, const N: usize> Default for NRange<T, N>
where
    T: Copy + Default,
{
    fn default() -> Self {
        NRange {
            p0: [T::default(); N],
            p1: [T::default(); N],
        }
    }
}

impl<T, const N: usize> NRange<T, N>
where
    T: Copy + PartialOrd + Zero,
{
    /// Create a new n-range. If p1 has components that are smaller than p0's,
    /// the components are swapped with those of p0 before going in the range.
    pub fn new(p0: impl Into<[T; N]>, p1: impl Into<[T; N]>) -> NRange<T, N> {
        let (mut p0, mut p1) = (p0.into(), p1.into());

        // Make sure the points are ordered.
        for i in 0..N {
            let (a, b) = (pmin(p0[i], p1[i]), pmax(p0[i], p1[i]));
            p0[i] = a;
            p1[i] = b;
        }

        NRange { p0, p1 }
    }

    pub fn sized(p: impl Into<[T; N]>) -> NRange<T, N> {
        NRange::new([T::zero(); N], p)
    }
}

impl<T, const N: usize> NRange<T, N>
where
    T: Copy
        + Default
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Zero
        + One,
{
    /// Builds a n-range from the elementwise minimum and maximum of the
    /// points in the input point cloud.
    ///
    /// NB. The resulting range does not contain the outer rim of the points
    /// since ranges are exclusive on the outer end.
    pub fn from_points(
        it: impl IntoIterator<Item = impl Into<[T; N]>>,
    ) -> NRange<T, N> {
        let mut it = it.into_iter();
        if let Some(p) = it.next().map(|e| e.into()) {
            let (p0, p1) =
                it.map(|e| e.into()).fold((p, p), |(mut p0, mut p1), p| {
                    for i in 0..N {
                        p0[i] = pmin(p0[i], p[i]);
                        p1[i] = pmax(p1[i], p[i]);
                    }
                    (p0, p1)
                });
            NRange { p0, p1 }
        } else {
            Default::default()
        }
    }

    /// Builds a n-range guaranteed to contain every point in the point cloud.
    /// For integer `T` the result is the smallest such range.
    pub fn from_points_inclusive(
        it: impl IntoIterator<Item = impl Into<[T; N]>>,
    ) -> NRange<T, N> {
        let mut it = it.into_iter();
        if let Some(p0) = it.next().map(|e| e.into()) {
            let mut p1 = p0;
            for e in p1.iter_mut() {
                *e = *e + T::one();
            }

            let (p0, p1) =
                it.map(|e| e.into()).fold((p0, p1), |(mut p0, mut p1), p| {
                    for i in 0..N {
                        p0[i] = pmin(p0[i], p[i]);
                        p1[i] = pmax(p1[i], p[i] + T::one());
                    }
                    (p0, p1)
                });
            NRange { p0, p1 }
        } else {
            Default::default()
        }
    }

    pub fn contains(&self, e: impl Into<[T; N]>) -> bool {
        let e = e.into();
        (0..N).all(move |i| (self.p0[i]..self.p1[i]).contains(&e[i]))
    }

    pub fn volume(&self) -> T {
        (0..N)
            .map(move |i| self.p1[i] - self.p0[i])
            .fold(T::one(), |a, b| a * b)
    }

    /// Return vector with dimensions of the range.
    pub fn dim<E: From<[T; N]>>(&self) -> E {
        let mut ret = self.p1;
        for i in 0..N {
            ret[i] = ret[i] - self.p0[i];
        }
        E::from(ret)
    }

    pub fn min<E: From<[T; N]>>(&self) -> E {
        E::from(self.p0)
    }

    pub fn max<E: From<[T; N]>>(&self) -> E {
        E::from(self.p1)
    }

    pub fn width(&self) -> T {
        self.p1[0] - self.p0[0]
    }

    pub fn height(&self) -> T {
        debug_assert!(N >= 2);
        self.p1[1] - self.p0[1]
    }

    pub fn depth(&self) -> T {
        debug_assert!(N >= 3);
        self.p1[2] - self.p0[2]
    }

    pub fn inflate(&self, amount: impl Into<[T; N]>) -> Self {
        let amount = amount.into();
        let (mut p0, mut p1) = (self.p0, self.p1);
        for i in 0..N {
            p0[i] = p0[i] - amount[i];
            p1[i] = p1[i] + amount[i];
        }

        NRange::new(p0, p1)
    }
}

impl<T, const N: usize> NRange<T, N>
where
    T: Copy + Euclid + Sub<Output = T> + Add<Output = T>,
{
    /// Projects a point into the inside of the range using modular arithmetic
    /// on each axis. A point leaving across one end will return on the other
    /// end.
    pub fn mod_proj<E>(&self, p: E) -> E
    where
        E: From<[T; N]> + Into<[T; N]>,
    {
        let mut p = p.into();
        for i in 0..N {
            p[i] = p[i] - self.p0[i];
            p[i] = p[i].rem_euclid(&(self.p1[i] - self.p0[i]));
            p[i] = p[i] + self.p0[i];
        }
        E::from(p)
    }
}

impl<T, const N: usize> NRange<T, N>
where
    T: Copy
        + Default
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Zero
        + One
        + Euclid
        + TryInto<usize>
        + TryFrom<usize>,
{
    pub fn index_of(&self, p: impl Into<[T; N]>) -> usize {
        let p = p.into();

        let size: [T; N] = self.dim();
        let mut span = [0; N];
        for i in 0..N {
            let Ok(x) = size[i].try_into() else {
                panic!("bad range");
            };
            span[i] = x;
        }

        let mut ret = 0;
        let mut scale = 1;
        for i in 0..N {
            let Ok(x) = (p[i] - self.p0[i]).rem_euclid(&size[i]).try_into() else {
                panic!("bad range");
            };
            ret += x * scale;
            scale *= span[i];
        }

        ret
    }

    pub fn get(&self, n: usize) -> [T; N] {
        let size: [T; N] = self.dim();
        let mut span = [0; N];
        for i in 0..N {
            let Ok(x) = size[i].try_into() else {
                panic!("bad range");
            };
            span[i] = x;
        }

        let mut v = [0; N];
        let mut scale = 1;
        for i in 0..N {
            v[i] = (n / scale) % span[i];
            scale *= span[i];
        }

        let mut e = [Default::default(); N];
        for i in 0..N {
            let Ok(x) = T::try_from(v[i]) else {
                panic!("bad range");
            };
            e[i] = self.p0[i] + x;
        }
        e
    }
}

impl<T, const N: usize> IntoIterator for NRange<T, N>
where
    T: Copy + PartialOrd + One + Add<Output = T>,
{
    type Item = [T; N];

    type IntoIter = RangeIter<T, N>;

    fn into_iter(self) -> RangeIter<T, N> {
        RangeIter {
            inner: self,
            x: self.p0,
        }
    }
}

pub struct RangeIter<T, const N: usize> {
    inner: NRange<T, N>,
    x: [T; N],
}

impl<T, const N: usize> Iterator for RangeIter<T, N>
where
    T: Copy + PartialOrd + One + Add<Output = T>,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..(N - 1) {
            if self.x[i] >= self.inner.p1[i] {
                self.x[i] = self.inner.p0[i];
                self.x[i + 1] = self.x[i + 1] + T::one();
            }
        }
        if self.x[N - 1] >= self.inner.p1[N - 1] {
            // Out of content.
            return None;
        }
        let ret = self.x;
        self.x[0] = self.x[0] + T::one();
        Some(ret)
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
    fn test_indexes() {
        let bounds: NRange<i32, 3> = NRange::new([1, 2, 3], [4, 5, 6]);

        for (i, p) in bounds.into_iter().enumerate() {
            eprintln!("{i} {p:?}, {}", bounds.index_of(p));
            if i == 0 {
                assert_eq!(p, [1, 2, 3]);
            }
            assert_eq!(i, bounds.index_of(p));
            assert_eq!(bounds.get(i), p);
        }
    }

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
