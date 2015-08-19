// Copyright 2015 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

extern crate ring;

#[cfg(test)]
extern crate rustc_serialize;

mod der;
mod input;
mod signed_data;

use input::Input;


pub enum PublicKey<'a> {
    EC(Input<'a>, &'static ring::EllipticCurve),
    RSA(Input<'a>)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    BadDER,
    BadSignature,
    Fatal(FatalError),
    UnsupportedEllipticCurve,
    UnsupportedKeyAlgorithm,
    UnsupportedSignatureAlgorithm,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FatalError {
    ImpossibleState,
}
