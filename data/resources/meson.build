# Resources
resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
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
