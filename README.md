# creator

![Version](https://img.shields.io/github/v/tag/an-dr/creator?filter=v*&label=Version&color=blue)

Application for creation new files and projects from user templates.

For the application design and architecture see [docs/design.md](docs/design.md).

## Environment variables

- `CREATOR_STORAGE` - path to the templates directory

## Template Structure

Storage Structure:

```plaintext
Storage/
    |- group_a/
    |  |- template_directory_a/
    |  |  |- [files and dirs]
    |  |- template_directory_b/
    |  |  |- [files and dirs]
    |  |  |- ...
    |  ...
    |
    |- group_b/
    |  |- template_directory_a/
    |  |  |- [files and dirs]
    |  ...
    |
    ...
    
```

Template variables:

- `#var_NAME_OF_VAR#` - variables in the file name and content
