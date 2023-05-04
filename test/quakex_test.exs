defmodule QuakexTest do
  use ExUnit.Case

  describe "latex_2_pdf_file/2" do
    test "writes a pdf file to path" do
      path = :quakex |> :code.priv_dir() |> Path.join(["hello_quakex.pdf"])
      latex = ~S|\documentclass{article}
      \begin{document}
      Hello from Quakex!
      \end{document}|

      refute File.exists?(path)
      assert :ok == Quakex.latex_2_pdf_file(latex, path)
      assert File.exists?(path)
      File.rm!(path)
    end
  end
end
