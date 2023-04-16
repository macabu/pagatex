defmodule Pagatex.MixProject do
  use Mix.Project

  def project do
    [
      app: :pagatex,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    []
  end

  defp deps do
    [
      {:rustler, "~> 0.27.0"}
    ]
  end
end
