use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::Path;

use tectonic;

#[rustler::nif]
fn latex_2_pdf(latex_str: String) -> Result<Vec<u8>, &'static str> {
    latex_2_pdf_priv(latex_str)
}

#[rustler::nif]
fn latex_2_pdf_file(latex_str: String, path_str: String) -> Result<(), &'static str> {
    match latex_2_pdf_file_priv(latex_str, path_str) {
        Ok(()) => Ok(()),
        Err(_) => Err("Error creating PDF file"),
    }
}

fn latex_2_pdf_priv(latex_str: String) -> Result<Vec<u8>, &'static str> {
    match tectonic::latex_to_pdf(latex_str) {
        Ok(pdf) => Ok(pdf),
        Err(_) => Err("Latex engine failed"),
    }
}

fn latex_2_pdf_file_priv(latex_str: String, path_str: String) -> std::io::Result<()> {
    let path = Path::new(&path_str);

    let mut file = File::create(path)?;

    let latex_error_as_io_error = Err(Error::new(ErrorKind::Other, "Latex engine failed"));

    let pdf_data = match latex_2_pdf_priv(latex_str) {
        Ok(data) => data,
        Err(_) => return latex_error_as_io_error,
    };

    file.write_all(&pdf_data)?;

    Ok(())
}

rustler::init!("Elixir.Quakex.TectonicNIF", [latex_2_pdf, latex_2_pdf_file]);
