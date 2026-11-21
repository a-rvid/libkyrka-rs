use libkyrka::{error::KyrkaNativeError::NoContext, kyrka::KyrkaConfig, prelude::*};

#[test]
fn context_works_until_emergency_erase() {
    let kyrka_config: KyrkaConfig = Kyrka::builder()
        .mtu(1422)
        .shroud(true)
        .secret(['A' as u8; 32])
        .validate()
        .unwrap();

    let mut kyrka: Kyrka = kyrka_config.load().unwrap();
    kyrka.set_mtu(1342).unwrap();

    kyrka_emergency_erase();

    assert!(kyrka.set_mtu(1234).is_err());
    assert_eq!(kyrka.last_error(), KyrkaError::ErasedContext);
}


#[test]
fn test_sys() {
    let version = libkyrka::kyrka_version();
    assert!(!version.is_empty());
}

// #[test]
// fn cathedral_config_load() {
//     let 
// }