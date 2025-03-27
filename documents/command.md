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

Command to cleanup resources from hard drive.

## OPTIONS

_-c, --config_ \<path>_ \
&emsp;Config file path(either absolute or relative path)

_-s, --source \<path>_ \
&emsp;Source file path(either absolute or relative path)

_-d, --destination \<path>_ \
&emsp;Destination directory path(either absolute or relative path)

_-h, --help_ \
&emsp;Display help for the command

## EXAMPLES

- Parse & create custom AST json file by source information provided in config file.

```sh
$ neaten --destination "absolute_or_relative_path" --type "folder_or_file" --patterns "dist,node_modules"
$ neaten --destination "absolute_or_relative_path" --type "folder_or_file" --patterns dist --patterns node_modules
$ neaten --config "absolute_or_relative_path"
```

- Parse & create custom AST json file from passing source file and store in destination directory.

```sh
$ wl parse --source "/usr/pool/sample/ProTool.h" --destionation "/usr/pool/sample/build"
$ wl p --source "/usr/pool/sample/ProTool.h" --destionation "/usr/pool/sample/build"

$ wl parse -s "/usr/pool/sample/ProTool.h" -d "/usr/pool/sample/build"
$ wl p -s "/usr/pool/sample/ProTool.h" -d "/usr/pool/sample/build"
```

- Parse & create custom AST json file from passing source file and store in current working directory.

```sh
$ wl parse --source "/usr/pool/sample/ProTool.h"
$ wl p --source "/usr/pool/sample/ProTool.h"

$ wl parse -s "/usr/pool/sample/ProTool.h"
$ wl p -s "/usr/pool/sample/ProTool.h"
```
