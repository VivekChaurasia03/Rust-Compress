extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
/* I want to accept the name of the file to be compressed from the user.
We are basically building a CLI tool and that's why we needs args. */
use std::env::args;
/* As we are working with a File we need that from the fs. */
use std::fs::File;
/* We need a reader to read from the file that is to be compressed. */
use std::io::BufReader;
/* When you take a file that you want to compress you are basically copying
content from the file to the compressed file and thats why we need copy. */
use std::io::copy;
/* we are using this to estimate the time it took to compress a file of x size. */
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        /* Use eprintln! only for error and progress messages.
        To tell the user how this tool works. */
        eprintln!("Usage: `source` `target`");
        return;
    }
    /* To run this tool we will do "cargo run source target" the BufReader will basically read the File
    that is passed into it. The File will pick the source with the help of args().nth(1) that we have passed.
    This is mutable because we want to mutate the input and create a compressed file out of it. */
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    /* This basically holds the name of the file that the user wants it to be named as. Passed
    as an argument. */
    let output = File::create(args().nth(2).unwrap()).unwrap();

    /* We are just creating an encoder variable to handle the writing part and letting it know throught
    the GzEncoder that it has to write in the output. */
    let mut encoder = GzEncoder::new(output, Compression::default());

    /* After creating an encoder. We have started the time to extract the time that has been elapsed. */
    let start = Instant::now();

    /* Copies the entire contents of a reader into a writer. This function will continuously read data
    from reader and then write it into writer in a streaming fashion until reader returns EOF.
    On success, the total number of bytes that were copied from reader to writer is returned. */
    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    /* The get_ref() method is part of the Read trait, and it returns a reference to the inner reader.
    metadata() is then called on this inner reader to get metadata.  */
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    /* On the other hand, output may be an instance of a type that directly implements the Write trait
    or another related trait. The Write trait might not have a get_ref() method, and you can directly
    call metadata() on the output instance. */
    println!("Target len: {:?}", output.metadata().unwrap().len());

    println!("Elapsed Time: {:?}", start.elapsed());
}
