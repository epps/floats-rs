# Data in depth - Floating-point numbers

## Overview

This (tiny) repo covers my reading and notes from _Rust in Action_, "Chapter 5 - Data in depth," the section on floating-point numbers. Each commit reflects the work from a section (e.g. 5.4.2 "Isolating the sign bit").

## Questions

- `to_bits()` interprets the bits of the `f32` as a `u32` _to allow for bit manipulation_. Is bit manipulation only possible on unsigned integers?
