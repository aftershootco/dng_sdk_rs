use std::ops::{Add, DivAssign};

pub trait Hypotenuse: Copy {
    fn hypotenuse(&self, other: Self) -> Self;
}
impl Hypotenuse for f32 {
    #[inline]
    fn hypotenuse(&self, other: Self) -> Self {
        self.hypot(other)
    }
}
impl Hypotenuse for f64 {
    #[inline]
    fn hypotenuse(&self, other: Self) -> Self {
        self.hypot(other)
    }
}

// class dng_1d_function
// 	{

// 	public:

// 		virtual ~dng_1d_function ();

// 		/// Returns true if this function is the map x -> y such that x == y for all x.
// 		/// That is if Evaluate(x) == x for all x.

// 		virtual bool IsIdentity () const;

// 		/// Return the mapping for value x.
// 		/// This method must be implemented by a derived class of dng_1d_function and
// 		/// the derived class determines the lookup method and function used.
// 		/// \param x A value between 0.0 and 1.0 (inclusive).
// 		/// \retval Mapped value for x

// 		virtual real64 Evaluate (real64 x) const = 0;

// 		/// Return the reverse mapped value for y.
// 		/// This method can be implemented by derived classes. The default implementation
// 		/// uses Newton's method to solve for x such that Evaluate(x) == y.
// 		/// \param y A value to reverse map. Should be within the range of the function

pub trait LinearFn {
    /// Returns true if this function is the map x -> y such that x == y for all x.
    /// That is if Evaluate(x) == x for all x.
    fn is_identity() -> bool;

    /// Return the mapping for value x.
    /// This method must be implemented by a derived class of dng_1d_function and
    /// the derived class determines the lookup method and function used.
    /// \param x A value between 0.0 and 1.0 (inclusive).
    /// \retval Mapped value for x

    fn evaluate(x: f64) -> f64;
    /// Return the reverse mapped value for y.
    /// This method can be implemented by derived classes. The default implementation
    /// uses Newton's method to solve for x such that Evaluate(x) == y.
    /// \param y A value to reverse map. Should be within the range of the function

    fn reverse_evaluate(y: f64) -> f64;
}

pub struct Identity;

impl LinearFn for Identity {
    fn is_identity() -> bool {
        true
    }

    fn evaluate(x: f64) -> f64 {
        x
    }

    fn reverse_evaluate(y: f64) -> f64 {
        y
    }
}

pub trait Normalize: Sized {
    fn normalize(self) -> Self {
        let mut v = self;
        v.normalize_in_place();
        v
    }
    fn normalize_in_place(&mut self);
}

impl<T, const LEN: usize> Normalize for [T; LEN]
where
    T: DivAssign + Add<Output = T> + Copy,
{
    fn normalize_in_place(&mut self) {
        if LEN == 0 {
            return;
        }
        let sum = self[0];
        let sum = self.iter().take(LEN).skip(1).fold(sum, |sum, v| sum + *v);
        self.iter_mut().take(LEN).for_each(|v| *v /= sum);
    }
}

pub trait SmoothStep: Sized {
    fn smooth_step(self) -> Self;
    fn smooth_step_in_place(&mut self);
}

impl<const LEN: usize> SmoothStep for [f64; LEN] {
    fn smooth_step(self) -> Self {
        let mut v = self;
        v.smooth_step_in_place();
        v
    }

    fn smooth_step_in_place(&mut self) {
        if LEN == 0 {
            return;
        }
        self.iter_mut()
            .take(LEN)
            .for_each(|v| *v = v.powi(2) * (3.0 - 2.0 * *v));
    }
}
