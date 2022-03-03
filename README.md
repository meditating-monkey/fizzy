# **Fizzy**

A simple CLI built in Rust to generate files with boilerplate. 

---
## Usage/List of Commands  

- `Add` command will prompt you to enter the boilerplate for the extension you want to add. Once you have entered the boilerplate, simply press <kbd>Ctrl</kbd>+<kbd>Z</kbd>.	For example:
	```bash 
	#Boilerplate for .cpp
	fizzy add cpp 
	``` 
	or
	```bash
	#Having a separate boilerplate for files with extension .test.cpp
	fizzy add test.cpp
	```

- `Create` command will figure out the extension from the file name (it is important that you write the extension too along with the file name) and create the file in the current path with the set boilerplate.
	```bash
	fizzy create main.cpp 
	```
---

## How to run project

Before cloning the project, make sure you have rust setup on your computer.

Once that's done, simply type in your terminal: 
```bash
cargo run 
```
or 
```bash
cargo run --release
```

