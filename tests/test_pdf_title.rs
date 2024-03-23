// mod tests {
//     use std::fs::File;
//     use std::io::{BufWriter, Result};
//
//     use lopdf::{Document, Object};
//
//     #[test]
//     fn set_pdf_title<P: AsRef<std::path::Path>>(pdf_path: P, new_title: &str) -> Result<()> {
//         let mut doc = Document::load(pdf_path.as_ref())?;
//         let mut info = doc
//             .trailer
//             .get_mut(b"Info")
//             .ok_or("Info dictionary not available")?
//             .as_dict_mut()?;
//
//         // Set the title
//         info.set("Title", Object::string_literal(new_title));
//
//         // Save the modified PDF
//         let output_file = File::create(pdf_path)?;
//         let buffer = BufWriter::new(output_file);
//         doc.save_to(buffer)?;
//         Ok(())
//     }
//
//     fn main() -> Result<()> {
//         let pdf_path = "/Users/richardlyon/Desktop/Goehring and Rozencwajg - 2023 - Goehring & Rozencwajg Natural Resource Market Comm.pdf"; // Specify the path to your PDF
//         let new_title = "New Title"; // Specify the new title
//
//         set_pdf_title(pdf_path, new_title)?;
//         println!("Title has been updated.");
//         Ok(())
//     }
// }
