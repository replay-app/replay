desktop_file = i18n.merge_file(
  input: configure_file(
    configuration: {'command': meson.project_name(), 'icon': app_id},
    input: f'@rdnn_app_name@.desktop.in.in',
    output: f'@rdnn_app_name@.desktop.in',
  ),
  install: true,
  install_dir: get_option('datadir') / 'applications',
  output: f'@app_id@.desktop',
  po_dir: meson.project_source_root() / 'i18n',
  type: 'desktop',
)

desktop_file_validate = find_program('desktop-file-validate', required: true)
if desktop_file_validate.found()
  test('desktop-file-validation', desktop_file_validate, args: [desktop_file])
endif
