defmodule Gluesql do
  @moduledoc """
  Documentation for `Gluesql`.
  """

  @default_timeout 5000

  defdelegate new_memory_db(), to: Gluesql.Native

  def execute_memory_db(db, stmt, timeout \\ @default_timeout) do
    case Gluesql.Native.execute_memory_db(db, stmt, self()) do
      {:ok, _} ->
        receive do
          x ->
            x
        after
          timeout -> {:error, :timeout}
        end

      other ->
        other
    end
  end
end
