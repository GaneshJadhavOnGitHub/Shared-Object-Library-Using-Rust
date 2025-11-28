# Shared-Object-Library-Using-Rust
A sample project which creates a shared object (.so) library on linux using Rust programming language and calls it's function from another rust program.

-------------------
Shared Object (.so) File:- 

In simple words, a '.so' file on Linux is similar to a 'dll' on Windows. 

It is a library that is dynamically linked to the executable/binary, meaning it's not embedded inside the executable/binary like a static library (.a). Instead, the executable/binary contains references to the .so file, which is loaded into memory when the executable/binary is run and the library's functionality is referenced. This dynamic linking allows multiple executables/binaries to share the same library in memory, promoting efficient use of system resources.

------------------
For System Requirements please refer 'Application_Requirements.txt'
 
----------------------

How to run the project.

1. Clone the repository.

2. Navigate inside project folder 'addition_library' from terminal.

3. Build this Library project using following command -  
   cargo build

4. Navigate inside project folder 'addition_client' from terminal.  

5. Build and run this binary project using following commands -

   cargo build

   cargo run
   
------------------------

Application is tested on Ubuntu 20.04.5 LTS with WSL2 on Windows 10, working well.

Tested on Ubuntu 22.04.2 LTS using github actions, working well.

-----------------------

Output :- 

1. Build Library project : 

![Output1](https://user-images.githubusercontent.com/86361080/234288935-f37ecda9-c406-43b4-92e6-0f83f0ce7c1d.png)


2. Build and Run Binary project :

![Output2](https://user-images.githubusercontent.com/86361080/234289054-19efb663-2859-45d9-901c-46ac780ce601.png)

--------------------
   
   
__addition_library project dependency tree__

```
addition_library v0.1.0 () - 

```
__addition_client project dependency tree__

```
addition_client v0.1.0  () - 
`-- addition_library feature "default"
    `-- addition_library v0.1.0 () - 

```

__Repository Tree Structure__

```
├── .github
    └── workflows
    │   └── rust.yml
├── Application_Requirements.txt
├── LICENSE
├── Output1.png
├── Output2.png
├── README.md
└── SharedObjectFile
    ├── addition_client
        ├── .gitignore
        ├── Cargo.lock
        ├── Cargo.toml
        └── src
        │   └── main.rs
    └── addition_library
        ├── .gitignore
        ├── Cargo.toml
        └── src
            └── lib.rs

```
