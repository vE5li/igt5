# description
Seamonkey is a scripting language designed for building compilers by automating tasks such as tokenizing and parsing.

# build
For building on linux, please use the install script provided in [nt33](https://github.com/ve5li/nt33). Simply clone the repository and execute the installer as such:
```
git clone https://github.com/ve5li/nt33
cd nt33/untility
./install.sh
```

# usage
Using seamonkey is pretty straight forward. Running it without any arguments will try to run the file 'project' in your working directory.
You can specify a different project file with the ```-p``` flag and a different working directory can be specified with ```-d```.
Everything after ```-a``` will be passed to the projects main function as a parameter.

There is currently not a lot of documentation other than a list of valid instructions and conditions but there are compilers built using seamonkey, mainly [h0vs](https://github.com/ve5li/h0vs), [ktl9](https://github.com/ve5li/ktl9) and [jts3](https://github.com/ve5li/jts3).
There is also a project that uses all three of these compilers called [nt33](https://github.com/ve5li/nt33).
