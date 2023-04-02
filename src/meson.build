# Values defined here are accessible from within Rust.
# This can be useful for setting build-time constants, such as the version or app id.
# See the config.rs.in file for more information.

config = configuration_data()
config.set_quoted('APP_ID', application_id)
config.set_quoted('APP_PATH', path_identifier)
if get_option('profile') == 'default'
  config.set_quoted('NAME_SUFFIX', '')
else
  config.set_quoted('NAME_SUFFIX', ' (Development)')
endif
config.set_quoted('VERSION', meson.project_version())
config.set_quoted('GETTEXT_PACKAGE', meson.project_name())
config.set_quoted('LOCALEDIR', localedir)

config = configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: config
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]

if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]
cargo_env += [ 'TARGET_DIR=' + RESOURCES_DIR ]

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: application_id,
  console: true,
  install: true,
  install_dir: bindir,
  depends: resources,
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)
