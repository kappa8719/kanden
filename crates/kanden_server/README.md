# `kanden_server`

Defines the "core" of the Kanden server that plugins depend on. If a plugin module here is large enough, it may be split off into its own crate to reduce compile times.

The contents of `kanden_server` are re-exported from the main `kanden` crate, so end users should not interact with this crate directly.
