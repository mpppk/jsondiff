# jsondiff

A tool for outputs semantic difference of json.  
"semantic" means:
* sort object key before comparison
* sort array before comparison (optional, but currently can not be disabled)

## Installation

```sh
$ cargo install jsondiff
```

## Usage

test1.json
```json
{
  "b": "bv",
  "arr": [1,2,3],
  "arr2": [
    {"a":  "av", "obj": {"arr": [1,2,3], "z":  "zv"}},
    {"b":  "bv", "obj": {"arr": [2,3,1], "z":  "zv"}},
    {"c":  "cv", "obj": {"arr": [3,2,1], "z":  "zv"}}
  ]
}
```

test2.json
```json
{
  "b": "bv",
  "arr": [1,3,2],
  "arr2": [
    {"c":  "cv", "obj": {"arr": [2,3,1], "z":  "zv"}},
    {"b":  "bv", "obj": {"arr": [1,2,3], "z":  "zv"}},
    {"a":  "av", "obj": {"arr": [3,2,1], "z":  "zv"}}
  ]
}
```

test3.json
```json
{
  "b": "bv",
  "arr": [3,2,1],
  "arr2": [
    {"c":  "cv", "obj": {"arr": [1,2,3], "z":  "zv"}},
    {"b":  "bv", "obj": {"arr": [2,3,1], "z":  "zv"}},
    {"a":  "av", "obj": {"arr": [3,2,1,4], "z":  "zv"}}
  ]
}
```

```shell
$ jsondiff test1.json test2.json
// => no output (no difference)

$ jsondiff test1.json test3.json
10:           "arr": [
11:             1,
12:             2,
13: -           3
13: +           3,
13: +           4
14:           ],
15:           "z": "zv"
16:         }
----
```

### options

```shell
$ jsondiff --help
jsondiff 0.1.0
A tool for outputs semantic difference of json

USAGE:
    jsondiff [FLAGS] [OPTIONS] <file-path1> <file-path2>

FLAGS:
    -h, --help                      Prints help information
    -n, --output-normalized-json    Outputs normalized json as "normalized1.json" and "normalized2.json"
    -V, --version                   Prints version information

OPTIONS:
    -U <unified>        Generate diffs with <n> lines of context [default: 3]

ARGS:
    <file-path1>    
    <file-path2>   
```