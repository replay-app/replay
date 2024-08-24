project(
  'replay',
  license: 'GPL-3.0-or-later',
  license_files: 'COPYING',
  meson_version: '>=1.1.0',
  version: '0.1.0',
)

i18n = import('i18n')

rdnn_app_name = 'one.naiara.Replay'
app_id_suffix = get_option('profile') == 'devel' ? '.Devel' : ''
app_id = f'@rdnn_app_name@@app_id_suffix@'
gettext_package = meson.project_name()

dependency('gio-2.0', version: '>=2.76')
dependency('glib-2.0', version: '>=2.76')
dependency('gobject-2.0', version: '>=2.76')
dependency('gtk4', version: '>=4.12')
dependency('libadwaita-1', version: '>=1.4')

subdir('i18n')
subdir('src')
subdir('dist')
