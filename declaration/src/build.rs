use unit::ImageBuild;

/// The standalone `login` image build. `manifest` is provider-relative; the
/// Session/Vanllia cluster supplies its own manifest prefix.
pub const BUILD: ImageBuild = ImageBuild {
    image: "login",
    manifest: "server/Cargo.toml",
    package: "login-server",
    bin: "login",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
