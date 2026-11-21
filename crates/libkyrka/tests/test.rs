use libkyrka::{cathedral::CathedralConfig, kyrka::KyrkaConfig, prelude::*};

#[test]
fn test_sys() {
    let version = libkyrka::kyrka_version();
    assert!(!version.is_empty());
}

#[test]
fn cathedral_config_load() {
    let kyrka_config: KyrkaConfig = Kyrka::builder().mtu(1422).shroud(false).validate().unwrap();

    let mut kyrka: Kyrka = kyrka_config.load().unwrap();

    let dir = std::env::temp_dir();
    let kek = dir.join("kyrka-test-kek");
    let cosk = dir.join("kyrka-test-cosk");
    let secret = dir.join("kyrka-test-secret");
    std::fs::write(&kek, [b'A'; 32]).unwrap();
    std::fs::write(&cosk, [b'B'; 64]).unwrap();
    std::fs::write(&secret, [b'C'; 32]).unwrap();
    let cathedral_cfg = CathedralConfig {
        kek: Some(kek),
        cosk: Some(cosk),
        secret: Some(secret),
        flock_src: 0xDEADBEEF,
        flock_dst: 0xDEADBEEF,
        hops: 0,
        group: 0xBEEF,
        tunnel: 0xF00D,
        identity: 0xCAFEBABE,
        hidden: false,
        remembrance: false,
    };
    kyrka.cathedral_config(cathedral_cfg).unwrap();
}
