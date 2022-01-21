use hello_pdf::pdf::file::Pdf;
use std::fs::File;
use std::io::Write;

fn main() {
  let mut pdf = Pdf::new();
  let contents = pdf.build("Hello world!");
  let mut f = File::create("test.pdf").unwrap();
  f.write_all(contents).unwrap();
}
