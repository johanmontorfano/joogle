// This module is only about local file storage.
pub mod local;

// Each of those modules are about a functionality linked to data stored on the
// database, and not related to specific routes.
pub mod jwt_auth;

// Each table of the database has it's own module here. Thus, the module `sites`
// contains actions that can be done on the `sites` table.
pub mod sites;
pub mod domains;
#[path = "./_[word].rs"]
pub mod _word;
