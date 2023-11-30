defmodule Gluesql.Native do
  use Rustler, otp_app: :gluesql, crate: "gluesql_native"

  # When your NIF is loaded, it will override this function.
  def new_memory_db(), do: :erlang.nif_error(:nif_not_loaded)
  def execute_memory_db(_db, _stmt, _send_to), do: :erlang.nif_error(:nif_not_loaded)
end
