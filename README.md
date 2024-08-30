# Script Packet Manager (SPM)

# WIP

Script Packet Manager (SPM) is a tool for transforming Lua scripts that use the `require` syntax to a custom format. This is useful for managing dependencies in a controlled environment and avoiding the use of global variables.

## Features

- Reads and processes Lua files to replace `require` statements with a custom-defined function.
- Ensures that dependencies are properly loaded and managed.
- Easier library managment.

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

