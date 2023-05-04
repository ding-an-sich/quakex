defmodule Quakex.TectonicNIF do
  use Rustler, otp_app: :quakex, crate: "tectonic_nif"

  def latex_2_pdf(_str), do: :erlang.nif_error(:nif_not_loaded)
end
