# Shared-Object-File-Sample-Rust
A sample project which creates a shared object (.so) file on linux and calls it's function from another program.

-------------------

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

Application is tested on Ubuntu 20.04.5 LTS with WSL2 on Windows 10.

-----------------------

Output :- 

1. Build Library project : 

![Output1](https://user-images.githubusercontent.com/86361080/234288935-f37ecda9-c406-43b4-92e6-0f83f0ce7c1d.png)


2. Build Binary project :

![Output2](https://user-images.githubusercontent.com/86361080/234289054-19efb663-2859-45d9-901c-46ac780ce601.png)

--------------------
   
   
