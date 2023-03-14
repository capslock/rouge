# rouge_queue

This crate adds a `Queue` resource type for use with Bevy. Can be used as an
n-to-1 communication channel between systems, supporting both concurrent readers
and writers without needing exclusive access to the resource. Built on
[crossbeam](https://crates.io/crates/crossbeam)'s `SegQueue`.

Can be used with full Bevy via the plugin, or with just the `bevy_ecs` crate by
adding the `Queue` resources manually.