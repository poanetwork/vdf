# vdf
An implementation of Verifiable Delay Functions (VDFs) in Rust

**This is currently highly expirimental.  Use at your own risk.**

## What is a VDF?

A VDF is a tuple of functions *Prepare*, *Eval*, and *Verify*.  *Prepare* generates public parameters for use by the other two.  *Eval* evaluates the function, and *Verify* verifies that *Eval* was computed correctly.  Furthermore, *Eval* must take time proportional to *n*, where *n* was passed to *Prepare*, even on a polynomial number of parallel processors.  *Verify*, however, must only take time polylogarithmic in *n*.

This has many uses.  See <https://eprint.iacr.org/2018/601> for details.
