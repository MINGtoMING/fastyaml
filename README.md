# Fast YAML

A Fast YAML Parser for Python. 

## Dependency
- Python ≥ 3.10
- Rust ≥ 1.63

## Installation
### 1. Build from source
```shell
git clone https://github.com/MINGtoMING/fastyaml.git
cd fastyaml
pip install maturin
maturin develop --release
```
### 2. install from PyPi
```shell
pip install fastyaml
```

### Usage
```python
import fastyaml

if __name__ == '__main__':
    with open("path/to/read.yaml", 'r') as fp:
        tmp = fastyaml.load(fp)
    
    with open("path/to/write.yaml", 'w') as fp:
        fastyaml.dump(tmp, fp)

```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>