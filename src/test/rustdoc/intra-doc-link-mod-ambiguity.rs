// ignore-tidy-linelength

#![deny(intra_doc_resolution_failures)]


pub fn foo() {

}

pub mod foo {}
// @has intra_doc_link_mod_ambiguity/struct.A.html '//a/@href' '../intra_doc_link_mod_ambiguity/foo/index.html'
/// Module is [`module@foo`]
pub struct A;


// @has intra_doc_link_mod_ambiguity/struct.B.html '//a/@href' '../intra_doc_link_mod_ambiguity/fn.foo.html'
/// Function is [`fn@foo`]
pub struct B;
