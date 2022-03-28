# Cow translator 🐮

![](images/1.png)

![](images/screenshot.png)

Official moo translator to communicate with cows 🐮 

# Installation 📦

## Arch Linux 🐧

Cow translator is available in the AUR

```bash
yay -S cow-translator
```

## Other 🪟🐧

## With make - Linux 🐧

Run make

```bash
# 📂 cow-translator/
make
```

## Build from source - Linux 🐧 & Windows 🪟

**Clone this repo**

```bash
git clone https://github.com/SkwalExe/cow-translator.git
```

build with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
# 📂 cow-translator/
cargo build --release
```

**[ LINUX ONLY ] :** Move the binary

```bash
# 📂 cow-translator/
sudo cp target/release/cow-translator /usr/bin/cow-translator
```

**On windows** the executable will be `target\release\cow-translator.exe` you can move it wherever you want.

# Usage 📝

![](images/usage.png)

## Example 

```bash
$ cow-translator -- Hello 

 => mOoOoOO moooOoo mooOoOO mooOoOO mooOOOo mOOOOOo mOOOoOo mooOOOo moOoooO mooOoOO mooooOO
```

# Troubleshooting 🔧

**Error:**

```
/app/target/release/cow-translator: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by /app/target/release/cow-translator)
```

**Solution:**

Compile with 

```bash
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build  --release
```

# Docker 🐳

### Build 🛠️

```bash
# 📂 cow-translator/
docker build -t cow-translator .
```

### Run 🏃

```bash
docker run --rm -it cow-translator [OPTIONS]
```

# Uninstall 🗑

## With make

Run make uninstall

```bash
# 📂 cow-translator/
make uninstall
```

## Or
Just remove the binary

```bash
sudo rm /usr/bin/cow-translator
```

# final

If you have any problem, don't hesitate to open an issue

# contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

<a href="https://github.com/SkwalExe#ukraine"><img src="https://raw.githubusercontent.com/SkwalExe/SkwalExe/main/ukraine.jpg" width="100%" height="15px" /></a>