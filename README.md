# rot

__*rot*__ is a command-line utility to cipher and decipher text using the [Caesar Cipher](https://en.wikipedia.org/wiki/Caesar_cipher).

## Note

Using equals with options is required, otherwise `rot` may crash. 
This is due to the lack of support of the YAML configuration ([this is fixed in `clap` 3.0.0](https://github.com/clap-rs/clap/issues/1426))

## Usage

### Encrypt a simple phrase

```bash
$ rot -c=13 "The troops are dead"

Gur gebbcf ner qrnq
```

### Decrypt a simple phrase

```bash
$ rot -d=13 "Gur gebbcf ner qrnq"

The troops are dead
```

### Try multiple combinations

#### Encrypt

```bash
$ rot -c=3,5,7,13 "The troops are dead"

Wkh wurrsv duh ghdg
Ymj ywttux fwj ijfi
Aol ayvvwz hyl klhk
Gur gebbcf ner qrnq
```

#### Decrypt

```bash
$ rot -d=3,5,7,13 "Gur gebbcf ner qrnq"

Dro dbyyzc kbo nokn
Bpm bzwwxa izm lmil
Znk zxuuvy gxk jkgj
The troops are dead
```

### Output to a file

```bash
$ rot -c=13 -o=output.txt "The troops are dead"

$ cat output.txt

Gur gebbcf ner qrnq
```

### Read from a file

```bash
$ rot -d=13 -f output.txt

The troops are dead
```

### Read from `stdin`

```bash
$ echo "The troops are dead" | rot -c=13 # implicit

Gur gebbcf ner qrnq

$ echo "The troops are dead" | rot -c=13 --stdin

Gur gebbcf ner qrnq
```

### Execute a blind brute force attack

```bash
$ rot -b "Gur gebbcf ner qrnq"

Ftq fdaabe mdq pqmp
Esp eczzad lcp oplo
Dro dbyyzc kbo nokn
Cqn caxxyb jan mnjm
Bpm bzwwxa izm lmil
Aol ayvvwz hyl klhk
Znk zxuuvy gxk jkgj
Ymj ywttux fwj ijfi
Xli xvsstw evi hieh
Wkh wurrsv duh ghdg
Vjg vtqqru ctg fgcf
Uif usppqt bsf efbe
The troops are dead # ( ͡° ͜ʖ ͡°)
Sgd sqnnor zqd cdzc
Rfc rpmmnq ypc bcyb
Qeb qollmp xob abxa
Pda pnkklo wna zawz
Ocz omjjkn vmz yzvy
Nby nliijm uly xyux
Max mkhhil tkx wxtw
Lzw ljgghk sjw vwsv
Kyv kiffgj riv uvru
Jxu jheefi qhu tuqt
Iwt igddeh pgt stps
Hvs hfccdg ofs rsor
```

### Getting help

```bash
$ rot --help

rot 0.2.0
José Duarte <jmg.duarte@campus.fct.unl.pt>

USAGE:
    rot [FLAGS] [OPTIONS] [--] [input]

FLAGS:
    -b, --brutef     Brute force all possible rotations
    -h, --help       Prints help information
        --stdin      Read input from "stdin"
    -V, --version    Prints version information

OPTIONS:
    -c, --cipher <cipher>...        Cipher input with the given rotation(s)
    -d, --decipher <decipher>...    Decipher input with the given rotation(s)
    -f, --file <file>               Read [input] as a file
    -o, --output <output>           Define an output file

ARGS:
    <input>    Text to (de)cipher
```

## Contributing

If you see a bug in the code, or want to add a feature, submit an [issue](https://github.com/jmg-duarte/rot/issues/new) or a pull request!