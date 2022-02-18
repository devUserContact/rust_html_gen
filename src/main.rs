extern crate rand;

use std::io::Write;
use std::fs::OpenOptions;
use rand::thread_rng;
use rand::Rng;

fn main() {
    
    let mut ofile = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open("./output/index.html")
        .expect("unable to create file");

    let header: &str = concat!(
        "<!DOCTYPE html>\n",
        "<html lang=\"en\">\n",
        "<head>\n",
        "\t<style>\n",
        "\t\thtml, body {\n",
        "\t\t\theight: 100%;\n",
        "\t\t\tmargin: 0px;\n",
        "\t\t}\n",
        "\t\t.container {\n",
        "\t\t\theight: 100%;\n",
        "\t\t}\n",
        "\t\t.grid {\n",
        "\t\t\tdisplay: grid;\n",
        "\t\t\tgrid-template-columns: 1fr 1fr 1fr;\n",
        "\t\t\tgrid-template-rows: 1fr 1fr 1fr;\n",
        "\t\t\theight: 100%;\n",
        "\t\t}\n",
        "\t\t.wrapper {\n",
        "\t\t\tmargin: 1%;\n",
        "\t\t}\n",
        "\t</style>\n",
        "</head>\n",
        "<body>\n",
        "\t<div class=\"container\">\n",
        "\t\t<div class=\"grid\">\n",
   );    
   ofile.write_all(header.as_bytes())    
       .expect("unable to write to file");

   for _i in 1..10 {

       let mut rgb_arr = [0u8; 3];    
       thread_rng().try_fill(&mut rgb_arr[..])
           .expect("unable to generate array");
       let rgb_values: Vec<String> = rgb_arr.iter().map(|n| n.to_string()).collect();

       writeln!(
           ofile, 
           "\t\t\t\t<div class=\"wrapper\" style=\"background-color: rgb({});\">\n\t\t\t\t</div>", 
           rgb_values.join(", ")
       )
            .expect("unable to append to file");
   }

   let tail: &str = concat!( 
        "\t\t</div>\n",
        "\t</div>\n",
        "</body>\n",
        "</html>",
   );
   ofile.write_all(tail.as_bytes())
       .expect("unable to append to file");
   println!("\ngenerated index.html\n");
}
