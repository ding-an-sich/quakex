use tectonic;

#[rustler::nif]
fn latex_2_pdf(latex_str: String) -> Result<Vec<u8>, &'static str> {
    match tectonic::latex_to_pdf(latex_str) {
        Ok(pdf) => Ok(pdf),
        Err(_) => Err("Latex engine failed"),
    }
}

rustler::init!("Elixir.Quakex.TectonicNIF", [latex_2_pdf]);
