use harper_core::weirpack::{Weirpack, WeirpackManifest};
#[test]
fn zip9_weirpack_unicode_round_trip_and_malformed_input() {
    let mut manifest = WeirpackManifest::new();
    manifest.set_author("Test Author");
    manifest.set_version("0.1.0");
    manifest.set_description("Test pack");
    manifest.set_license("MIT");
    let mut pack = Weirpack::new(manifest);
    pack.rules.insert("правило".into(), "test contents".into());
    let bytes = pack.to_bytes().unwrap();
    let parsed = Weirpack::from_bytes(&bytes).unwrap();
    assert_eq!(parsed.rules, pack.rules);
    assert!(Weirpack::from_bytes(b"not a zip").is_err());
}
#[test]
fn lru_detaches_before_panicking_key_drop() {
    use std::{
        hash::{Hash, Hasher},
        num::NonZeroUsize,
        panic::{AssertUnwindSafe, catch_unwind},
    };
    #[derive(Eq)]
    struct Key(u32, bool);
    impl PartialEq for Key {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl Hash for Key {
        fn hash<H: Hasher>(&self, h: &mut H) {
            self.0.hash(h);
        }
    }
    impl Drop for Key {
        fn drop(&mut self) {
            if self.1 {
                panic!("intentional key drop");
            }
        }
    }
    let mut cache = lru::LruCache::new(NonZeroUsize::new(2).unwrap());
    cache.put(Key(1, true), 10);
    assert!(catch_unwind(AssertUnwindSafe(|| cache.pop(&Key(1, false)))).is_err());
    assert_eq!(cache.len(), 0);
    for i in 2..10 {
        cache.put(Key(i, false), i);
    }
    assert_eq!(cache.get(&Key(9, false)), Some(&9));
    assert_eq!(cache.len(), 2);
}
