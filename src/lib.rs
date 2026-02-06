//! `librank` is a small Rust library for adding ranking information to an iterator.
//!
//! The main entry point is the `RankedExt` trait, which provides the `rank_by` method
//! for any iterator.
//!
//! # Examples
//!
//! ```
//! use librank::Rank;
//! use librank::RankedExt;
//!
//! let data = vec![10, 20, 10, 30, 20, 10];
//! let ranked: Vec<(Rank, i32)> = data.into_iter().rank_by(|&x| x).collect();
//!
//! let expected = vec![
//!     (Rank(1), 10),
//!     (Rank(1), 10),
//!     (Rank(1), 10),
//!     (Rank(2), 20),
//!     (Rank(2), 20),
//!     (Rank(3), 30),
//! ];
//!
//! assert_eq!(ranked, expected);
//! ```

pub mod rank;

pub use rank::Rank;
pub use rank::RankedBy;
pub use rank::RankedExt;
