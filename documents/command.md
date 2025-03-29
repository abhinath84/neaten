## COMMAND

neaten - Command to cleanup resources from hard drive

## SYNOPSIS

```sh
neaten  -c, --config        <config_file>
        -d, --destination   <destination_folder>
        -k, --kind          <folder/file>
        -p, --patterns      <comma_sep_string>
        -e, --exclude       <comma_sep_string>
            --dryrun        <bool>
        -h, --help
```

## DESCRIPTION

Command to cleanup resources from hard drive. command will remove all files or folders mentioned in `patterns` from `destionation` & it's sub directories.

You can remove files/folders for a specific destination directory or from multiple destination directories.

If you want to remove files/folder from a specific directory, then pass `destination`, `kind` & `patterns` as argument.

```sh
neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target,dist"
```

or

```sh
neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target" --patterns "dist"
```

For multiple destination directories pass `config` argument with config file path which configured with multiple destination directories.

```sh
 neaten --config "/usr/sample/config.json"
```

Config file must be a `json` file. There is **no** specific name for config file, you can choose any name.

Root object of the config file must be an **array**. Here is the format of each config option:

```json

{
    "destination": "<destination_path>",
    "kind": "folder/file",
    "patterns": ["pattern1", "pattern2"],
    "exclude": ["exclude1", "exclude2", "exclude3"]
}

```

> `exclude` is an optional field.

Refer [Config file Sample](#config-file-sample) section for more about config file.

> If you provide both `config` & `destination, kind, patterns` combo, command will continue with `config` argument.

You can dry-run to check which files or folders will be removed if execute the command. You can dry-run with optional argument `dryrun`.

Refer [EXAMPLES](#examples) section for more examples of how to use the command.

## OPTIONS

_-c, --config_ \<path>_ \
&emsp;Config file with path(absolute or relative path). Config file must be in `json` format.

_-d, --destination \<path>_ \
&emsp;destination directory path(absolute or relative path).

_-k, --kind \<enum>_ \
&emsp;what kind of item wants to remove. It's an enum type with value: _folder_ or _file_.

_-p, --patterns \<string>_ \
&emsp;List of patterns to remove(comma separated value). Pass a _comma(,)_ separated string or call multiple time.

_-e, --exclude \<string>_ \
&emsp;List of items to be excluded from remove(comma separated value). Pass a _comma(,)_ separated string or call multiple time.

_--dryrun \<enum>_ \
&emsp;dry-run mode to check list of item to be removed.

_-h, --help_ \
&emsp;Display help for the command.

## EXAMPLES

- Remove using config file.

```sh
$ neaten --config "/usr/sample/config.json"
$ neaten -c "/usr/sample/config.json"
```

- Remove folders by passing `destionation`, `kind` & `patterns` arguments.

```sh
$ neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target"
$ neaten -d "/usr/sample/rust" -k "folder" -p "target"

$ neaten --destination "/usr/sample/node" --kind "folder" --patterns "dist,node_modules"
$ neaten -d "/usr/sample/node" -k "folder" -p "dist,node_modules"

$ neaten --destination "/usr/sample/node" --kind "folder" --patterns dist --patterns node_modules
$ neaten -d "/usr/sample/node" -k "folder" -p dist -p node_modules
```

- Remove files by passing `destionation`, `kind` & `patterns` arguments.

```sh
$ neaten --destination "/usr/sample/rust" --kind "file" --patterns "exe"
$ neaten -d "/usr/sample/rust" -k "file" -p "exe"

$ neaten --destination "/usr/sample/node" --kind "file" --patterns "txt,log"
$ neaten -d "/usr/sample/node" -k "file" -p "txt,log"

$ neaten --destination "/usr/sample/node" --kind "file" --patterns txt --patterns "log"
$ neaten -d "/usr/sample/node" -k "file" -p txt -p "log"
```

- Remove items but exclude some items.

```sh
$ neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target" --exclude "obj"
$ neaten -d "/usr/sample/rust" -k "folder" -p "target" -e "obj"

$ neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target" --exclude "obj,dist"
$ neaten -d "/usr/sample/rust" -k "folder" -p "target" -e "obj,dist"

$ neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target" --exclude "obj" --exclude "dist"
$ neaten -d "/usr/sample/rust" -k "folder" -p "target" -e "obj" -e "dist"
```

- Dry-run.

```sh
$ neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target" --exclude "obj" --dryrun
$ neaten -d "/usr/sample/rust" -k "folder" -p "target" -e "obj" --dryrun

$ neaten --destination "/usr/sample/rust" --kind "folder" --patterns "target" --dryrun
$ neaten -d "/usr/sample/rust" -k "folder" -p "target" --dryrun
```

## Config file Sample

### Remove folder with or without `exclude` optional field

Currently removing of folder support full folder name, no _regular expression_ support is there. You must provide full folder name in fields like: `patterns`, `exclude`.

```json
[
  {
    "destination": "/usr/sample/rust",
    "kind": "folder",
    "patterns": ["target"],
    "exclude": ["special_sub_folder", "another_folder"]
  },
  {
    "destination": "/usr/sample/node",
    "kind": "folder",
    "patterns": ["dist", "node_modules"]
  },
  {
    "destination": "/usr/sample/C++",
    "kind": "folder",
    "patterns": ["build", "Debug", "Release"]
  }
]
```

### Remove file with or without `exclude` optional field

Currently removing of files support full extension name, no _regular expression_ support is there. You must provide full extension name in the field`patterns`. For optional field `exclude` you need to provide full file name.

```json
[
  {
    "destination": "/usr/sample/rust",
    "kind": "file",
    "patterns": ["toml"],
    "exclude": ["cargo.toml"]
  },
  {
    "destination": "/usr/sample/node",
    "kind": "file",
    "patterns": ["txt", "log"]
  },
  {
    "destination": "/usr/sample/C++",
    "kind": "file",
    "patterns": ["exe", "obj", "log"]
  }
]
```

### Remove folder and file with or without `exclude` optional field

```json
[
  {
    "destination": "/usr/sample/rust",
    "kind": "folder",
    "patterns": ["target"],
    "exclude": ["special_sub_folder", "another_folder"]
  },
  {
    "destination": "/usr/sample/rust",
    "kind": "file",
    "patterns": ["toml"],
    "exclude": ["cargo.toml"]
  },
  {
    "destination": "/usr/sample/node",
    "kind": "folder",
    "patterns": ["dist", "node_modules"]
  },
  {
    "destination": "/usr/sample/node",
    "kind": "file",
    "patterns": ["txt", "log"]
  },
  {
    "destination": "/usr/sample/C++",
    "kind": "folder",
    "patterns": ["build", "Debug", "Release"]
  },
  {
    "destination": "/usr/sample/C++",
    "kind": "file",
    "patterns": ["exe", "obj", "log"]
  }
]
```
