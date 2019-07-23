# rot

__*rot*__ is a command-line utility to cipher and decipher text using the [Caesar Cipher](https://en.wikipedia.org/wiki/Caesar_cipher).

## Usage

### Encrypt a simple phrase

```bash
$ rot 13 "The troops are dead"

Gur gebbcf ner qrnq
```

### Decrypt a simple phrase

```bash
$ rot 13 "Gur gebbcf ner qrnq" -d

The troops are dead
```

### Output to a file

```bash
$ rot 13 "The troops are dead" -o output.txt

$ cat output.txt

Gur gebbcf ner qrnq
```

### Read from a file

```bash
$ rot 13 output.txt -d -i

The troops are dead
```

### Read from `stdin`

```bash
$ echo "The troops are dead" | rot 13

Gur gebbcf ner qrnq
```

### Getting help

```bash
$ rot --help

Caesar Cipher Tool 0.1.0
José Duarte <jmg.duarte@campus.fct.unl.pt>
Tool to (de)cipher text using the Caesar Cipher

USAGE:
    rot [FLAGS] [OPTIONS] <rotation> [text]

FLAGS:
    -d               Decipher the input text
    -h, --help       Prints help information
    -i               Read text from file
    -V, --version    Prints version information

OPTIONS:
    -o <output>        Write result to a file

ARGS:
    <rotation>    Rotation value that should be used in the process
    <text>        Text for encoding/decoding
```

## Contributing

If you see a bug in the code, or want to add a feature, submit an [issue](https://github.com/jmg-duarte/rot/issues/new) or a pull request!