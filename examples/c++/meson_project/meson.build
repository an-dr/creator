project('template_project',
        ['c','cpp'],
        version: files('version'),
        meson_version: '>=1.0.0',
        default_options : ['warning_level=3', 'cpp_std=c++20'],
        license: 'GPL3')

# Install directory
install_dir = meson.project_source_root() / 'install' / meson.project_name()

# Files, libs, etc.
all_src = ['src/main.cpp']
all_include = []
deps = []

#Add subprojects
hello_proj = subproject('hello')

# Target
bin = executable(   'template_project',
                    all_src,
                    include_directories : all_include,
                    install : true,
                    dependencies : [
                        dependency('hello')
                    ],
                    install_dir : install_dir   )
