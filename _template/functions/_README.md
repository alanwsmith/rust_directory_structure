This is where to setup functions
that aren't inside of structs. To make one:

- Update the `mod.rs` file to change every
occurrence of `example_function` to whatever
you want your function's name to be and 
uncomment the lines.

- Change the name of `example_function.rs` to
match your function

- Change all instances of `example_function` and
`example_function_test` to match your 
function name

Note that until you uncomment the lines in 
`mod.rs` the function files are out of the
mix and won't be tested or used. 

