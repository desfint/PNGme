# PNGme
A very basic Rust implementation of the PNG spec following the [PNGme Book](https://github.com/picklenerd/PNGme_book) project.

##Commands

**encode**: Encodes a message into a PNG file and saves the result in the same file, or in a new one if an output is given.

`pngme encode <FILE PATH> <CHUNK TYPE> <MESSAGE> [OUTPUT PATH]`

**decode**: Searches for a message hidden in a PNG file and prints it if one is found.

`pngme decode <FILE PATH> <CHUNK TYPE>`

**remove**: Removes a data chunk from a PNG file and saves the result in the same file, or in a new one if an output is given.

`pngme decode <FILE PATH> <CHUNK TYPE> [OUTPUT PATH]`

**print**: Prints all the chunks in a PNG file.

`pngme print <FILE PATH>`
