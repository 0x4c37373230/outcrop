# THIS PROJECT IS NOW OBSOLETE. SEE THE NEW [outcrop](https://github.com/0x4c37373230/outcrop-final)
#### I decided to archive all my standalone bedrock reverse engineering and modding projects in order to combine all of them into a single, user friendly tool so others can use them as well. outcrop and all other BDS tools will be continued and maintained there.

# outcrop
A windows BDS mod loader

To use this program place the executable in the BDS folder and create a plugins folder. Place all the DLLs you want to inject inside. When the program is first executed, a file named outcrop.cfg will be created. 
The file will contain a single value which determines if all mods in the plugins folder should be loaded. It's set to 0 by default. If set to 1, no GUI will appear, BDS will be started and all the DLLs in the folder will be injected. If set to 0 a GUI will appear where you can see a list of mods and specify the one you want to inject using the name of the file (excluding the .dll suffix)
