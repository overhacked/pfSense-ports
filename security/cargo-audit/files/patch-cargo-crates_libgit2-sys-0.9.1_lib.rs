Revert to libgit2 0.28 API per https://github.com/rust-lang/git2-rs/issues/458

--- cargo-crates/libgit2-sys-0.9.1/lib.rs.orig	2019-08-16 20:27:18 UTC
+++ cargo-crates/libgit2-sys-0.9.1/lib.rs
@@ -331,7 +331,6 @@ pub struct git_remote_callbacks {
     pub push_negotiation: Option<git_push_negotiation>,
     pub transport: Option<git_transport_cb>,
     pub payload: *mut c_void,
-    pub resolve_url: Option<git_url_resolve_cb>,
 }
 
 #[repr(C)]
@@ -385,8 +384,6 @@ pub type git_push_negotiation =
 
 pub type git_push_update_reference_cb =
     extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int;
-pub type git_url_resolve_cb =
-    extern "C" fn(*mut git_buf, *const c_char, c_int, *mut c_void) -> c_int;
 
 #[repr(C)]
 pub struct git_push_update {
@@ -2233,7 +2230,7 @@ extern "C" {
         source: *const git_tree,
     ) -> c_int;
     pub fn git_treebuilder_clear(bld: *mut git_treebuilder);
-    pub fn git_treebuilder_entrycount(bld: *mut git_treebuilder) -> size_t;
+    pub fn git_treebuilder_entrycount(bld: *mut git_treebuilder) -> c_uint;
     pub fn git_treebuilder_free(bld: *mut git_treebuilder);
     pub fn git_treebuilder_get(
         bld: *mut git_treebuilder,
