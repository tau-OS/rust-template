# This is where your app's icons will be installed from.

# app.svg is the primary visible icon for your application, displayed in the dock, for instance.
# symbolic.svg is a symbolic representation of your icon, usually a black and white outline.
# source.svg isn't used in the build pipeline, but is provided as a convenience when designing your own icon.

install_data(
  get_option('profile') == 'default' ? 'app.svg' : 'app-devel.svg',
  install_dir: iconsdir / 'hicolor' / 'symbolic' / 'apps',
  rename: '@0@.svg'.format(application_id),
)

install_data(
  'symbolic.svg',
  install_dir: iconsdir / 'hicolor' / 'symbolic' / 'apps',
  rename: '@0@-symbolic.svg'.format(application_id),
)
