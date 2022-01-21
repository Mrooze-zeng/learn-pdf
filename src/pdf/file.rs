use super::units;
use std::io::Write;

pub struct Pdf {
  pub contents: Vec<u8>,
  pub counter: usize,
  pub offsets: Vec<usize>,
}

impl Pdf {
  pub fn new() -> Self {
    let mut contents = Vec::new();
    writeln!(contents, "%PDF-1.3").unwrap();
    Self {
      contents,
      counter: 0,
      offsets: Vec::new(),
    }
  }
  pub fn get_new_object_id(&mut self) -> usize {
    self.counter += 1;
    self.counter
  }
  pub fn create_object(&mut self, contents: &str, id: usize) {
    self.offsets.push(self.contents.len() as usize);
    writeln!(self.contents, "{} 0 obj", id).unwrap();
    writeln!(self.contents, "{}", contents).unwrap();
    writeln!(self.contents, "endobj").unwrap();
  }
  pub fn create_text_stream(contents: &str) -> String {
    let text_contents = &format!(
      "BT 
      /F1 100 Tf
      10 400 Td
      ({}) Tj
    ET",
      contents
    );
    String::from(&format!(
      "<</Length {size}>> 
      stream
      {content}
      endstream
      ",
      size = text_contents.len(),
      content = &text_contents
    ))
  }
  pub fn set_trailer(&mut self, xref_start_offset: usize, xref_id: usize) {
    writeln!(self.contents, "trailer").unwrap();
    writeln!(self.contents, "<</Root 1 0 R /Size {}>>", xref_id).unwrap();
    writeln!(self.contents, "startxref").unwrap();
    writeln!(self.contents, "{}", xref_start_offset).unwrap();
  }
  pub fn set_xref_table(&mut self) -> (usize, usize) {
    let id = self.get_new_object_id();
    let xref_start_offset = self.contents.len();
    writeln!(self.contents, "xref").unwrap();
    writeln!(self.contents, "0 {}", id).unwrap();
    writeln!(self.contents, "0000000000 65535 f ").unwrap();
    for offset in &self.offsets {
      writeln!(self.contents, "{:010} 00000 n ", offset).unwrap();
    }
    (xref_start_offset, id)
  }
  pub fn set_end(&mut self) {
    writeln!(self.contents, "%%EOF").unwrap();
  }
  pub fn build(&mut self, contents: &str) -> &Vec<u8> {
    let first_obj_id = self.get_new_object_id();
    // let proc_set_id = self.get_new_object_id();
    let secode_obj_id = self.get_new_object_id();
    let third_obj_id = self.get_new_object_id();
    let forth_obj_id = self.get_new_object_id();
    self.create_object(
      &format!(
        "<</Type /Catalog /Pages {page_id} 0 R>>",
        page_id = secode_obj_id
      ),
      first_obj_id,
    );
    // self.create_object("[/PDF /Text]", proc_set_id);
    self.create_object(
      &format!(
        "<</Type /Pages /Kids [{kid} 0 R] /Count 1>>",
        kid = third_obj_id
      ),
      secode_obj_id,
    );
    self.create_object(
      &format!(
        "<<
        /Type /Page 
        /Parent {page_id} 0 R 
        /MediaBox [0 0 {page_width} {page_height}]
        /Resources
          <</Font
            <</F1
              <<
                /Type /Font 
                /Subtype /Type1 
                /BaseFont /Arial
              >>
            >>
          >>
        /Contents {content_id} 0 R
        >>
    ",
        page_id = 2,
        page_width = units::A4.0,
        page_height = units::A4.1,
        content_id = forth_obj_id
      ),
      third_obj_id,
    );
    let text_stream = Self::create_text_stream(contents);
    self.create_object(text_stream.as_str(), forth_obj_id);

    let (xref_start_offset, xref_id) = self.set_xref_table();
    self.set_trailer(xref_start_offset, xref_id);
    self.set_end();
    &self.contents
  }
}
