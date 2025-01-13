project('hello',
        ['c','cpp'],
        version: files('version'),
        meson_version: '>=1.0.0',
        default_options : ['warning_level=3', 'cpp_std=c++20'],
        license: 'GPL3')

# Install directory
install_dir = meson.project_source_root() / 'install' / meson.project_name()

# Files, libs, etc.
src = ['src/hello.c']
include = ['include']

# Target
lib = static_library('hello',
                     src,
                     include_directories : include,
                     install : true,
                     install_dir : install_dir
                     )

# Install include files
install_subdir('include', install_dir : install_dir)

# Dependency for a top level project
hello_dep = declare_dependency(link_with : lib, include_directories: include)
meson.override_dependency(meson.project_name(), hello_dep)

# Tests
hello_test = executable('test_hello',
                  'test/unit/test_hello.cpp',
                  dependencies : hello_dep,
                  install : false,
                  install_dir : install_dir
                  )
test('test_hello', hello_test)
