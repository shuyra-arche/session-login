use login_declaration::{BUILD, PROGRAM, unit};
use unit_contract::{document, record};

fn declaration() -> document::Unit<String> {
    toml_edit::de::from_str(include_str!("../../server/unit.toml")).unwrap()
}

fn expected<S: From<&'static str>>() -> document::Unit<S> {
    let legacy = &PROGRAM;
    assert_eq!(legacy.name, "login");
    assert_eq!(legacy.loader.image, BUILD.bin);
    assert!(legacy.operator.namespace.is_empty());
    assert!(legacy.identity.grants.is_empty());
    assert!(legacy.control.construction.is_empty());
    assert!(legacy.control.retirement.is_empty());
    assert!(legacy.supply.bindings.is_empty());
    assert_eq!(
        legacy.control.after,
        Some(
            [
                unit::Dependency::Bootstrap(unit::Bootstrap::Operator),
                unit::Dependency::Unit("terminal"),
                unit::Dependency::Unit("account"),
                unit::Dependency::Target(unit::SCENE),
            ]
            .as_slice()
        )
    );
    let providers = || {
        legacy
            .control
            .after
            .unwrap()
            .iter()
            .filter_map(|dependency| match dependency {
                unit::Dependency::Unit(id) => Some(S::from(*id)),
                unit::Dependency::Bootstrap(_) | unit::Dependency::Target(_) => None,
            })
            .collect()
    };
    assert_eq!(legacy.loader.space, unit::LoaderConfig::user("").space);
    assert_eq!(legacy.control.ready, unit::Ready::OnSpawn);
    assert_eq!(legacy.control.ending, Some(unit::Ending::Told));
    document::Unit {
        schema: 1,
        id: S::from(legacy.name),
        loader: document::Loader {
            image: S::from(legacy.loader.image),
            space: document::Space::User,
            args: legacy.loader.args.iter().map(|arg| *arg as u64).collect(),
            stack: legacy.loader.stack as u64,
        },
        operator: document::Operator { namespace: vec![] },
        identity: document::Identity {
            aliases: legacy
                .identity
                .aliases
                .iter()
                .map(|id| S::from(*id))
                .collect(),
            grants: vec![],
        },
        control: document::Control {
            creation: document::Creation::Startup,
            requires: providers(),
            after: providers(),
            ready: document::Ready::Spawn,
            ending: document::Ending::Told,
            supervision: legacy.control.supervision.map(S::from),
            construction: vec![],
            retirement: vec![],
            groups: vec![],
            guarded: false,
        },
        supply: document::Supply { bindings: vec![] },
    }
}

#[test]
fn toml_preserves_every_legacy_login_field() {
    let declared = declaration();
    declared.validate().unwrap();
    assert_eq!(declared, expected::<String>());
}

#[test]
fn image_is_the_real_cargo_binary() {
    let manifest = include_str!("../../server/Cargo.toml")
        .parse::<toml_edit::DocumentMut>()
        .unwrap();
    let declared = declaration();
    assert_eq!(manifest["package"]["name"].as_str(), Some(BUILD.package));
    assert_eq!(BUILD.manifest, "server/Cargo.toml");
    assert_eq!(declared.loader.image, BUILD.image);
    assert!(
        manifest["bin"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .any(|bin| {
                bin.get("name").and_then(toml_edit::Item::as_str)
                    == Some(declared.loader.image.as_str())
                    && bin.get("path").and_then(toml_edit::Item::as_str) == Some("src/main.rs")
            })
    );
}

#[test]
fn binary_record_preserves_every_login_field() {
    let encoded = record::encode(&declaration()).unwrap();
    let decoded = record::decode(&encoded).unwrap();
    assert_eq!(decoded, expected::<&str>());
    let begin = encoded.as_ptr() as usize;
    let id = decoded.id.as_ptr() as usize;
    assert!(id >= begin && id + decoded.id.len() <= begin + encoded.len());
}
