# Non-code data for your application is handled here.
# Think icons, UI files, settings schemas, and other assets.
# For your convenience, we included the essentials. Make sure to rename them (and modify the contents) to use your actual app iname.

subdir('icons')
subdir('ui')

# A desktop file provides basic information about the application to the shell.
# This allows the shell to display your application's name, icon, and other metadata.
# It's also required for visual launchers, such as the app drawer, to discover and launch your application.
# For more information about desktop files, see https://wiki.archlinux.org/title/desktop_entries and https://specifications.freedesktop.org/desktop-entry-spec/latest/

desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)
desktop_file = i18n.merge_file(
  input: configure_file(
    input: 'app.desktop.in.in',
    output: application_id + '.desktop.in',
    configuration: {
      'icon': application_id,
      'command': base_id,
    },
  ),
  output: application_id + '.desktop',
  po_dir: podir,
  type: 'desktop',
  install: true,
  install_dir: datadir / 'applications'
)

if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path()
    ],
    depends: desktop_file,
  )
endif

# Appstream is a standard specification for representing additional metadata about applications.
# This metadata will be used within software centers, such as tauOS's catalogue.
# For more information about appstream, see https://www.freedesktop.org/software/appstream/docs/
# There's also a generator that you might find useful: https://www.freedesktop.org/software/appstream/metainfocreator/

appdata_file = i18n.merge_file(
  input: configure_file(
    input: 'appdata.xml.in.in',
    output: application_id + '.appdata.xml.in',
    configuration: {
      'app-id': application_id,
      'gettext-package': gettext_package,
    },
  ),
  output: '@0@.metainfo.xml'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'metainfo'
)

if appstream_util.found()
  test(
    'validate-appdata', appstream_util,
    args: [
      'validate', '--nonet', appdata_file.full_path()
    ],
    depends: appdata_file,
  )
endif

# GSettings provides a simple interface for your application to store user settings.
# A gschema file declares information about your application's settings.
# For more information about GSettings and the gschema format, see: https://wiki.gnome.org/HowDoI/GSettings

install_data(
  configure_file(
    input: 'gschema.xml.in',
    output: application_id + '.gschema.xml',
    configuration: {
      'app-id': application_id,
      'app-path': path_identifier,
      'gettext-package': gettext_package,
    },
  ),
  install_dir: datadir / 'glib-2.0' / 'schemas'
)

if glib_compile_schemas.found()
  test(
    'validate-gschema', glib_compile_schemas,
    args: [
      '--strict', '--dry-run', meson.current_build_dir()
    ],
  )
endif

# GResources allow you to bundle and reference assets within your application.
# Resources are specified using a gresource file.
# For more information about GResources, see: https://docs.gtk.org/gio/struct.Resource.html

resources = gnome.compile_resources(
  'resources',
  configure_file(
    input: 'gresource.xml.in',
    output: 'resources.gresource.xml',
    configuration: {
      'app-path': base_path_identifier,
    },
  ),
  dependencies: blueprints,
  source_dir: meson.current_build_dir(),
  export: true
)

resources_lib = static_library(
  'resources',
  sources: resources,
  dependencies: [
    dependency('gio-2.0')
  ]
)

RESOURCES_DIR = meson.current_build_dir()
