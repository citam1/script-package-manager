# Script Packet Manager (SPM)

# WIP

Script Packet Manager (SPM) is a tool for transforming Lua scripts that use the `require` syntax to a custom format. This is useful for managing dependencies in a controlled environment and avoiding the use of global variables.
SPM is very much a work-in-progress as I will add a server to upload dependecies and optimize the code.

## Features

- Reads and processes Lua files to replace `require` statements with a custom-defined function.
- Ensures that dependencies are properly loaded and managed.
- Easier library managment.

## TO DO
- Add a new server
- Read configuration files and check versions of dependencies
- Seamlessly integrate with Visual Studio Code (VSC)
- Optimize libraries by using shared dependencies where possible
- Much more

## Example Transformation

SPM transforms the following Lua code:


# FROM
```lua
local x = require("dep")

print(x.func())
```

# INTO

```lua
--file built by SPM

local x = function()
	local dep = {}
		
	function dep.func() 
	    return "something";
	end
	
	return dep
end
x=x()
if x == nil or type(x) ~= 'table' then
	warn('Error while loading the library: dep');
	return;
end

print(x.func());
```

# USAGE

Build the source by using the rust compiler, put the exe into your folder with lua scripts, all dependencies MUST in the spm_modules folder and the folder MUST be in the same dir as the exe.

```
spm.exe build <PATH_TO_SOURCE>
```

