defmodule GluesqlTest do
  use ExUnit.Case
  doctest Gluesql

  test "greets the world" do
    db = Gluesql.new_memory_db()
    Gluesql.execute_memory_db(db, "create table users (id INTEGER PRIMARY KEY)") |> IO.inspect()
    Gluesql.execute_memory_db(db, "insert into users (id) VALUES (1)") |> IO.inspect()
    Gluesql.execute_memory_db(db, "select * from users;") |> IO.inspect()
  end
end
