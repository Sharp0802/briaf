use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// A fractional integer, whose denominator is 6.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fract(i32);

impl Fract {
    /// A zero.
    pub const ZERO: Self = Self(0);

    /// Creates a new [`Fract`] from [`f32`].
    ///
    /// Note that precision is not guarenteed.
    #[must_use]
    pub const fn new(value: f32) -> Self {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "intentionally truncated to an int numerator"
        )]
        let q = (value * 6.0 + 0.5) as i32;
        Self(q)
    }

    /// Converts `self` to [`f32`].
    #[must_use]
    pub const fn to_f32(self) -> f32 {
        #![expect(clippy::cast_precision_loss, reason = "it was from f32")]
        let frt = (self.0 % 6) as f32;
        let int = (self.0 / 6) as f32;
        int + frt * (1.0 / 6.0)
    }
}

impl AddAssign for Fract {
    /// Adds `rhs` to `self`.
    ///
    /// # Panics
    ///
    /// Panics if the sum overflows and overflow checks are enabled.
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Fract {
    /// Subtracts `rhs` from `self`.
    ///
    /// # Panics
    ///
    /// Panics if the difference overflows and overflow checks are enabled.
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Fract {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl RemAssign for Fract {
    /// Assigns the remainder of division by `rhs` to `self`.
    ///
    /// # Panics
    ///
    /// Panics if any of the following conditions are met:
    ///
    /// - `rhs` is [`Self::ZERO`].
    /// - The numerator of `self` is [`i32::MIN`] and the numerator of `rhs` is `-1`.
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl Add for Fract {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`.
    ///
    /// # Panics
    ///
    /// Panics if the sum overflows and overflow checks are enabled.
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Fract {
    type Output = Self;

    /// Returns the difference between `self` and `rhs`.
    ///
    /// # Panics
    ///
    /// Panics if the difference overflows and overflow checks are enabled.
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f32> for Fract {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.to_f32() * rhs)
    }
}

impl Mul<Fract> for f32 {
    type Output = Fract;

    fn mul(self, rhs: Fract) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div for Fract {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.to_f32() / rhs.to_f32()
    }
}

impl Rem for Fract {
    type Output = Self;

    /// Returns the remainder of `self` divided by `rhs`.
    ///
    /// # Panics
    ///
    /// Panics if any of the following conditions are met:
    ///
    /// - `rhs` is [`Self::ZERO`].
    /// - The numerator of `self` is [`i32::MIN`] and the numerator of `rhs` is `-1`.
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl From<i32> for Fract {
    fn from(value: i32) -> Self {
        Self(value * 6)
    }
}

impl From<f32> for Fract {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<Fract> for f32 {
    fn from(value: Fract) -> Self {
        value.to_f32()
    }
}
