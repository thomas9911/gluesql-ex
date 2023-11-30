defmodule Gluesql.MixProject do
  use Mix.Project

  def project do
    [
      app: :gluesql,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.30.0"}
    ]
  end

  defp aliases do
    [
      format: ["format", "cmd --cd ./native/gluesql_native cargo fmt"]
    ]
  end
end
