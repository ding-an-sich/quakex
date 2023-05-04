defmodule Quakex do
  @moduledoc """
  Documentation for `Quakex`.
  """

  alias Quakex.TectonicNIF, as: T

  @type pdf :: [<<_::8>>]

  @spec latex_2_pdf(latex :: binary()) :: {:ok, pdf()} | {:error, :latex_engine_failed}
  def latex_2_pdf(latex) do
    case T.latex_2_pdf(latex) do
      {:error, _msg} -> {:error, :latex_engine_failed}
      result -> result
    end
  end
end
