defmodule Quakex.MixProject do
  use Mix.Project

  def project do
    [
      app: :quakex,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.28.0"}
    ]
  end
end
