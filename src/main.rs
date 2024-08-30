use std::fs;
use std::path;
use args::{SPMArgs, SPMCommand};
use clap::Parser;
use regex::Regex;
mod args;





fn main() {

    let args = SPMArgs::parse();

    match args.command {
        SPMCommand::Build { file } => {

            // Used "clone" not really optimised but works fine.
            // Storing the first path to a variable
            let mut correct_file = file.clone();

            // Getting the current directory
            let current_dir = std::env::current_dir().expect("Couldn't find the current directory");

            // Prasing the file as a Path Buffer
            let current_file = current_dir.join(path::Path::new(&file));

            //Checking if the directory exists so we can do [spm build test.lua] insted of providing the full path
            if current_file.exists() {
                correct_file = current_file.to_str().expect("Couldn't prase <PATH> to <STRING>").to_string();
                return;
            }
            
            //Reading the contents of the file
            let contents = fs::read_to_string(correct_file).expect("Failed to read the source file.");

            //Getting the "require" parts of the file with regex
            let re = Regex::new(r#"local\s+(\w+)\s*=\s*require\("([^"]+)"\)"#).expect("Couldn't find any 'require' matches");

            let mut modified_contents = contents.clone();




            // looping trough the captures and changing the req to a func
            for capture in re.captures_iter(&contents) {

                //geting the name of the modules
                let variable_name = &capture[1]; // Captures the variable name (e.g., x)
                let module_name = &capture[2]; // Captures the module name (e.g., dep)

                // saving the vari



                //
                // get the path of the module
                let module_path = current_dir.join(format!("spm_modules/{}.lua",module_name));

                // check if it exists
                if !module_path.exists() {
                    println!("Couldn't find the dependency, you may not have it installed.");
                    return;
                }

                // read the contents
                let contents = fs::read_to_string(module_path).expect("Couldn't read the module code.");

                // indent the contents
                let indented_contents: String = contents
                    .lines()  
                    .map(|line| format!("\t{}", line))  
                    .collect::<Vec<String>>()  
                    .join("\n");  

                // create a replacement
                let replacement = format!("local {} = function()\n{}\nend\n{}={}()\nif {} == nil or type({}) ~= 'table' then\n\twarn('Error while loading the library: {}');\n\treturn;\nend",
                 variable_name,indented_contents,variable_name,variable_name,variable_name,variable_name,module_name);

                // replace.
                modified_contents = modified_contents.replacen(&capture[0], &replacement, 1);
            }

            // Write to output
            fs::write("output.lua", format!("--file built by SPM\n\n{}",modified_contents)).expect("Couldn't write output.lua");

            println!("Successfully built the script.");

            


        },
        SPMCommand::Add { name } => {

        }
    }
}






