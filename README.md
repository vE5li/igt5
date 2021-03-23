# description
leafsheep is a scripting language designed for building compilers by automating tasks such as tokenizing and parsing.

# build
For building on linux, you can use the install script provided in [yardstick](https://github.com/ve5li/yardstick) to install the entire toolchain. Simply clone the repository and execute the installer as such:
```
git clone https://github.com/ve5li/yardstick
cd yardstick/untility
./install.sh
```

If you only want to install leafsheep without the toolchain, make sure that you have the latest version of rust nightly installed on your system. You can install rust using this command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
With rust installed you can compile leafsheep with these commands:
```
git clone https://github.com/ve5li/leafsheep
cd leafsheep/
cargo build
sudo cp target/debug/leafsheep /usr/bin/leafsheep
```

# usage
Using seamonkey is pretty straight forward. Running it without any arguments will try to run the file ```project``` in your working directory.
You can specify a different project file with the ```-p``` flag and a different working directory can be specified with ```-d```.
Everything after ```-a``` will be passed to the projects main function as a parameter.

There is currently not a lot of documentation other than a list of valid instructions and conditions but there are compilers built using seamonkey, mainly [cipher](https://github.com/ve5li/cipher), [doofenshmirtz](https://github.com/ve5li/doofenshmirtz) and [entleman](https://github.com/ve5li/entleman).
There is also a project for testing the toolchain called [yardstick](https://github.com/ve5li/yardstick).
