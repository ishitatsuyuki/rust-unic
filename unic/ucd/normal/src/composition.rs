// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use unic_utils::CharBsearchTable;


pub struct Slice {
    pub offset: u16,
    pub length: u16,
}

fn bsearch_lookup_table<T>(
    c: char,
    r: &'static [(char, Slice)],
    chars_table: &'static [T],
) -> Option<&'static [T]> {
    r.find(c).map(|slice| {
        let offset = slice.offset as usize;
        let length = slice.length as usize;
        &chars_table[offset..(offset + length)]
    })
}

// == Canonical Composition (C) ==
const CANONICAL_COMPOSITION_LOOKUP: &'static [(char, Slice)] =
    include!("tables/canonical_composition_mapping_lookup.rsv");
const CANONICAL_COMPOSITION_VALUES: &'static [(char, char)] =
    include!("tables/canonical_composition_mapping_values.rsv");

/// Canonical Composition of the character.
pub fn canonical_composition(c: char) -> Option<&'static ([(char, char)])> {
    bsearch_lookup_table(
        c,
        CANONICAL_COMPOSITION_LOOKUP,
        CANONICAL_COMPOSITION_VALUES,
    )
}

// == Canonical Decomposition (D) ==
const CANONICAL_DECOMPOSITION_LOOKUP: &'static [(char, Slice)] =
    include!("tables/canonical_decomposition_mapping_lookup.rsv");
const CANONICAL_DECOMPOSITION_VALUES: &'static [char] =
    include!("tables/canonical_decomposition_mapping_values.rsv");

/// Canonical Decomposition of the character.
pub fn canonical_decomposition(c: char) -> Option<&'static [char]> {
    bsearch_lookup_table(
        c,
        CANONICAL_DECOMPOSITION_LOOKUP,
        CANONICAL_DECOMPOSITION_VALUES,
    )
}

// == Compatibility Decomposition (KD) ==
const COMPATIBILITY_DECOMPOSITION_LOOKUP: &'static [(char, Slice)] =
    include!("tables/compatibility_decomposition_mapping_lookup.rsv");
const COMPATIBILITY_DECOMPOSITION_VALUES: &'static [char] =
    include!("tables/compatibility_decomposition_mapping_values.rsv");

/// Compatibility Decomposition of the character.
pub fn compatibility_decomposition(c: char) -> Option<&'static [char]> {
    bsearch_lookup_table(
        c,
        COMPATIBILITY_DECOMPOSITION_LOOKUP,
        COMPATIBILITY_DECOMPOSITION_VALUES,
    )
}
