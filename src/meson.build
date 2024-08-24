target_dir = meson.project_build_root() / 'src'
cargo_args = ['--target-dir', target_dir]

if get_option('buildtype') == 'release'
  cargo_args += '--release'
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_build = custom_target(
  'cargo-build',
  build_always_stale: true,
  command: [find_program('cargo'), 'build', cargo_args],
  console: true,
  env: {
    'APP_ID': app_id,
    'CARGO_HOME': meson.project_build_root() / 'cargo-home',
    'GETTEXT_PACKAGE': gettext_package,
    'LOCALEDIR': get_option('prefix') / get_option('localedir'),
  },
  output: rust_target,
)

custom_target(
  'copy-binary',
  build_by_default: true,
  command: [
    find_program('cp'),
    target_dir / rust_target / meson.project_name(),
    '@OUTPUT@',
  ],
  depends: cargo_build,
  install: true,
  install_dir: get_option('bindir'),
  output: meson.project_name(),
)
