use core::ops::Deref;

pub use cipher;
pub use digest;

// TODO: is this renaming confusing?
// Using the name as is is not so nice
// `wg` is for "Rust Embedded Working Group (WG)"
// TODO: this pulls in the not-very-well-organised
// entire library, in particular not just traits and types.
// Would be worth being more explicit.
pub use embedded_hal as wg;

pub use rand_core;

// TODO: Add more as needed,
// - internal
// - specific (CASPER, PUF, etc.)
// - experiments
// - etc.
//
// Idea is to try and have (peripheral) drivers implement
// a well-designed trait.

pub mod reg_proxy {
    /// Implemented for registers that `RegProxy` can proxy
    ///
    /// Use the `reg!` macro to implement this trait for a register from a crate
    /// generated by svd2rust.
    ///
    /// # Safety
    ///  The pointer returned by `get` must be valid for the duration of the program.
    pub unsafe trait Reg {
        /// The type that `RegProxy` should derefence to
        ///
        /// If only one instance of the register exists, this should be `Self`.
        /// If the same type in the svd2rust API is used to represent registers at
        /// multiple memory locations, this trait must be implemented for a type
        /// that represents a specific register at a specific location, and `Target`
        /// must be the common type.
        type Target;

        /// Return a pointer to the memory location of the register
        fn get() -> *const Self::Target;
    }

    /// # Safety
    ///  The pointer returned by `get` must be valid for the duration of the program.
    pub unsafe trait RegCluster {
        /// The type that `RegProxy` should derefence to
        ///
        /// If only one instance of the register exists, this should be `Self`.
        /// If the same type in the svd2rust API is used to represent registers at
        /// multiple memory locations, this trait must be implemented for a type
        /// that represents a specific register at a specific location, and `Target`
        /// must be the common type.
        type Target;

        /// Return a pointer to the memory location of the register
        fn get() -> *const [Self::Target];
    }
}

// maybe put in submodule?
pub trait Gint: Deref<Target = crate::raw::gint0::RegisterBlock> {}

pub mod aligned;
pub mod flash;
pub mod usb;
