defmodule Gluesql do
  @moduledoc """
  Documentation for `Gluesql`.
  """

  defdelegate new_memory_db(), to: Gluesql.Native

  def execute_memory_db(db, stmt) do
    case Gluesql.Native.execute_memory_db(db, stmt, self()) do
      {:ok, _} ->
        receive do
          x ->
            x

          after
            5000 -> {:error, :timeout}
        end


      other -> other
    end
  end
end
