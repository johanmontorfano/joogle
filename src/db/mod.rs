// This module is only about local file storage.
pub mod local;

// Each table of the database has it's own module here. Thus, the module `sites`
// contains actions that can be done on the `sites` table.

pub mod sites;
pub mod domains;
#[path = "./_[word].rs"]
pub mod _word;
